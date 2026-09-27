# jev-cli

A command-line interface tool in Rust to interact with AI providers.

## Features

- Environment validation for token and provider.
- Supports `typesafe.ai` (default) and `openrouter`.
- Pass state payloads and specify question kinds (`noul`, `choice`, `score`).
- Cross-platform binaries for Linux, macOS, and Windows.

## Installation

### Linux / WSL (Easy Install)
You can easily install `jev` and add it to your path with our installation script:

```bash
curl -sSL https://raw.githubusercontent.com/Salv4d/jev-cli/master/install.sh | bash
```
*Note: The script installs the binary to `~/.local/bin/jev`. Make sure `~/.local/bin` is in your PATH.*

### Manual Installation (All Platforms)
You can download the pre-compiled binaries from the [Releases page](https://github.com/Salv4d/jev-cli/releases).

#### macOS
```bash
wget https://github.com/Salv4d/jev-cli/releases/latest/download/jev-cli-x86_64-apple-darwin.tar.gz
tar -xzf jev-cli-x86_64-apple-darwin.tar.gz
chmod +x jev-cli
sudo mv jev-cli /usr/local/bin/jev
```

#### Windows
Download `jev-cli-x86_64-pc-windows-msvc.zip` from the Releases page, extract it, and add the executable (rename it to `jev.exe` if desired) to your `PATH`.

## Usage

Set the required environment variables:
```bash
export JEV_TOKEN="your_token_here"
export JEV_PROVIDER="typesafe.ai" # Or "openrouter"
```

Run the application specifying the state and the kind of question (`noul`, `choice`, `score`):
```bash
jev --state '{"key": "value"}' --kind choice
```

Or pass all arguments explicitly:
```bash
jev --token "your_token" --provider openrouter --state ./my_state.json --kind score
```

To see all available commands:
```bash
jev --help
```

## Building from Source

Ensure you have Rust installed, then run:
```bash
cargo build --release
```
