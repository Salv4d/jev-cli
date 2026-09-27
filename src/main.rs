use anyhow::{Context, Result};
use clap::Parser;
use dotenvy::dotenv;
use reqwest::Client;
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Provider {
    TypesafeAi,
    Openrouter,
}

impl std::str::FromStr for Provider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "typesafe.ai" | "typesafeai" | "typesafe" => Ok(Provider::TypesafeAi),
            "openrouter" => Ok(Provider::Openrouter),
            _ => Err(format!("Invalid provider: {}. Allowed values are 'typesafe.ai' and 'openrouter'.", s)),
        }
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::TypesafeAi => write!(f, "typesafe.ai"),
            Provider::Openrouter => write!(f, "openrouter"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "jev")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The API token to use. Can also be set via JEV_TOKEN environment variable.
    #[arg(short, long, env = "JEV_TOKEN")]
    token: String,

    /// The provider to use (typesafe.ai or openrouter). Can also be set via JEV_PROVIDER environment variable.
    #[arg(short, long, env = "JEV_PROVIDER", default_value = "typesafe.ai")]
    provider: Provider,

    /// The specific model to use. Optional. Falls back to provider defaults if not set.
    #[arg(short, long, env = "JEV_MODEL")]
    model: Option<String>,

    /// The scale for a score question. If provided, the question is evaluated as a Score.
    #[arg(long = "escala", num_args = 1..)]
    escala: Option<Vec<String>>,

    /// The state to evaluate (e.g., text or JSON context).
    state: String,

    /// The question or instructions to evaluate against the state.
    instructions: String,

    /// Options for a choice question. If provided, the question is evaluated as a Choice.
    #[arg(trailing_var_arg = true)]
    choices: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv();
    let args = Args::parse();

    if args.token.trim().is_empty() {
        eprintln!("Error: Token provided is empty.");
        std::process::exit(1);
    }

    let (endpoint, default_model) = match args.provider {
        Provider::TypesafeAi => (
            "https://api.typesafe.ai/v1/systemone",
            "jev-latest",
        ),
        Provider::Openrouter => (
            "https://openrouter.ai/api/alpha/decisions",
            "typesafe/jev-latest",
        ),
    };

    let model = args.model.unwrap_or_else(|| default_model.to_string());

    // Determine the question kind and criteria based on arguments
    let mut q1 = serde_json::Map::new();
    q1.insert("instructions".to_string(), json!(args.instructions));

    if let Some(escala) = args.escala {
        // It's a Score question
        q1.insert("type".to_string(), json!("score"));
        q1.insert("criteria".to_string(), json!(escala));
    } else if !args.choices.is_empty() {
        // It's a Choice question
        q1.insert("type".to_string(), json!("choice"));
        
        let mut criteria_map = serde_json::Map::new();
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for (i, choice) in args.choices.iter().enumerate() {
            let letter = if i < alphabet.len() {
                alphabet.chars().nth(i).unwrap().to_string()
            } else {
                format!("OPT{}", i)
            };
            criteria_map.insert(letter, json!(choice));
        }
        q1.insert("criteria".to_string(), json!(criteria_map));
    } else {
        // It's a Noul (Yes/No) question
        q1.insert("type".to_string(), json!("noul"));
    }

    let request_body = json!({
        "model": model,
        "state": args.state,
        "questions": {
            "q1": q1
        }
    });

    let client = Client::new();
    let res = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", args.token))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .context("Failed to send request")?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        eprintln!("Error response from API: {} - {}", status, body);
        std::process::exit(1);
    }

    let response_json: Value = res.json().await.context("Failed to parse JSON response")?;
    
    println!("{}", serde_json::to_string_pretty(&response_json).unwrap());

    Ok(())
}
