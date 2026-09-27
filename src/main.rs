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

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq)]
enum QuestionKind {
    Noul,
    Choice,
    Score,
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

    /// The state to evaluate (e.g., a JSON string or text).
    #[arg(short, long)]
    state: String,

    /// The kind of question to ask (noul, choice, score).
    #[arg(short, long, value_enum)]
    kind: QuestionKind,

    /// Instructions for the question.
    #[arg(short, long, default_value = "Evaluate the state")]
    instructions: String,

    /// Criteria for Choice or Score kinds (format as JSON string array or object).
    #[arg(short, long)]
    criteria: Option<String>,
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

    // Build the "q1" object based on kind
    let mut q1 = serde_json::Map::new();
    
    let kind_str = match args.kind {
        QuestionKind::Noul => "noul",
        QuestionKind::Choice => "choice",
        QuestionKind::Score => "score",
    };
    q1.insert("type".to_string(), json!(kind_str));
    q1.insert("instructions".to_string(), json!(args.instructions));

    if args.kind == QuestionKind::Choice || args.kind == QuestionKind::Score {
        if let Some(criteria_str) = &args.criteria {
            let parsed_criteria: Value = serde_json::from_str(criteria_str).context("Criteria must be valid JSON")?;
            q1.insert("criteria".to_string(), parsed_criteria);
        } else {
            eprintln!("Error: --criteria is required for choice and score kinds.");
            std::process::exit(1);
        }
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
    
    // Output the formatted JSON cleanly to stdout
    println!("{}", serde_json::to_string_pretty(&response_json).unwrap());

    Ok(())
}
