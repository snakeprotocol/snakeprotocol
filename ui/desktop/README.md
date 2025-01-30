# Snake App

Mac (windows soon) app for snake. 

```
git clone git@github.com:snakeprotocol/snakeprotocol.git
cd snake/ui/desktop
npm install
npm start
```

# Building notes

This is an electron forge app, using vite and react.js. `snaked` runs as multi process binaries on each window/tab similar to chrome.

see `package.json`: 

`npm run bundle:default` will give you a snake.app/zip which is signed/notarized but only if you setup the env vars as per `forge.config.ts` (you can empty out the section on osxSign if you don't want to sign it) - this will have all defaults.

`npm run bundle:preconfigured` will make a snake.app/zip signed and notarized, but use the following:

```python
            f"        process.env.snake_PROVIDER__TYPE = '{os.getenv("snake_BUNDLE_TYPE")}';",
            f"        process.env.snake_PROVIDER__HOST = '{os.getenv("snake_BUNDLE_HOST")}';",
            f"        process.env.snake_PROVIDER__MODEL = '{os.getenv("snake_BUNDLE_MODEL")}';"
```

This allows you to set for example snake_PROVIDER__TYPE to be "databricks" by default if you want (so when people start snake.app - they will get that out of the box). There is no way to set an api key in that bundling as that would be a terrible idea, so only use providers that can do oauth (like databricks can), otherwise stick to default snake.


# Runninng with snaked server from source

Set `VITE_START_EMBEDDED_SERVER=yes` to no in `.env.
Run `cargo run -p snake-server` from parent dir.
`npm run start` will then run against this.
You can try server directly with `./test.sh`
