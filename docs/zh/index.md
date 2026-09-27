---
layout: default
title: "jev-cli 文档"
description: "使用 jev-cli 工具调用 TypeSafe Jev 模型的全面指南。"
---

[English](/jev-cli/) | [Português](/jev-cli/pt-br/) | [Español](/jev-cli/es/) | [中文](/jev-cli/zh/)

# jev-cli 文档

欢迎来到 **jev-cli** 的官方文档。此命令行界面允许您在本地或通过 OpenRouter 即时查询 TypeSafe Jev 模型，使用高度直观的位置简写语法。

Jev 评估“状态”（可以是文本字符串、JSON 块或结构化数据），针对目标问题或指令，返回高度受限且带有概率的 JSON 决策，而不是非结构化文本。

---

## 1. 认证与配置

在使用 CLI 之前，您必须设置您的 API 令牌。

### 设置您的令牌
您可以通过环境变量（推荐）或明确使用 `--token` 标志安全地传递您的令牌。

**Mac / Linux:**
```bash
export JEV_TOKEN="您的_token_在这里"
```

**Windows (PowerShell):**
```powershell
$env:JEV_TOKEN="您的_token_在这里"
```

### 选择提供商
默认情况下，CLI 查询 `typesafe.ai`。如果您想通过 OpenRouter 路由您的查询，您必须明确设置提供商：

**Mac / Linux:**
```bash
export JEV_PROVIDER="openrouter"
```

**Windows (PowerShell):**
```powershell
$env:JEV_PROVIDER="openrouter"
```

或者，您可以按命令动态传递提供商：
```bash
jev --provider openrouter "状态" "问题？"
```

### 模型覆盖
默认情况下，CLI 使用 `jev-latest` 模型。如果您需要固定到特定版本（例如 `typesafe/jev-1.13-20260917`），您可以使用 `--model` 标志：
```bash
jev --model "typesafe/jev-1.13-20260917" "我喜欢苹果。" "这提到了水果吗？"
```

---

## 2. 用法和问题类型

`jev-cli` 会根据您传递的参数智能判断您正在询问的问题类型。

### Noul (是/否 判断)
**Noul** 评估充当布尔或“是/否”判定器。它返回 `0` 到 `1` 之间的概率分数，表示条件为真的程度。

**语法:** `jev "<状态>" "<问题>"`

**示例:**
```bash
jev "天空是蓝色的，草是绿色的。" "这段文字描述了颜色吗？"
```

**预期输出:**
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

### Choice (多项选择)
**Choice** 评估从一组选项中选择最准确的一项。

**语法:** `jev "<状态>" "<问题>" <选项_1> <选项_2> <选项_3> ...`

**示例:**
```bash
jev "我最喜欢的动物是长鼻子的那种。" "这是什么动物？" 大象 长颈鹿 狮子
```

**预期输出:**
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

### Score (评分标准)
**Score** 评估将状态与有序图例/标度进行映射，识别最合适的标度索引。

**语法:** `jev "<状态>" "<问题>" --scale <标度_1> <标度_2> <标度_3> ...`

**示例:**
```bash
jev "我绝对喜欢这家餐厅，食物太棒了！" "对情绪进行评分" --scale "糟糕" "差" "中立" "好" "极好"
```

**预期输出:**
```json
{
  "answers": {
    "q1": {
      "type": "score",
      "score": 4,
      "legend": {
        "0": "糟糕",
        "1": "差",
        "2": "中立",
        "3": "好",
        "4": "极好"
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

## 3. 高级示例

### 评估 JSON 数据
您的状态不必是原始句子。它可以是序列化的 JSON 数据以进行结构化评估！

```bash
jev '{"user_age": 17, "has_license": false}' "这个用户合法允许驾驶吗？"
```

### 复杂的选择映射
如果您的选项是很长的句子，只需用引号将它们包围：
```bash
jev "错误 503: 服务不可用" "系统应该怎么做？" "立即重试" "指数退避并重试" "静默失败"
```

---

尽情享受使用 `jev-cli` 构建决策驱动的逻辑吧！
