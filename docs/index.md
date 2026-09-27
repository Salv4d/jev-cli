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

Before using the CLI, you must set up your API token. 

### Setting your Token
You can pass your token securely via environment variables (recommended) or explicitly with the `--token` flag.

**Mac / Linux:**
```bash
export JEV_TOKEN="your_token_here"
```

**Windows (PowerShell):**
```powershell
$env:JEV_TOKEN="your_token_here"
```

### Choosing a Provider
By default, the CLI queries `typesafe.ai`. If you want to route your queries through OpenRouter, you must explicitly set the provider:

**Mac / Linux:**
```bash
export JEV_PROVIDER="openrouter"
```

**Windows (PowerShell):**
```powershell
$env:JEV_PROVIDER="openrouter"
```

Alternatively, you can pass the provider dynamically per command:
```bash
jev --provider openrouter "State" "Question?"
```

### Model Overrides
By default, the CLI uses the `jev-latest` model. If you need to pin to a specific version (e.g. `typesafe/jev-1.13-20260917`), you can use the `--model` flag:
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

---

Enjoy building decision-driven logic with `jev-cli`!
