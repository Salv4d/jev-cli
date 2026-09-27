---
layout: default
title: "Documentação jev-cli"
description: "Guia completo para usar a ferramenta jev-cli para o modelo Jev da TypeSafe."
---

[🇺🇸 English](/jev-cli/) | [🇧🇷 Português](/jev-cli/pt-br/) | [🇪🇸 Español](/jev-cli/es/) | [🇨🇳 中文](/jev-cli/zh/)

# Documentação jev-cli

Bem-vindo à documentação oficial do **jev-cli**. Esta interface de linha de comando permite consultar instantaneamente o modelo TypeSafe Jev localmente ou via OpenRouter, utilizando uma sintaxe posicional altamente intuitiva e curta.

O Jev avalia "estados" (que podem ser uma string de texto, um bloco JSON ou dados estruturados) em relação a perguntas ou instruções direcionadas, retornando decisões JSON altamente restritas e probabilísticas em vez de texto não estruturado.

---

## 1. Autenticação e Configuração

Antes de usar o CLI, você deve configurar seu token de API e (opcionalmente) seu provedor. Por padrão, o CLI consulta `typesafe.ai`, mas você pode rotear consultas através do OpenRouter.

### Linux / macOS
```bash
# 1. Configure seu Token de API
export JEV_TOKEN="seu_token_aqui"

# 2. (Opcional) Configure seu provedor para openrouter
export JEV_PROVIDER="openrouter"
```

### Windows (PowerShell)
```powershell
# 1. Configure seu Token de API
$env:JEV_TOKEN="seu_token_aqui"

# 2. (Opcional) Configure seu provedor para openrouter
$env:JEV_PROVIDER="openrouter"
```

### Windows (Prompt de Comando)
```cmd
:: 1. Configure seu Token de API
set JEV_TOKEN=seu_token_aqui

:: 2. (Opcional) Configure seu provedor para openrouter
set JEV_PROVIDER=openrouter
```

*(Alternativamente, você pode passá-los dinamicamente por comando: `jev --token "..." --provider openrouter "Estado" "Pergunta?"`)*

### Substituições de Modelo
Por padrão, o CLI usa o modelo `jev-latest`. Se você precisar fixar em uma versão específica (ex: `typesafe/jev-1.13-20260917`), você pode usar a flag `--model`:
```bash
jev --model "typesafe/jev-1.13-20260917" "Gosto de maçãs." "Isto menciona frutas?"
```

---

## 2. Uso e Tipos de Perguntas

O `jev-cli` descobre inteligentemente qual tipo de pergunta você está fazendo com base nos argumentos que você passa.

### Noul (Julgamentos Sim/Não)
Uma avaliação **Noul** atua como um juiz booleano ou sim/não. Ela retorna uma pontuação de probabilidade entre `0` e `1` indicando o quão verdadeira é a condição.

**Sintaxe:** `jev "<estado>" "<pergunta>"`

**Exemplo:**
```bash
jev "O céu é azul e a grama é verde." "Este texto descreve cores?"
```

**Saída Esperada:**
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

### Choice (Múltipla Escolha)
Uma avaliação **Choice** (Escolha) seleciona a opção mais precisa de um conjunto de escolhas.

**Sintaxe:** `jev "<estado>" "<pergunta>" <opção_1> <opção_2> <opção_3> ...`

**Exemplo:**
```bash
jev "Meu animal favorito é aquele com uma tromba." "Qual é o animal?" Elefante Girafa Leão
```

**Saída Esperada:**
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

### Score (Rubrica de Pontuação)
Uma avaliação **Score** (Pontuação) mapeia o estado contra uma legenda/escala ordenada, identificando o índice da escala que melhor se encaixa.

**Sintaxe:** `jev "<estado>" "<pergunta>" --scale <escala_1> <escala_2> <escala_3> ...`

**Exemplo:**
```bash
jev "Eu absolutamente amei este restaurante, a comida estava divina!" "Avalie o sentimento" --scale "Terrível" "Ruim" "Neutro" "Bom" "Excelente"
```

**Saída Esperada:**
```json
{
  "answers": {
    "q1": {
      "type": "score",
      "score": 4,
      "legend": {
        "0": "Terrível",
        "1": "Ruim",
        "2": "Neutro",
        "3": "Bom",
        "4": "Excelente"
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

## 3. Exemplos Avançados

### Avaliando Dados JSON
Seu estado não precisa ser uma frase bruta. Pode ser uma carga JSON serializada para avaliação estruturada!

```bash
jev '{"idade_usuario": 17, "tem_carteira": false}' "Este usuário tem permissão legal para dirigir?"
```

### Mapeamento Complexo de Escolhas
Se suas escolhas são frases longas, basta envolvê-las em aspas:
```bash
jev "Erro 503: Serviço Indisponível" "O que o sistema deve fazer?" "Tentar novamente imediatamente" "Recuar exponencialmente e tentar novamente" "Falhar silenciosamente"
```

---

Aproveite construindo lógica orientada a decisões com o `jev-cli`!
