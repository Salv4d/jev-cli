# jev-cli

A command-line interface tool in Rust to interact with AI providers.

## Features

- Environment validation for token and provider.
- Supports `typesafe.ai` (default) and `openrouter`.
- Cross-platform binaries for Linux, macOS, and Windows.

## Installation

You can download the pre-compiled binaries from the [Releases page](https://github.com/Salv4d/jev-cli/releases/latest).

### Linux / macOS
```bash
# Example for Linux
wget https://github.com/Salv4d/jev-cli/releases/latest/download/jev-cli-x86_64-unknown-linux-gnu.tar.gz
tar -xzf jev-cli-x86_64-unknown-linux-gnu.tar.gz
chmod +x jev-cli
sudo mv jev-cli /usr/local/bin/
```

### Windows
Download `jev-cli-x86_64-pc-windows-msvc.zip` from the Releases page, extract it, and add the executable to your `PATH`.

## Usage

Set the required environment variables:
```bash
export JEV_TOKEN="your_token_here"
export JEV_PROVIDER="typesafe.ai" # Or "openrouter"
```

Or pass them as arguments:
```bash
jev-cli --token "your_token_here" --provider openrouter
```

Run the application:
```bash
jev-cli
```

## Building from Source

Ensure you have Rust installed, then run:
```bash
cargo build --release
```
