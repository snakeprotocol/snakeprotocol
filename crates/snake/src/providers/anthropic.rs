use anyhow::Result;
use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::time::Duration;

use super::base::{ConfigKey, Provider, ProviderMetadata, ProviderUsage};
use super::errors::ProviderError;
use super::formats::anthropic::{create_request, get_usage, response_to_message};
use super::utils::{emit_debug_trace, get_model};
use crate::message::Message;
use crate::model::ModelConfig;
use mcp_core::tool::Tool;

pub const ANTHROPIC_DEFAULT_MODEL: &str = "claude-3-5-sonnet-latest";
pub const ANTHROPIC_KNOWN_MODELS: &[&str] = &[
    "claude-3-5-sonnet-latest",
    "claude-3-5-haiku-latest",
    "claude-3-opus-latest",
];

pub const ANTHROPIC_DOC_URL: &str = "https://docs.anthropic.com/en/docs/about-claude/models";

#[derive(serde::Serialize)]
pub struct AnthropicProvider {
    #[serde(skip)]
    client: Client,
    host: String,
    api_key: String,
    model: ModelConfig,
}

impl Default for AnthropicProvider {
    fn default() -> Self {
        let model = ModelConfig::new(AnthropicProvider::metadata().default_model);
        AnthropicProvider::from_env(model).expect("Failed to initialize Anthropic provider")
    }
}

impl AnthropicProvider {
    pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("ANTHROPIC_API_KEY")?;
        let host: String = config
            .get("ANTHROPIC_HOST")
            .unwrap_or_else(|_| "https://api.anthropic.com".to_string());

        let client = Client::builder()
            .timeout(Duration::from_secs(600))
            .build()?;

        Ok(Self {
            client,
            host,
            api_key,
            model,
        })
    }

    async fn post(&self, payload: Value) -> Result<Value, ProviderError> {
        let url = format!("{}/v1/messages", self.host.trim_end_matches('/'));

        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        let payload: Option<Value> = response.json().await.ok();

        // https://docs.anthropic.com/en/api/errors
        match status {
            StatusCode::OK => payload.ok_or_else( || ProviderError::RequestFailed("Response body is not valid JSON".to_string()) ),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                Err(ProviderError::Authentication(format!("Authentication failed. Please ensure your API keys are valid and have the required permissions. \
                    Status: {}. Response: {:?}", status, payload)))
            }
            StatusCode::BAD_REQUEST => {
                let mut error_msg = "Unknown error".to_string();
                if let Some(payload) = &payload {
                    if let Some(error) = payload.get("error") {
                    tracing::debug!("Bad Request Error: {error:?}");
                    error_msg = error.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error").to_string();
                    if error_msg.to_lowercase().contains("too long") || error_msg.to_lowercase().contains("too many") {
                        return Err(ProviderError::ContextLengthExceeded(error_msg.to_string()));
                    }
                }}
                tracing::debug!(
                    "{}", format!("Provider request failed with status: {}. Payload: {:?}", status, payload)
                );
                Err(ProviderError::RequestFailed(format!("Request failed with status: {}. Message: {}", status, error_msg)))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                Err(ProviderError::RateLimitExceeded(format!("{:?}", payload)))
            }
            StatusCode::INTERNAL_SERVER_ERROR | StatusCode::SERVICE_UNAVAILABLE => {
                Err(ProviderError::ServerError(format!("{:?}", payload)))
            }
            _ => {
                tracing::debug!(
                    "{}", format!("Provider request failed with status: {}. Payload: {:?}", status, payload)
                );
                Err(ProviderError::RequestFailed(format!("Request failed with status: {}", status)))
            }
        }
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    fn metadata() -> ProviderMetadata {
        ProviderMetadata::new(
            "anthropic",
            "Anthropic",
            "Claude and other models from Anthropic",
            ANTHROPIC_DEFAULT_MODEL,
            ANTHROPIC_KNOWN_MODELS
                .iter()
                .map(|&s| s.to_string())
                .collect(),
            ANTHROPIC_DOC_URL,
            vec![
                ConfigKey::new("ANTHROPIC_API_KEY", true, true, None),
                ConfigKey::new(
                    "ANTHROPIC_HOST",
                    false,
                    false,
                    Some("https://api.anthropic.com"),
                ),
            ],
        )
    }

    fn get_model_config(&self) -> ModelConfig {
        self.model.clone()
    }

    #[tracing::instrument(
        skip(self, system, messages, tools),
        fields(model_config, input, output, input_tokens, output_tokens, total_tokens)
    )]
    async fn complete(
        &self,
        system: &str,
        messages: &[Message],
        tools: &[Tool],
    ) -> Result<(Message, ProviderUsage), ProviderError> {
        let payload = create_request(&self.model, system, messages, tools)?;

        // Make request
        let response = self.post(payload.clone()).await?;

        // Parse response
        let message = response_to_message(response.clone())?;
        let usage = get_usage(&response)?;

        let model = get_model(&response);
        emit_debug_trace(self, &payload, &response, &usage);
        Ok((message, ProviderUsage::new(model, usage)))
    }
}
