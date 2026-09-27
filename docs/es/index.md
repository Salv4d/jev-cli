---
layout: default
title: "Documentación jev-cli"
description: "Guía completa para usar la herramienta jev-cli para el modelo Jev de TypeSafe."
---

[English](/jev-cli/) | [Português](/jev-cli/pt-br/) | [Español](/jev-cli/es/)

# Documentación jev-cli

Bienvenido a la documentación oficial de **jev-cli**. Esta interfaz de línea de comandos le permite consultar instantáneamente el modelo TypeSafe Jev localmente o a través de OpenRouter, utilizando una sintaxis posicional altamente intuitiva.

Jev evalúa "estados" (que pueden ser una cadena de texto, un bloque JSON o datos estructurados) frente a preguntas o instrucciones específicas, devolviendo decisiones JSON altamente restringidas y probabilísticas en lugar de texto no estructurado.

---

## 1. Autenticación y Configuración

Antes de usar el CLI, debe configurar su token de API.

### Configuración de su Token
Puede pasar su token de forma segura a través de variables de entorno (recomendado) o explícitamente con la bandera `--token`.

**Mac / Linux:**
```bash
export JEV_TOKEN="su_token_aqui"
```

**Windows (PowerShell):**
```powershell
$env:JEV_TOKEN="su_token_aqui"
```

### Elección de un Proveedor
Por defecto, el CLI consulta a `typesafe.ai`. Si desea enrutar sus consultas a través de OpenRouter, debe establecer explícitamente el proveedor:

**Mac / Linux:**
```bash
export JEV_PROVIDER="openrouter"
```

**Windows (PowerShell):**
```powershell
$env:JEV_PROVIDER="openrouter"
```

Alternativamente, puede pasar el proveedor dinámicamente por comando:
```bash
jev --provider openrouter "Estado" "¿Pregunta?"
```

### Sobrescritura de Modelos
Por defecto, el CLI usa el modelo `jev-latest`. Si necesita anclar a una versión específica (ej. `typesafe/jev-1.13-20260917`), puede usar la bandera `--model`:
```bash
jev --model "typesafe/jev-1.13-20260917" "Me gustan las manzanas." "¿Menciona esto una fruta?"
```

---

## 2. Uso y Tipos de Preguntas

El `jev-cli` averigua inteligentemente qué tipo de pregunta está haciendo según los argumentos que pase.

### Noul (Juicios Sí/No)
Una evaluación **Noul** actúa como un juez booleano o de sí/no. Devuelve una puntuación de probabilidad entre `0` y `1` indicando cuán cierta es la condición.

**Sintaxis:** `jev "<estado>" "<pregunta>"`

**Ejemplo:**
```bash
jev "El cielo es azul y la hierba es verde." "¿Este texto describe colores?"
```

**Salida Esperada:**
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

### Choice (Opción Múltiple)
Una evaluación **Choice** (Opción) selecciona la opción más precisa de un conjunto de opciones.

**Sintaxis:** `jev "<estado>" "<pregunta>" <opcion_1> <opcion_2> <opcion_3> ...`

**Ejemplo:**
```bash
jev "Mi animal favorito es el que tiene trompa." "¿Qué animal es?" Elefante Jirafa León
```

**Salida Esperada:**
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

### Score (Rúbrica de Puntuación)
Una evaluación **Score** (Puntuación) asigna el estado contra una leyenda/escala ordenada, identificando el índice de la escala que mejor se ajusta.

**Sintaxis:** `jev "<estado>" "<pregunta>" --scale <escala_1> <escala_2> <escala_3> ...`

**Ejemplo:**
```bash
jev "¡Me encantó este restaurante, la comida fue divina!" "Califica el sentimiento" --scale "Terrible" "Malo" "Neutral" "Bueno" "Excelente"
```

**Salida Esperada:**
```json
{
  "answers": {
    "q1": {
      "type": "score",
      "score": 4,
      "legend": {
        "0": "Terrible",
        "1": "Malo",
        "2": "Neutral",
        "3": "Bueno",
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

## 3. Ejemplos Avanzados

### Evaluación de Datos JSON
Su estado no tiene que ser una frase sin formato. ¡Puede ser una carga JSON serializada para evaluación estructurada!

```bash
jev '{"edad_usuario": 17, "tiene_licencia": false}' "¿Tiene este usuario permiso legal para conducir?"
```

### Mapeo Complejo de Opciones
Si sus opciones son oraciones largas, simplemente envuélvalas entre comillas:
```bash
jev "Error 503: Servicio no disponible" "¿Qué debe hacer el sistema?" "Reintentar inmediatamente" "Retroceder exponencialmente y reintentar" "Fallar silenciosamente"
```

---

¡Disfrute construyendo lógica impulsada por decisiones con `jev-cli`!
