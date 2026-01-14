//! Data loader module for loading token data from JSON files.

use crate::models::{AIEvolutionCategory, Token, TokenType};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Load tokens from a JSON file.
#[derive(Deserialize)]
struct TokenJson {
    symbol: String,
    name: String,
    #[serde(default)]
    token_type: Option<String>,
    archetype: String,
    insider_score: i32,
    tags: Vec<String>,
    tariff_override: i32,
    reason: String,
    #[serde(default)]
    parent_token: Option<String>,
    #[serde(default)]
    commodity_type: Option<String>,
    #[serde(default)]
    rank: Option<i32>,
    #[serde(default)]
    ai_category: Option<String>,
    #[serde(default)]
    market_cap: Option<f64>,
    #[serde(default)]
    fdv: Option<f64>,
    #[serde(default)]
    price: Option<f64>,
}

/// Convert JSON token to domain model.
impl From<TokenJson> for Token {
    fn from(json: TokenJson) -> Self {
        use crate::models::RiskClass;
        let archetype = match json.archetype.as_str() {
            "Class A (Real Yield)" => RiskClass::ClassA,
            "Class B (Systemic)" => RiskClass::ClassB,
            "Class C (Venture Risk)" => RiskClass::ClassC,
            "Class D (Speculative)" => RiskClass::ClassD,
            _ => RiskClass::ClassA, // Default
        };

        // Parse token_type if provided, otherwise leave as default (ProtocolUtility)
        // Auto-detection will happen later based on tags
        let token_type = match json.token_type {
            Some(tt) => match tt.as_str() {
                "fiat_pegged" => TokenType::FiatPegged,
                "commodity_backed" => TokenType::CommodityBacked,
                "hard_money" => TokenType::HardMoney,
                "protocol_utility" => TokenType::ProtocolUtility,
                "real_yield" => TokenType::RealYield,
                "wrapped_bridge" => TokenType::WrappedBridge,
                "liquid_staking" => TokenType::LiquidStaking,
                "governance" => TokenType::Governance,
                "meme" => TokenType::Meme,
                "stableyield" => TokenType::StableYield,
                _ => TokenType::ProtocolUtility, // Default
            },
            None => TokenType::ProtocolUtility, // Will be detected from tags
        };

        // Parse ai_category if provided, otherwise use default (PassiveUtility)
        // Auto-detection will happen later based on tags
        let ai_category = match json.ai_category {
            Some(ai) => match ai.as_str() {
                "static" => AIEvolutionCategory::Static,
                "passive_utility" => AIEvolutionCategory::PassiveUtility,
                "ai_enabled" => AIEvolutionCategory::AIEnabled,
                "ai_native" => AIEvolutionCategory::AINative,
                "ai_evolving" => AIEvolutionCategory::AIEvolving,
                _ => AIEvolutionCategory::PassiveUtility, // Default
            },
            None => AIEvolutionCategory::PassiveUtility, // Will be detected from tags
        };

        Token {
            symbol: json.symbol,
            name: json.name,
            token_type,
            archetype,
            insider_score: json.insider_score,
            tags: json.tags,
            tariff_override: json.tariff_override,
            reason: json.reason,
            parent_token: json.parent_token,
            commodity_type: json.commodity_type,
            rank: json.rank.unwrap_or(9999), // Default to 9999 for unknown rank
            ai_category,
            market_cap: json.market_cap.unwrap_or(0.0),
            price: json.price.unwrap_or(0.0),
            fdv: json.fdv,
        }
    }
}

/// Load all tokens from JSON files in a directory.
pub fn load_tokens(data_dir: &str) -> Result<Vec<Token>, anyhow::Error> {
    let path = Path::new(data_dir);

    if !path.exists() {
        return Err(anyhow::anyhow!("Data directory does not exist: {}", data_dir));
    }

    let mut tokens = Vec::new();

    // Read all JSON files in the directory
    let entries = fs::read_dir(path)?;

    let mut total_files = 0;
    let mut loaded_files = 0;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "json" {
                    total_files += 1;
                    match load_tokens_from_file(&path, &mut tokens) {
                        Ok(count) => {
                            println!("  ✓ Loaded {} tokens from {}", count, path.display());
                            loaded_files += 1;
                        }
                        Err(e) => {
                            println!("  ✗ Failed to load {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }
    }

    if tokens.is_empty() {
        return Err(anyhow::anyhow!(
            "No JSON files found in {}",
            data_dir
        ));
    }

    // Filter out tokens that don't have price or market cap data
    tokens.retain(|t| t.price > 0.0 && t.market_cap > 0.0);

    // Auto-detect token types for tokens that don't have explicit type
    for token in &mut tokens {
        if matches!(token.token_type, TokenType::ProtocolUtility) && !token.tags.is_empty() {
            let detected = token.detect_token_type();
            if !matches!(detected, TokenType::ProtocolUtility) {
                token.token_type = detected;
            }
        }
    }

    println!("Total tokens loaded: {}", tokens.len());
    Ok(tokens)
}

/// Load tokens from a single JSON file.
fn load_tokens_from_file(path: &Path, tokens: &mut Vec<Token>) -> Result<usize, anyhow::Error> {
    let content = fs::read_to_string(path)?;
    let token_jsons: Vec<TokenJson> = serde_json::from_str(&content)?;

    let count = token_jsons.len();
    for json in token_jsons {
        tokens.push(json.into());
    }

    Ok(count)
}
