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

    // EXPLICIT FLAGS (Long version)
    /// Explicitly provide the state to evaluate.
    #[arg(long = "state")]
    explicit_state: Option<String>,

    /// Explicitly provide the question or instructions.
    #[arg(long = "instructions")]
    explicit_instructions: Option<String>,

    /// Explicitly define the kind (noul, choice, score).
    #[arg(long = "type")]
    explicit_kind: Option<String>,

    /// Explicitly provide criteria as a raw JSON string (dict for choice, array for score).
    #[arg(long = "criteria")]
    explicit_criteria: Option<String>,

    // SHORTHAND FLAGS (Positional)
    /// The state to evaluate (Shorthand syntax).
    state_pos: Option<String>,

    /// The question or instructions to evaluate (Shorthand syntax).
    instructions_pos: Option<String>,

    /// Options for a choice question (Shorthand syntax).
    #[arg(trailing_var_arg = true)]
    choices: Vec<String>,

    /// Scale for a score question (Shorthand syntax).
    #[arg(long = "scale", num_args = 1..)]
    scale: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv();
    let args = Args::parse();

    if args.token.trim().is_empty() {
        eprintln!("Error: Token provided is empty.");
        std::process::exit(1);
    }

    let final_state = match args.explicit_state.or(args.state_pos) {
        Some(s) => s,
        None => {
            eprintln!("Error: You must provide a state, either positionally or via --state.");
            std::process::exit(1);
        }
    };

    let final_instructions = match args.explicit_instructions.or(args.instructions_pos) {
        Some(s) => s,
        None => {
            eprintln!("Error: You must provide instructions, either positionally or via --instructions.");
            std::process::exit(1);
        }
    };

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

    let mut q1 = serde_json::Map::new();
    q1.insert("instructions".to_string(), json!(final_instructions));

    // Determine the question kind and criteria based on arguments
    if let Some(explicit_kind) = args.explicit_kind {
        // EXPLICIT MODE
        q1.insert("type".to_string(), json!(explicit_kind.to_lowercase()));
        if let Some(crit) = args.explicit_criteria {
            let parsed_crit: Value = serde_json::from_str(&crit).context("Failed to parse --criteria as JSON")?;
            q1.insert("criteria".to_string(), parsed_crit);
        }
    } else {
        // SHORTHAND MODE
        if let Some(scale) = args.scale {
            q1.insert("type".to_string(), json!("score"));
            q1.insert("criteria".to_string(), json!(scale));
        } else if !args.choices.is_empty() {
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
            q1.insert("type".to_string(), json!("noul"));
        }
    }

    let request_body = json!({
        "model": model,
        "state": final_state,
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
