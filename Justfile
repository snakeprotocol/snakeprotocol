# Justfile

# Default release command
release:
    @echo "Building release version..."
    cargo build --release
    @just copy-binary

# Copy binary command
copy-binary:
    @if [ -f ./target/release/snaked ]; then \
        echo "Copying snaked binary to ui/desktop/src/bin with permissions preserved..."; \
        cp -p ./target/release/snaked ./ui/desktop/src/bin/; \
    else \
        echo "Release binary not found."; \
        exit 1; \
    fi

# Run UI with latest
run-ui:
    @just release
    @echo "Running UI..."
    cd ui/desktop && npm install && npm run start-gui

# Run Docusaurus server for documentation
run-docs:
    @echo "Running docs server..."
    cd documentation && yarn && yarn start

# Run server
run-server:
    @echo "Running server..."
    cargo run -p snake-server

# make GUI with latest binary
make-ui:
    @just release
    cd ui/desktop && npm run bundle:default

# Setup langfuse server
langfuse-server:
    #!/usr/bin/env bash
    ./scripts/setup_langfuse.sh
