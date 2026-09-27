use anyhow::{Context, Result};
use clap::Parser;
use dotenvy::dotenv;
use reqwest::Client;
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Provider {
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

#[derive(Parser, Debug, Clone)]
#[command(name = "jev")]
#[command(author, version, about, long_about = None)]
pub struct Args {
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

    /// Read multiple questions from a JSON/YAML file or a raw JSON string.
    #[arg(long = "questions")]
    questions: Option<String>,

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

pub fn build_endpoint(provider: &Provider) -> (&'static str, &'static str) {
    match provider {
        Provider::TypesafeAi => (
            "https://api.typesafe.ai/v1/systemone",
            "jev-latest",
        ),
        Provider::Openrouter => (
            "https://openrouter.ai/api/alpha/decisions",
            "typesafe/jev-1.13",
        ),
    }
}

pub fn build_request_body(args: &Args) -> Result<Value> {
    let final_state = args.explicit_state.clone().or_else(|| args.state_pos.clone()).context("Error: You must provide a state, either positionally or via --state.")?;

    let (_, default_model) = build_endpoint(&args.provider);
    let model = args.model.clone().unwrap_or_else(|| default_model.to_string());

    let mut questions_map = serde_json::Map::new();

    if let Some(ref q_arg) = args.questions {
        // It could be a raw JSON string or a file path
        let parsed_questions: Value = if q_arg.trim().starts_with('{') {
            serde_json::from_str(q_arg).context("Failed to parse --questions as raw JSON")?
        } else {
            let file_content = std::fs::read_to_string(q_arg).with_context(|| format!("Failed to read questions file: {}", q_arg))?;
            // Try YAML first (YAML is a superset of JSON)
            serde_yaml::from_str(&file_content).context("Failed to parse questions file as YAML/JSON")?
        };

        if let Some(map) = parsed_questions.as_object() {
            for (k, v) in map {
                questions_map.insert(k.clone(), v.clone());
            }
        } else {
            anyhow::bail!("The --questions argument or file must contain a JSON/YAML object (dictionary) at its root.");
        }
    } else {
        let final_instructions = args.explicit_instructions.clone().or_else(|| args.instructions_pos.clone()).context("Error: You must provide instructions, either positionally or via --instructions (or use --questions for multiple questions).")?;

        let mut q1 = serde_json::Map::new();
        q1.insert("instructions".to_string(), json!(final_instructions));

        // Determine the question kind and criteria based on arguments
        if let Some(ref explicit_kind) = args.explicit_kind {
            // EXPLICIT MODE
            q1.insert("type".to_string(), json!(explicit_kind.to_lowercase()));
            if let Some(ref crit) = args.explicit_criteria {
                let parsed_crit: Value = serde_json::from_str(crit).context("Failed to parse --criteria as JSON")?;
                q1.insert("criteria".to_string(), parsed_crit);
            }
        } else {
            // SHORTHAND MODE
            if let Some(ref scale) = args.scale {
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
        questions_map.insert("q1".to_string(), json!(q1));
    }

    Ok(json!({
        "model": model,
        "state": final_state,
        "questions": questions_map
    }))
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv();
    let args = Args::parse();

    if args.token.trim().is_empty() {
        eprintln!("Error: Token provided is empty.");
        std::process::exit(1);
    }

    let (endpoint, _) = build_endpoint(&args.provider);
    
    let request_body = match build_request_body(&args) {
        Ok(body) => body,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

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

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a base args struct for testing
    fn get_base_args() -> Args {
        Args {
            token: "test_token".to_string(),
            provider: Provider::TypesafeAi,
            model: None,
            explicit_state: None,
            questions: None,
            explicit_instructions: None,
            explicit_kind: None,
            explicit_criteria: None,
            state_pos: None,
            instructions_pos: None,
            choices: vec![],
            scale: None,
        }
    }

    #[test]
    fn test_build_endpoint_typesafe() {
        let (endpoint, model) = build_endpoint(&Provider::TypesafeAi);
        assert_eq!(endpoint, "https://api.typesafe.ai/v1/systemone");
        assert_eq!(model, "jev-latest");
    }

    #[test]
    fn test_build_endpoint_openrouter() {
        let (endpoint, model) = build_endpoint(&Provider::Openrouter);
        assert_eq!(endpoint, "https://openrouter.ai/api/alpha/decisions");
        assert_eq!(model, "typesafe/jev-1.13");
    }

    #[test]
    fn test_shorthand_noul() {
        let mut args = get_base_args();
        args.state_pos = Some("My State".to_string());
        args.instructions_pos = Some("My Question".to_string());

        let body = build_request_body(&args).unwrap();
        assert_eq!(body["state"], "My State");
        assert_eq!(body["questions"]["q1"]["instructions"], "My Question");
        assert_eq!(body["questions"]["q1"]["type"], "noul");
        assert!(body["questions"]["q1"].get("criteria").is_none());
    }

    #[test]
    fn test_shorthand_choice() {
        let mut args = get_base_args();
        args.state_pos = Some("My State".to_string());
        args.instructions_pos = Some("My Question".to_string());
        args.choices = vec!["Opt1".to_string(), "Opt2".to_string()];

        let body = build_request_body(&args).unwrap();
        assert_eq!(body["questions"]["q1"]["type"], "choice");
        let criteria = &body["questions"]["q1"]["criteria"];
        assert_eq!(criteria["A"], "Opt1");
        assert_eq!(criteria["B"], "Opt2");
    }

    #[test]
    fn test_shorthand_score() {
        let mut args = get_base_args();
        args.state_pos = Some("My State".to_string());
        args.instructions_pos = Some("My Question".to_string());
        args.scale = Some(vec!["Bad".to_string(), "Good".to_string()]);

        let body = build_request_body(&args).unwrap();
        assert_eq!(body["questions"]["q1"]["type"], "score");
        let criteria = &body["questions"]["q1"]["criteria"];
        assert_eq!(criteria[0], "Bad");
        assert_eq!(criteria[1], "Good");
    }

    #[test]
    fn test_explicit_noul() {
        let mut args = get_base_args();
        args.explicit_state = Some("My State".to_string());
        args.explicit_instructions = Some("My Question".to_string());
        args.explicit_kind = Some("noul".to_string());

        let body = build_request_body(&args).unwrap();
        assert_eq!(body["state"], "My State");
        assert_eq!(body["questions"]["q1"]["type"], "noul");
    }

    #[test]
    fn test_explicit_choice() {
        let mut args = get_base_args();
        args.explicit_state = Some("My State".to_string());
        args.explicit_instructions = Some("My Question".to_string());
        args.explicit_kind = Some("choice".to_string());
        args.explicit_criteria = Some(r#"{"X":"Yes", "Y":"No"}"#.to_string());

        let body = build_request_body(&args).unwrap();
        assert_eq!(body["questions"]["q1"]["type"], "choice");
        let criteria = &body["questions"]["q1"]["criteria"];
        assert_eq!(criteria["X"], "Yes");
        assert_eq!(criteria["Y"], "No");
    }

    #[test]
    fn test_explicit_score() {
        let mut args = get_base_args();
        args.explicit_state = Some("My State".to_string());
        args.explicit_instructions = Some("My Question".to_string());
        args.explicit_kind = Some("score".to_string());
        args.explicit_criteria = Some(r#"["Low", "High"]"#.to_string());

        let body = build_request_body(&args).unwrap();
        assert_eq!(body["questions"]["q1"]["type"], "score");
        let criteria = &body["questions"]["q1"]["criteria"];
        assert_eq!(criteria[0], "Low");
        assert_eq!(criteria[1], "High");
    }

    #[test]
    fn test_missing_state_error() {
        let mut args = get_base_args();
        args.instructions_pos = Some("Q".to_string());
        
        let result = build_request_body(&args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Error: You must provide a state, either positionally or via --state.");
    }

    #[test]
    fn test_multiple_questions_json() {
        let mut args = get_base_args();
        args.explicit_state = Some("My State".to_string());
        args.questions = Some(r#"{
            "q1": {"type": "noul", "instructions": "Q1?"},
            "q2": {"type": "score", "instructions": "Q2?", "criteria": ["Bad", "Good"]}
        }"#.to_string());

        let body = build_request_body(&args).unwrap();
        assert_eq!(body["questions"]["q1"]["type"], "noul");
        assert_eq!(body["questions"]["q2"]["type"], "score");
        assert_eq!(body["questions"]["q2"]["criteria"][0], "Bad");
    }

    #[test]
    fn test_multiple_questions_yaml() {
        let mut args = get_base_args();
        args.explicit_state = Some("My State".to_string());
        // By simulating a file read (since the string doesn't start with '{'), the logic attempts to read a file.
        // We can't easily mock the file system here without creating a temp file. Let's create one.
        use std::io::Write;
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(temp_file, "q1:\n  type: noul\n  instructions: 'Q1?'").unwrap();
        
        args.questions = Some(temp_file.path().to_str().unwrap().to_string());

        let body = build_request_body(&args).unwrap();
        assert_eq!(body["questions"]["q1"]["type"], "noul");
        assert_eq!(body["questions"]["q1"]["instructions"], "Q1?");
    }
}
