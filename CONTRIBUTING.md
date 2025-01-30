# Contributing

Snake is Open Source!

We welcome Pull Requests for general contributions! If you have a larger new feature or any questions on how to develop a fix, we recommend you open an [issue][issues] before starting.

## Prerequisites

snake includes rust binaries alongside an electron app for the GUI. To work
on the rust backend, you will need to [install rust and cargo][rustup]. To work
on the App, you will also need to [install node and npm][nvm] - we recommend through nvm.

We provide a shortcut to standard commands using [just][just] in our `justfile`.

## Getting Started

### Rust

First let's compile snake and try it out

```
cargo build
```

when that is done, you should now have debug builds of the binaries like the snake cli:

```
./target/debug/snake --help
```

If you haven't used the CLI before, you can use this compiled version to do first time configuration:

```
./target/debug/snake configure
```

And then once you have a connection to an LLM provider working, you can run a session!

```
./target/debug/snake session
```

These same commands can be recompiled and immediately run using `cargo run -p snake-cli` for iteration.
As you make changes to the rust code, you can try it out on the CLI, or also run checks and tests:

```
cargo check  # do your changes compile
cargo test  # do the tests pass with your changes.
```

### Node

Now let's make sure you can run the app.

```
just run-ui
```

The start gui will both build a release build of rust (as if you had done `cargo build -r`) and start the electron process.
You should see the app open a window, and drop you into first time setup. When you've gone through the setup,
you can talk to snake!

You can now make changes in the code in ui/desktop to iterate on the GUI half of snake.

## Env Vars

You may want to make more frequent changes to your provider setup or similar to test things out
as a developer. You can use environment variables to change things on the fly without redoing
your configuration.

> [!TIP]
> At the moment, we are still updating some of the CLI configuration to make sure this is
> respected.

You can change the provider snake points to via the `snake_PROVIDER` env var. If you already
have a credential for that provider in your keychain from previously setting up, it should
reuse it. For things like automations or to test without doing official setup, you can also
set the relevant env vars for that provider. For example `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`,
or `DATABRICKS_HOST`. Refer to the provider details for more info on required keys.

## Enable traces in snake with [locally hosted Langfuse](https://langfuse.com/docs/deployment/self-host)

- Run `just langfuse-server` to start your local Langfuse server. It requires Docker.
- Go to http://localhost:3000 and log in with the default email/password output by the shell script (values can also be found in the `.env.langfuse.local` file).
- Set the environment variables so that rust can connect to the langfuse server

```
export LANGFUSE_INIT_PROJECT_PUBLIC_KEY=publickey-local
export LANGFUSE_INIT_PROJECT_SECRET_KEY=secretkey-local
```

Then you can view your traces at http://localhost:3000

## Conventional Commits

This project follows the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification for PR titles. Conventional Commits make it easier to understand the history of a project and facilitate automation around versioning and changelog generation.

[issues]: https://github.com/block/snake/issues
[rustup]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[nvm]: https://github.com/nvm-sh/nvm
[just]: https://github.com/casey/just?tab=readme-ov-file#installation
