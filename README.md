# jev-cli

A command-line interface tool in Rust to query the TypeSafe Jev model locally or via OpenRouter.

## Table of Contents
- [Features](#features)
- [Getting Started: Linux / WSL](#getting-started-linux--wsl)
- [Getting Started: macOS](#getting-started-macos)
- [Getting Started: Windows](#getting-started-windows)
- [Usage Examples](#usage-examples)
  - [Model Overrides](#passing-the-specific-model-optional)
  - [1. Noul (Yes/No Judgement)](#1-noul-yesno-judgement)
  - [2. Choice (Multiple Choice)](#2-choice-multiple-choice)
  - [3. Score (Grading Rubric)](#3-score-grading-rubric)
- [Building from Source](#building-from-source)

---

## Features
- Evaluates states using **Noul** (Yes/No), **Choice** (Multiple Choice), or **Score** (Rubric) question kinds.
- Validates tokens and allows querying both `typesafe.ai` and `openrouter` seamlessly.
- Allows explicitly overriding the model using the `--model` flag.
- Provides well-formatted JSON output natively from the APIs.

---

## Getting Started: Linux / WSL

### 1. Installation
Run this single command to download and install the CLI to `~/.local/bin/jev`:
```bash
curl -sSL https://raw.githubusercontent.com/Salv4d/jev-cli/master/install.sh | bash
```
*(Ensure `~/.local/bin` is in your `PATH`)*

### 2. Environment Variables
You can pass your token using `--token` or set it globally:
```bash
export JEV_TOKEN="your_token_here"
```
If using OpenRouter, you must also specify the provider (default is `typesafe.ai`):
```bash
export JEV_PROVIDER="openrouter"
```

---

## Getting Started: macOS

### 1. Installation
Download and install the pre-compiled binary:
```bash
wget https://github.com/Salv4d/jev-cli/releases/latest/download/jev-cli-x86_64-apple-darwin.tar.gz
tar -xzf jev-cli-x86_64-apple-darwin.tar.gz
chmod +x jev-cli
sudo mv jev-cli /usr/local/bin/jev
```

### 2. Environment Variables
You can pass your token using `--token` or set it globally:
```bash
export JEV_TOKEN="your_token_here"
```
If using OpenRouter, you must also specify the provider (default is `typesafe.ai`):
```bash
export JEV_PROVIDER="openrouter"
```

---

## Getting Started: Windows

### 1. Installation
1. Download `jev-cli-x86_64-pc-windows-msvc.zip` from the [Releases page](https://github.com/Salv4d/jev-cli/releases).
2. Extract the ZIP file.
3. Move `jev-cli.exe` to a permanent folder and rename it to `jev.exe`.
4. Add that folder to your System `PATH` environment variable.

### 2. Environment Variables
You can pass your token using `--token` or set it globally.

**Using PowerShell:**
```powershell
$env:JEV_TOKEN="your_token_here"
$env:JEV_PROVIDER="openrouter" # (Optional, default is typesafe.ai)
```

**Using Command Prompt (CMD):**
```cmd
set JEV_TOKEN=your_token_here
set JEV_PROVIDER=openrouter
```

---

## Usage Examples

### Passing the specific Model (Optional)
By default, the CLI uses the latest Jev model. To specify an exact model on OpenRouter (e.g., if you don't want to use the latest), you can use the `--model` argument or `JEV_MODEL` environment variable:
```bash
jev --model "typesafe/jev-1.13-20260917" ...
```

### 1. Noul (Yes/No Judgement)
Evaluates a boolean/probabilistic yes/no based on the instructions.
```bash
jev --state "Water is made of Hydrogen and Oxygen." \
    --kind noul \
    --instructions "Is this scientifically accurate?"
```

### 2. Choice (Multiple Choice)
Selects the best choice based on criteria. Pass `--criteria` as a JSON dictionary.
```bash
jev --state "This animal has a long trunk and big ears." \
    --kind choice \
    --instructions "What animal is this?" \
    --criteria '{"A": "Elephant", "B": "Giraffe", "C": "Lion"}'
```

### 3. Score (Grading Rubric)
Scores the state based on a grading array. Pass `--criteria` as a JSON array.
```bash
jev --state "The service was incredible, 10/10 would return!" \
    --kind score \
    --instructions "Rate the sentiment of this feedback" \
    --criteria '["Terrible", "Poor", "Average", "Good", "Excellent"]'
```

To see all available commands, run:
```bash
jev --help
```

---

## Building from Source

Ensure you have Rust installed, then run:
```bash
cargo build --release
```
