---
title: Troubleshooting
---

# Troubleshooting

Goose, like any system, may run into occasional issues. This guide provides solutions for common problems.

### Goose Edits Files
Goose can and will edit files as part of its workflow. To avoid losing personal changes, use version control to stage your personal edits. Leave Goose edits unstaged until reviewed. Consider separate commits for Goose's edits so you can easily revert them if needed.

---

### Interrupting Goose
If Goose is heading in the wrong direction or gets stuck, you can interrupt it by pressing `CTRL+C`. This will stop Goose and give you the opportunity to correct its actions or provide additional information.

---

### Stuck in a Loop or Unresponsive
In rare cases, Goose may enter a "doom spiral" or become unresponsive during a long session. This is often resolved by ending the current session, and starting a new session.

1. Hold down `Ctrl + C` to cancel
2. Start a new session:
  ```sh
  goose session
  ```
:::tip
For particularly large or complex tasks, consider breaking them into smaller sessions.
:::

---

### Context Length Exceeded Error

This error occurs when the input provided to Goose exceeds the maximum token limit of the LLM being used. To resolve this try breaking down your input into smaller parts. You can also use `.goosehints` as a way to provide goose with detailed context. Refer to the [Using Goosehints Guide][goosehints] for more information.

---

### Using Ollama Provider

Ollama provides local LLMs, which means you must first [download Ollama and run a model](/docs/getting-started/providers#local-llms-ollama) before attempting to use this provider with Goose. If you do not have the model downloaded, you'll run into the follow error:

> ExecutionError("error sending request for url (http://localhost:11434/v1/chat/completions)")


Another thing to note is that the DeepSeek models do not support tool calling, so all Goose [extensions must be disabled](/docs/getting-started/using-extensions#enablingdisabling-extensions) to use one of these models. Unfortunately, without the use of tools, there is not much Goose will be able to do autonomously if using DeepSeek. However, Ollama's other models such as `qwen2.5` do support tool calling and can be used with Goose extensions.

---

### Handling Rate Limit Errors
Goose may encounter a `429 error` (rate limit exceeded) when interacting with LLM providers. The recommended solution is to use OpenRouter. See [Handling LLM Rate Limits][handling-rate-limits] for more info.

---

### Hermit Errors

If you see an issue installing an extension in the app that says "hermit:fatal", you may need to reset your hermit cache. We use
a copy of hermit to ensure npx and uvx are consistently available. If you have already used an older version of hermit, you may
need to cleanup the cache - on Mac this cache is at

```
sudo rm -rf ~/Library/Caches/hermit
```

---

### API Errors

Users may run into an error like the one below when there are issues with their LLM API tokens, such as running out of credits or incorrect configuration:

```sh
Traceback (most recent call last):
  File "/Users/admin/.local/pipx/venvs/goose-ai/lib/python3.13/site-packages/exchange/providers/utils.py",
line 30, in raise_for_status
    response.raise_for_status()
    ~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/admin/.local/pipx/venvs/goose-ai/lib/python3.13/site-packages/httpx/_models.py",
line 829, in raise_for_status
    raise HTTPStatusError(message, request=request, response=self)
httpx.HTTPStatusError: Client error '404 Not Found' for url
'https://api.openai.com/v1/chat/completions'

...
```
This error typically occurs when LLM API credits are exhausted or your API key is invalid. To resolve this issue:

1. Check Your API Credits:
    - Log into your LLM provider's dashboard
    - Verify that you have enough credits. If not, refill them
2. Verify API Key:
    - Run the following command to reconfigure your API key:
    ```sh
    goose configure
    ```
For detailed steps on updating your LLM provider, refer to the [Installation][installation] Guide.

---

### Remove Cached Data

Goose stores data in a few places. Secrets, such as API keys, are stored exclusively in the system keychain.
Logs and configuration data are stored in `~/.config/goose`. And the app stores a small amount of data in
`~/Library/Application Support/Goose`.

You can remove all of this data by following these steps.

* stop any copies of goose running (CLI or GUI)
  * consider confirming you've stopped them all via the activity monitor
* open the keychain and delete the credential called "goose", which contains all secrets stored by goose
* `rm -rf ~/.config/goose`
* For the App on macos, `rm -rf ~/Library/Application Support/Goose`
* Delete the "Goose" app from your Applications folder

After this cleanup, if you are looking to try out a fresh install of Goose, you can now start from the usual
install instructions.

---

### Keychain/Keyring Errors

Goose tries to use the system keyring to store secrets. In environments where there is no keyring support, you may
see an error like:

```bash
Error Failed to access secure storage (keyring): Platform secure storage failure: DBus error: The name org.freedesktop.secrets was not provided by any .service files
Please check your system keychain and run 'goose configure' again.
If your system is unable to use the keyring, please try setting secret key(s) via environment variables.
```

In this case, you will need to set your provider specific environment variable(s), which can be found at [Supported LLM Providers][configure-llm-provider].

You can set them either by doing:
* `export GOOGLE_API_KEY=$YOUR_KEY_HERE` - for the duration of your session
* in your `~/.bashrc` or `~/.zshrc` - (or equivalents) so it persists on new shell each new session

Then select the `No` option when prompted to save the value to your keyring.

```bash
$ goose configure

Welcome to goose! Let's get you set up with a provider.
  you can rerun this command later to update your configuration

┌   goose-configure
│
◇  Which model provider should we use?
│  Google Gemini
│
◇  GOOGLE_API_KEY is set via environment variable
│
◇  Would you like to save this value to your keyring?
│  No
│
◇  Enter a model from that provider:
│  gemini-2.0-flash-exp
```

---

### Need Further Help? 
If you have questions, run into issues, or just need to brainstorm ideas join the [Discord Community][discord]!



[handling-rate-limits]: /docs/guides/handling-llm-rate-limits-with-goose
[installation]: /docs/getting-started/installation
[discord]: https://discord.gg/block-opensource
[goosehints]: /docs/guides/using-goosehints
[configure-llm-provider]: /docs/getting-started/providers
