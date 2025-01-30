use super::errors::ProviderError;
use crate::message::Message;
use crate::model::ModelConfig;
use crate::providers::base::{ConfigKey, Provider, ProviderMetadata, ProviderUsage};
use crate::providers::formats::google::{create_request, get_usage, response_to_message};
use crate::providers::utils::{emit_debug_trace, unescape_json_values};
use anyhow::Result;
use async_trait::async_trait;
use mcp_core::tool::Tool;
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::time::Duration;

pub const GOOGLE_API_HOST: &str = "https://generativelanguage.googleapis.com";
pub const GOOGLE_DEFAULT_MODEL: &str = "gemini-2.0-flash-exp";
pub const GOOGLE_KNOWN_MODELS: &[&str] = &[
    "models/gemini-1.5-pro-latest",
    "models/gemini-1.5-pro",
    "models/gemini-1.5-flash-latest",
    "models/gemini-1.5-flash",
    "models/gemini-2.0-flash-exp",
    "models/gemini-2.0-flash-thinking-exp-01-21",
];

pub const GOOGLE_DOC_URL: &str = "https://ai.google/get-started/our-models/";

#[derive(Debug, serde::Serialize)]
pub struct GoogleProvider {
    #[serde(skip)]
    client: Client,
    host: String,
    api_key: String,
    model: ModelConfig,
}

impl Default for GoogleProvider {
    fn default() -> Self {
        let model = ModelConfig::new(GoogleProvider::metadata().default_model);
        GoogleProvider::from_env(model).expect("Failed to initialize Google provider")
    }
}

impl GoogleProvider {
    pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("GOOGLE_API_KEY")?;
        let host: String = config
            .get("GOOGLE_HOST")
            .unwrap_or_else(|_| GOOGLE_API_HOST.to_string());

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
        let url = format!(
            "{}/v1beta/models/{}:generateContent?key={}",
            self.host.trim_end_matches('/'),
            self.model.model_name,
            self.api_key
        );

        let response = self
            .client
            .post(&url)
            .header("CONTENT_TYPE", "application/json")
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        let payload: Option<Value> = response.json().await.ok();

        match status {
            StatusCode::OK =>  payload.ok_or_else( || ProviderError::RequestFailed("Response body is not valid JSON".to_string()) ),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                Err(ProviderError::Authentication(format!("Authentication failed. Please ensure your API keys are valid and have the required permissions. \
                    Status: {}. Response: {:?}", status, payload )))
            }
            StatusCode::BAD_REQUEST => {
                let mut error_msg = "Unknown error".to_string();
                if let Some(payload) = &payload {
                    if let Some(error) = payload.get("error") {
                        error_msg = error.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error").to_string();
                        let error_status = error.get("status").and_then(|s| s.as_str()).unwrap_or("Unknown status");
                        if error_status == "INVALID_ARGUMENT" && error_msg.to_lowercase().contains("exceeds") {
                            return Err(ProviderError::ContextLengthExceeded(error_msg.to_string()));
                        }
                    }
                }
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
impl Provider for GoogleProvider {
    fn metadata() -> ProviderMetadata {
        ProviderMetadata::new(
            "google",
            "Google Gemini",
            "Gemini models from Google AI",
            GOOGLE_DEFAULT_MODEL,
            GOOGLE_KNOWN_MODELS.iter().map(|&s| s.to_string()).collect(),
            GOOGLE_DOC_URL,
            vec![
                ConfigKey::new("GOOGLE_API_KEY", true, true, None),
                ConfigKey::new("GOOGLE_HOST", false, false, Some(GOOGLE_API_HOST)),
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
        let message = response_to_message(unescape_json_values(&response))?;
        let usage = get_usage(&response)?;
        let model = match response.get("modelVersion") {
            Some(model_version) => model_version.as_str().unwrap_or_default().to_string(),
            None => self.model.model_name.clone(),
        };
        emit_debug_trace(self, &payload, &response, &usage);
        let provider_usage = ProviderUsage::new(model, usage);
        Ok((message, provider_usage))
    }
}
