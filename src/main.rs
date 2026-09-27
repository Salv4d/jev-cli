use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
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

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
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

    /// The state to pass (e.g., a JSON string or path to a state file).
    #[arg(short, long)]
    state: String,

    /// The kind of question to ask.
    #[arg(short, long, value_enum)]
    kind: QuestionKind,
}

fn main() -> Result<()> {
    // Load environment variables from .env file if it exists
    let _ = dotenv();

    let args = Args::parse();

    if args.token.trim().is_empty() {
        eprintln!("Error: Token provided is empty.");
        std::process::exit(1);
    }

    println!("✓ Token is valid.");
    println!("✓ Provider: {}", args.provider);
    println!("✓ State: {}", args.state);
    println!("✓ Question Kind: {:?}", args.kind);
    
    // Add any further logic here based on state and kind.
    
    Ok(())
}
