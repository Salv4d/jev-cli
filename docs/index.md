---
layout: default
title: "jev-cli Documentation"
description: "Comprehensive guide to using the jev-cli tool for TypeSafe's Jev model."
---

[🇺🇸 English](/jev-cli/) | [🇧🇷 Português](/jev-cli/pt-br/) | [🇪🇸 Español](/jev-cli/es/) | [🇨🇳 中文](/jev-cli/zh/)

# jev-cli Documentation

Welcome to the official documentation for **jev-cli**. This command-line interface allows you to instantly query the TypeSafe Jev model locally or via OpenRouter, utilizing a highly intuitive positional shorthand syntax.

Jev evaluates "states" (which can be a string of text, a block of JSON, or structured data) against targeted questions or instructions, returning highly constrained and probabilistic JSON decisions rather than unstructured text.

---

## 1. Authentication & Configuration

Before using the CLI, you must configure your API token and (optionally) your provider. By default, the CLI queries `typesafe.ai`, but you can route queries through OpenRouter.

### Linux / macOS
```bash
# 1. Set your API Token
export JEV_TOKEN="your_token_here"

# 2. (Optional) Set your provider to openrouter
export JEV_PROVIDER="openrouter"
```

### Windows (PowerShell)
```powershell
# 1. Set your API Token
$env:JEV_TOKEN="your_token_here"

# 2. (Optional) Set your provider to openrouter
$env:JEV_PROVIDER="openrouter"
```

### Windows (Command Prompt)
```cmd
:: 1. Set your API Token
set JEV_TOKEN=your_token_here

:: 2. (Optional) Set your provider to openrouter
set JEV_PROVIDER=openrouter
```

*(Alternatively, you can pass these dynamically per command: `jev --token "..." --provider openrouter "State" "Question?"`)*

### Model Overrides
By default, the CLI uses the `jev-latest` model for TypeSafe, and `~typesafe/jev-latest` for OpenRouter. If you need to pin to a specific version (e.g. `typesafe/jev-1.13-20260917`), you can use the `--model` flag:
```bash
jev --model "typesafe/jev-1.13-20260917" "I like apples." "Does this mention fruit?"
```

---

## 2. Usage & Question Kinds

The `jev-cli` intelligently figures out what type of question you are asking based on the arguments you pass.

### Noul (Yes/No Judgements)
A **Noul** evaluation acts as a boolean or yes/no judge. It returns a probability score between `0` and `1` indicating how true the condition is.

**Syntax:** `jev "<state>" "<question>"`

**Example:**
```bash
jev "The sky is blue and the grass is green." "Does this text describe colors?"
```

**Expected Output:**
```json
{
  "answers": {
    "q1": {
      "type": "noul",
      "noul": 0.98
    }
  }
}
```

---

### Choice (Multiple Choice)
A **Choice** evaluation selects the most accurate option from a set of choices.

**Syntax:** `jev "<state>" "<question>" <option_1> <option_2> <option_3> ...`

**Example:**
```bash
jev "My favorite animal is the one with a trunk." "What is the animal?" Elephant Giraffe Lion
```

**Expected Output:**
```json
{
  "answers": {
    "q1": {
      "type": "choice",
      "choice": "A",
      "probabilities": {
        "A": 0.99,
        "B": 0.0,
        "C": 0.0
      },
      "confidence": 0.99
    }
  }
}
```

---

### Score (Grading Rubric)
A **Score** evaluation maps the state against an ordered legend/scale, identifying the index of the scale that fits best.

**Syntax:** `jev "<state>" "<question>" --scale <scale_1> <scale_2> <scale_3> ...`

**Example:**
```bash
jev "I absolutely loved this restaurant, the food was divine!" "Rate the sentiment" --scale "Terrible" "Poor" "Neutral" "Good" "Excellent"
```

**Expected Output:**
```json
{
  "answers": {
    "q1": {
      "type": "score",
      "score": 4,
      "legend": {
        "0": "Terrible",
        "1": "Poor",
        "2": "Neutral",
        "3": "Good",
        "4": "Excellent"
      },
      "probabilities": {
        "0": 0.0,
        "1": 0.0,
        "2": 0.0,
        "3": 0.01,
        "4": 0.99
      },
      "confidence": 0.99
    }
  }
}
```

---

## 3. Advanced Examples

### Evaluating JSON Data
Your state does not have to be a raw sentence. It can be a serialized JSON payload for structured evaluation!

```bash
jev '{"user_age": 17, "has_license": false}' "Is this user legally allowed to drive?"
```

### Complex Choice Mapping
If your choices are long sentences, simply wrap them in quotes:
```bash
jev "Error 503: Service Unavailable" "What should the system do?" "Retry immediately" "Back off exponentially and retry" "Fail silently"
```

### Explicit Syntax (For Scripts and CI/CD)
While the positional shorthand syntax is great for humans typing in the terminal, you might prefer explicit named arguments in scripts to ensure stability and readability. 

The CLI fully supports long-form arguments:
```bash
jev --state "I want a car." --instructions "Does this text express desire?" --type noul
jev --state "Text" --instructions "Question" --type choice --criteria '{"A":"Yes", "B":"No"}'
jev --state "Text" --instructions "Question" --type score --criteria '["Bad", "Good"]'
```

### Multiple Questions in a Single Request
The CLI also supports sending multiple questions at once to evaluate the same state against different criteria simultaneously. You can do this by using the `--questions` flag, which accepts a raw JSON string OR a path to a JSON/YAML file.

**Using a YAML file (e.g., `questions.yml`):**
```yaml
q1:
  type: noul
  instructions: Is the sentiment positive?
q2:
  type: score
  instructions: Rate the intensity of the emotion
  criteria: ["Low", "Medium", "High"]
```

```bash
jev --state "I am absolutely thrilled about this new feature!" --questions questions.yml
```

**Using a raw JSON string:**
```bash
jev --state "I am absolutely thrilled!" --questions '{"q1": {"type": "noul", "instructions": "Is it positive?"}, "q2": {"type": "choice", "instructions": "Which emotion?", "criteria": {"A": "Joy", "B": "Sadness"}}}'
```

---

Enjoy building decision-driven logic with `jev-cli`!
