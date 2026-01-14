use serde::{Deserialize, Serialize};
use std::fs;
use crate::utils::{calculate_effective_tariff, format_sci, format_smart_price, format_compact_val, format_tariff_rate, format_multiplier};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub symbol: String,
    pub name: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub archetype: Option<String>,
    #[serde(default)]
    pub insider_score: Option<i32>,
    #[serde(default)]
    pub tariff_override: Option<f64>,
    #[serde(default)]
    pub ai_category: Option<String>,
    #[serde(default)]
    pub rank: Option<i32>,
    #[serde(default)]
    pub token_type: Option<String>,
    #[serde(default)]
    pub parent_token: Option<String>,
    pub price: f64,
    pub market_cap: f64,
    // Keep risk_class as alias for archetype for backwards compatibility
    #[serde(default, alias = "risk_class")]
    pub risk_class_compat: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAnalysisResult {
    pub symbol: String,
    pub name: String,
    pub token_type: String,
    pub risk_class: String,
    pub insider_score: i32,
    pub tariff_override: f64,
    pub exchange_multiplier: f64,
    pub base_multiplier_range: String,
    pub type_adjustment_factor: f64,
    pub risk_class_adjustment: f64,
    pub insider_risk_factor: f64,
    pub ai_timeline_factor: f64,
    pub liquidity_risk_factor: f64,
    pub ai_category: String,
    pub real_valuation_multiplier: f64,
    pub uncertainty_min: f64,
    pub uncertainty_max: f64,
    pub current_price: f64,
    pub projected_price: f64,
    pub market_cap: f64,
    pub trading_signal: String,
    pub reasoning: String,
}

#[derive(Debug, Clone)]
pub enum AIEvolutionCategory {
    Static,           // Cannot evolve (BTC, gold)
    PassiveUtility,   // Limited AI adaptation
    AIEnabled,        // Can integrate AI
    AINative,         // Built for AI ecosystems
    AIEvolving,       // Self-modifying with AI
}

impl AIEvolutionCategory {
    pub fn from_tags(tags: &[String], explicit: Option<&str>) -> Self {
        if let Some(cat) = explicit {
            return match cat.to_lowercase().as_str() {
                "static" => AIEvolutionCategory::Static,
                "passive_utility" | "passiveutility" => AIEvolutionCategory::PassiveUtility,
                "ai_enabled" | "aienabled" => AIEvolutionCategory::AIEnabled,
                "ai_native" | "ainative" => AIEvolutionCategory::AINative,
                "ai_evolving" | "aievolving" => AIEvolutionCategory::AIEvolving,
                _ => AIEvolutionCategory::PassiveUtility,
            };
        }

        let has_ai = tags.iter().any(|t| t.to_lowercase() == "ai");
        let has_agent = tags.iter().any(|t| t.to_lowercase() == "agent");
        let has_dao = tags.iter().any(|t| t.to_lowercase() == "dao");
        let has_investment = tags.iter().any(|t| t.to_lowercase() == "investment");
        let has_gpu = tags.iter().any(|t| t.to_lowercase() == "gpu-compute");
        let has_depin = tags.iter().any(|t| t.to_lowercase() == "depin");
        let has_hard_money = tags.iter().any(|t| t.to_lowercase() == "hard-money");
        let has_pow = tags.iter().any(|t| t.to_lowercase() == "pow");
        let has_gold = tags.iter().any(|t| t.to_lowercase() == "gold");
        let has_commodity = tags.iter().any(|t| t.to_lowercase() == "commodity");

        // AI-native indicators
        if has_ai && (has_agent || has_dao || has_investment) {
            return AIEvolutionCategory::AINative;
        }

        // AI compute / GPU / DePIN
        if has_gpu || has_depin || (has_ai && (has_gpu || has_depin)) {
            return AIEvolutionCategory::AIEnabled;
        }

        // Hard money / static
        if has_hard_money || has_pow || has_gold || has_commodity {
            return AIEvolutionCategory::Static;
        }

        AIEvolutionCategory::PassiveUtility
    }

    pub fn timeline_multiplier(&self) -> f64 {
        match self {
            AIEvolutionCategory::Static => 0.20,
            AIEvolutionCategory::PassiveUtility => 1.80,
            AIEvolutionCategory::AIEnabled => 5.00,
            AIEvolutionCategory::AINative => 25.00,
            AIEvolutionCategory::AIEvolving => 40.00,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            AIEvolutionCategory::Static => "Static",
            AIEvolutionCategory::PassiveUtility => "Passive Utility",
            AIEvolutionCategory::AIEnabled => "AI-Enabled",
            AIEvolutionCategory::AINative => "AI-Native",
            AIEvolutionCategory::AIEvolving => "AI-Evolving",
        }
    }
}

pub fn load_tokens(data_dir: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    
    for i in 1..=22 {
        let path = format!("{}/{}.json", data_dir, i);
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&contents) {
                if let Some(arr) = data.as_array() {
                    for item in arr {
                        if let Ok(token) = serde_json::from_value(item.clone()) {
                            tokens.push(token);
                        }
                    }
                }
            }
        }
    }
    
    // Filter out tokens that don't have price or market cap data
    tokens.retain(|t| t.price > 0.0 && t.market_cap > 0.0);
    
    tokens
}

/// Converts PascalCase token type to space-separated display name with emojis
fn format_token_type(token_type: &str) -> String {
    match token_type {
        "HardMoney" => "🪙 Hard Money".to_string(),
        "WrappedBridge" => "🌉 Wrapped Bridge".to_string(),
        "CommodityBacked" => "🧈 Commodity Backed".to_string(),
        "RealYield" => "💸 Real Yield".to_string(),
        "StableYield" => "🛡️ Stable Yield".to_string(),
        "FiatPegged" => "💵 Fiat Pegged".to_string(),
        "LiquidStaking" => "💧 Liquid Staking".to_string(),
        "ProtocolUtility" => "⚙️ Protocol Utility".to_string(),
        "Governance" => "🗳️ Governance".to_string(),
        "Meme" => "🤣 Meme".to_string(),
        "AINative" => "🤖 AI Native".to_string(),
        "AIEnabled" => "🧠 AI Enabled".to_string(),
        _ => format!("🔸 {}", token_type),
    }
}

fn format_risk_class(risk_class: &str) -> String {
    let emoji = match risk_class {
        rc if rc.contains("Class A") => "🛡️",
        rc if rc.contains("Class B") => "🏦",
        rc if rc.contains("Class C") => "🚀",
        rc if rc.contains("Class D") => "🤣",
        rc if rc.contains("Class E") => "🧪",
        _ => "🔸",
    };
    format!("{} {}", emoji, risk_class)
}

fn format_ai_category(category: &str) -> String {
    let emoji = match category {
        "Static" => "🗿",
        "Passive Utility" => "🔋",
        "AI-Enabled" => "🧠",
        "AI-Native" => "🤖",
        "AI-Evolving" => "🧬",
        _ => "🔸",
    };
    format!("{} {}", emoji, category)
}

fn format_trend(multiplier: f64) -> &'static str {
    if multiplier >= 1.0 {
        "📈"
    } else {
        "📉"
    }
}


pub fn analyze_token(token: &Token) -> TokenAnalysisResult {
    let ai_category = AIEvolutionCategory::from_tags(&token.tags, token.ai_category.as_deref());
    let ai_timeline_factor = ai_category.timeline_multiplier();
    
    // Determine token type and base multipliers - use explicit token_type if available
    let (token_type, base_min, base_max) = if let Some(ref explicit_type) = token.token_type {
        determine_token_type_explicit(explicit_type)
    } else {
        determine_token_type(&token.tags)
    };
    
    // Risk class adjustment - check archetype first, then risk_class_compat for backwards compatibility
    let risk_class = token.archetype.clone()
        .or_else(|| token.risk_class_compat.clone())
        .unwrap_or_default();
    let risk_adjustment = match risk_class.as_str() {
        "Class A (Real Yield)" => 1.2,
        "Class B (Systemic)" => 1.0,
        "Class C (Venture Risk)" | "Class C (Speculative)" => 0.8,
        "Class D (Memetic)" | "Class D (Speculative)" => 0.5,
        "Class E (Experimental)" => 0.3,
        _ => 1.0,
    };
    
    // Insider score factor (Range: 0.5 to 1.0)
    let insider_score = token.insider_score.unwrap_or(50);
    let insider_factor = 1.0 - ((insider_score as f64 / 100.0) * 0.5);
    
    // Tariff override
    let tariff_override = token.tariff_override.unwrap_or(0.0);
    let exchange_multiplier = 1.0 + (tariff_override / 100.0);
    
    // Calculate real valuation multiplier
    let type_factor = (base_min + base_max) / 2.0;
    let real_multiplier = type_factor * risk_adjustment * insider_factor * ai_timeline_factor * exchange_multiplier;
    
    // Rank-based capital flight factor - uses actual rank for proper premium/penalty
    let rank = token.rank.unwrap_or(500);
    let rank_factor = if rank < 10 {
        1.2  // Blue-chip premium for top 10
    } else if rank < 50 {
        1.0  // No penalty for top 50
    } else if rank < 100 {
        0.8  // Slight penalty for mid-tier
    } else if rank < 300 {
        0.5  // Moderate penalty for lower-mid tier
    } else {
        0.2  // Heavy penalty for illiquid alts
    };
    
    // Liquidity Risk Factor - penalty for small market caps
    let liquidity_risk_factor = if token.symbol == "SOL" {
        1.0
    } else if token.market_cap >= 1_000_000_000.0 {
        1.0
    } else if token.market_cap >= 100_000_000.0 {
        0.95 // 5% penalty
    } else {
        0.90 // 10% penalty
    };
    
    let real_multiplier = real_multiplier * rank_factor * liquidity_risk_factor;
    
    // Uncertainty range
    let uncertainty_min = real_multiplier * 0.8;
    let uncertainty_max = real_multiplier * 1.4;
    
    // Projected price
    let current_price = token.price;
    let projected_price = current_price * real_multiplier;
    
    // Trading signal
    let trading_signal = match real_multiplier {
        m if m >= 10.0 => "HOLD",
        m if m >= 3.0 => "BUY",
        m if m >= 1.0 => "HOLD",
        m if m >= 0.2 => "SELL",
        _ => "AVOID",
    }.to_string();
    
    // Generate reasoning
    let reasoning = generate_reasoning(&token.symbol, &token_type, &risk_class, insider_score, &ai_category, ai_timeline_factor, real_multiplier, &trading_signal, projected_price, rank, liquidity_risk_factor, token.market_cap);
    
    TokenAnalysisResult {
        symbol: token.symbol.clone(),
        name: token.name.clone(),
        token_type,
        risk_class,
        insider_score,
        tariff_override,
        exchange_multiplier,
        base_multiplier_range: format!("{:.0}x - {:.0}x", base_min, base_max),
        type_adjustment_factor: type_factor,
        risk_class_adjustment: risk_adjustment,
        insider_risk_factor: insider_factor,
        ai_timeline_factor,
        liquidity_risk_factor,
        ai_category: ai_category.display_name().to_string(),
        real_valuation_multiplier: real_multiplier,
        uncertainty_min,
        uncertainty_max,
        current_price,
        projected_price,
        market_cap: token.market_cap,
        trading_signal,
        reasoning,
    }
}

fn determine_token_type(tags: &[String]) -> (String, f64, f64) {
    let has_hard_money = tags.iter().any(|t| t.to_lowercase() == "hard-money");
    let has_pow = tags.iter().any(|t| t.to_lowercase() == "pow");
    let has_wrapped_bridge = tags.iter().any(|t| t.to_lowercase() == "wrappedbridge");
    let has_commodity = tags.iter().any(|t| t.to_lowercase() == "commodity");
    let has_gold = tags.iter().any(|t| t.to_lowercase() == "gold");
    let has_real_yield = tags.iter().any(|t| t.to_lowercase() == "real-yield");
    let has_stable_yield = tags.iter().any(|t| t.to_lowercase() == "stableyield");
    let has_fiat_pegged = tags.iter().any(|t| t.to_lowercase() == "fiat-pegged");
    let has_liquid_staking = tags.iter().any(|t| t.to_lowercase() == "liquid-staking");
    let has_protocol_utility = tags.iter().any(|t| t.to_lowercase() == "protocol-utility");
    let has_governance = tags.iter().any(|t| t.to_lowercase() == "governance");
    let has_meme = tags.iter().any(|t| t.to_lowercase() == "meme");
    
    if has_hard_money || has_pow {
        return ("HardMoney".to_string(), 20.0, 50.0);
    }
    if has_wrapped_bridge {
        return ("WrappedBridge".to_string(), 10.0, 25.0);
    }
    if has_commodity || has_gold {
        return ("CommodityBacked".to_string(), 5.0, 10.0);
    }
    if has_real_yield {
        return ("RealYield".to_string(), 0.5, 1.5);
    }
    if has_stable_yield || has_fiat_pegged {
        return ("StableYield".to_string(), 0.01, 0.2);
    }
    if has_liquid_staking {
        return ("LiquidStaking".to_string(), 0.5, 1.5);
    }
    if has_protocol_utility {
        return ("ProtocolUtility".to_string(), 10.0, 25.0);
    }
    if has_governance {
        return ("Governance".to_string(), 0.1, 1.0);
    }
    if has_meme {
        return ("Meme".to_string(), 0.01, 0.2);
    }
    
    ("ProtocolUtility".to_string(), 5.0, 15.0)
}

/// Maps explicit token_type field to type name and base multiplier range
fn determine_token_type_explicit(explicit_type: &str) -> (String, f64, f64) {
    match explicit_type.to_lowercase().as_str() {
        "hardmoney" | "hard_money" => ("HardMoney".to_string(), 20.0, 50.0),
        "wrappedbridged" | "wrappedbridge" | "wrapped_bridge" => ("WrappedBridge".to_string(), 10.0, 25.0),
        "commoditybacked" | "commodity_backed" => ("CommodityBacked".to_string(), 5.0, 10.0),
        "realyield" | "real_yield" => ("RealYield".to_string(), 0.5, 1.5),
        "stableyield" | "stable_yield" => ("StableYield".to_string(), 0.1, 0.5),
        "fiatpegged" | "fiat_pegged" => ("FiatPegged".to_string(), 0.01, 0.05),
        "liquidstaking" | "liquid_staking" => ("LiquidStaking".to_string(), 0.5, 1.5),
        "protocolutility" | "protocol_utility" => ("ProtocolUtility".to_string(), 10.0, 25.0),
        "governance" => ("Governance".to_string(), 0.1, 1.0),
        "meme" => ("Meme".to_string(), 0.01, 0.2),
        "ainative" | "ai_native" => ("AINative".to_string(), 15.0, 40.0),
        "aienabled" | "ai_enabled" => ("AIEnabled".to_string(), 10.0, 30.0),
        _ => ("ProtocolUtility".to_string(), 5.0, 15.0),
    }
}

fn generate_reasoning(symbol: &str, token_type: &str, risk_class: &str, insider_score: i32, ai_category: &AIEvolutionCategory, ai_timeline_factor: f64, real_multiplier: f64, signal: &str, projected_price: f64, rank: i32, _liquidity_risk_factor: f64, market_cap: f64) -> String {
    let ai_desc = match ai_category {
        AIEvolutionCategory::Static => "Asset classified as Static (Cannot Evolve). Static assets that cannot adapt to AI acceleration. These assets decline in value as AI progresses.",
        AIEvolutionCategory::PassiveUtility => "Asset classified as Passive Utility (Limited AI Adaptation). These assets have limited ability to benefit from AI acceleration.",
        AIEvolutionCategory::AIEnabled => "Asset classified as AI-Enabled (Can Integrate AI). These assets can incorporate AI capabilities and benefit from acceleration.",
        AIEvolutionCategory::AINative => "Asset classified as AI-Native (Built specifically for AI ecosystems. Maximum AI acceleration benefits.).",
        AIEvolutionCategory::AIEvolving => "Asset classified as AI-Evolving (Self-Modifying with AI. Highest potential for AI-driven evolution.).",
    };
    
    let ai_impact = if ai_timeline_factor < 1.0 {
        format!("AI Timeline Factor: {:.2}x (AI acceleration penalizes this static asset)", ai_timeline_factor)
    } else if ai_timeline_factor < 5.0 {
        format!("AI Timeline Factor: {:.2}x (Limited AI acceleration benefits)", ai_timeline_factor)
    } else {
        format!("AI Timeline Factor: {:.2}x (AI acceleration boosts this asset significantly)", ai_timeline_factor)
    };
    
    let type_desc = match token_type {
        "HardMoney" => "Hard Money (Store of Value): becomes the new reserve currency",
        "WrappedBridge" => "Wrapped/Bridged Asset: maintains utility but with bridging risks",
        "CommodityBacked" => "Commodity-Backed: physical asset derivative",
        "RealYield" => "Real Yield Asset: generates actual yields and value capture",
        "StableYield" => "Stable Yield: attempts to maintain value with yield",
        "LiquidStaking" => "Liquid Staking: provides staking rewards with liquidity",
        "ProtocolUtility" => "Protocol Utility Token: benefits from network effects and protocol utility",
        "Governance" => "Governance Token: value derived from protocol control",
        "Meme" => "Meme Token: speculative value with high volatility",
        _ => "Token with mixed utility characteristics",
    };
    
    let risk_desc = match risk_class {
        "Class A (Real Yield)" => "Class A (Real Yield) receives 1.2x risk class boost - resilient during market stress",
        "Class B (Systemic)" => "Class B (Systemic) receives standard treatment",
        "Class C (Venture Risk)" => "Class C (Venture Risk) receives 0.7x penalty - speculative with high uncertainty",
        "Class C (Speculative)" => "Class C (Speculative) receives 0.8x penalty - higher risk profile",
        "Class D (Memetic)" => "Class D (Memetic) receives 0.6x penalty - purely speculative",
        "Class D (Speculative)" => "Class D (Speculative) receives 0.5x penalty - highest risk tier",
        "Class E (Experimental)" => "Class E (Experimental) receives 0.5x penalty - experimental with extreme risk",
        _ => "Standard risk treatment applies",
    };
    
    let signal_desc = match signal {
        "BUY" => "Undervalued post-collapse. Accumulate before systemic shift.",
        "SELL" => "Overvalued or high risk post-collapse. Reduce exposure.",
        "HOLD" => "Moderate value post-collapse. Maintain current position.",
        "AVOID" => "Near-worthless post-collapse. Do not hold.",
        _ => "Monitor for changes.",
    };
    
    let liquidity_tariff_penalty = if symbol == "SOL" {
        0.0
    } else {
        if market_cap >= 1_000_000_000.0 { 10.0 }
        else if market_cap >= 100_000_000.0 { 15.0 }
        else if market_cap >= 10_000_000.0 { 20.0 }
        else { 25.0 }
    };

    let liquidity_desc = if symbol == "SOL" {
        "Exempt from liquidity tariff (Solana baseline).".to_string()
    } else {
        format!("Liquidity friction (Market Cap: ${}) adds a {:.0}% baseline tariff.",
            format_sci(market_cap),
            liquidity_tariff_penalty)
    };
    
    format!("[AI Timeline: Creative Renaissance] {}. {}. {} {} Insider control ({}): reduces multiplier due to centralization risks. Rank #{} gives {:.1}x adjustment (capital flight risk). {} Final real valuation multiplier: {:.1}x. Token trading at $1.00 today would be worth ${:.2} post-fiat collapse. Trading signal ({SIGNAL}): {SIGNAL_DESC}.",
        ai_desc,
        ai_impact,
        type_desc,
        risk_desc,
        insider_score,
        rank,
        if rank < 10 { 1.2 } else if rank < 100 { 0.8 } else { 0.2 },
        liquidity_desc,
        real_multiplier,
        projected_price,
        SIGNAL = signal,
        SIGNAL_DESC = signal_desc
    )
}

pub fn calculate_tariff(multiplier: f64, symbol: &str, market_cap: f64) -> f64 {
    calculate_effective_tariff(multiplier, symbol, market_cap)
}

pub fn generate_tariffs_report(analyses: &[TokenAnalysisResult]) -> String {
    let mut report = String::new();
    
    report.push_str("# Comprehensive Tariff Table\n\n");
    report.push_str("## Overview\n\n");
    report.push_str("This document provides a complete reference of all tokens in the Predictive Reasoning Pricer system along with their **effective tariff rates**. The tariff rate externalizes the risk of each asset by showing how much friction is applied when exchanging into higher-quality assets.\n\n");
    report.push_str("### Tariff Philosophy\n\n");
    report.push_str("The tariff system is designed to be intuitive:\n");
    report.push_str("- **Higher multiplier** = **Lower tariff** (better asset, less friction)\n");
    report.push_str("- **Lower multiplier** = **Higher tariff** (worse asset, more friction)\n");
    report.push_str("- **Minimum tariff is 0%** - no negative tariffs allowed\n\n");
    report.push_str("### Calculation Formula\n\n");
    report.push_str("The **effective tariff rate** is calculated from the real valuation multiplier:\n\n");
    report.push_str("```\nEffective Tariff Rate = max(0, (100 / Real Valuation Multiplier) - 10)\n```\n\n");
    report.push_str("The `max(0, ...)` ensures no negative tariffs - assets can only have 0% or higher tariffs.\n\n");
    report.push_str("This means a 10x multiplier asset has 0% tariff (the baseline). Each 10x reduction in multiplier adds approximately 1000% to the tariff.\n\n");
    report.push_str("### AI Timeline Impact\n\n");
    report.push_str("The pricing model now incorporates AI acceleration based on the 2027 timeline:\n\n");
    report.push_str("| Phase | Year | Static Assets | AI-Native Assets |\n");
    report.push_str("|-------|------|---------------|------------------|\n");
    report.push_str("| Pre-AI Baseline | 2024 | 25.0x | 3.0x |\n");
    report.push_str("| Personal Architect | 2025 Q2 | 15.0x | 8.0x |\n");
    report.push_str("| Global Acceleration Accord | 2025 Q4 | 10.0x | 15.0x |\n");
    report.push_str("| **Creative Renaissance** | **2026** | **5.0x** | **25.0x** |\n");
    report.push_str("| Agent-4 (Great Aligner) | 2027 | 2.0x | 50.0x |\n\n");
    report.push_str("Static assets (BTC, gold) decline as AI progresses, while AI-native/AI-evolving assets appreciate.\n\n");
    report.push_str("### Tariff Tier Definitions\n\n");
    report.push_str("| Tier | Tariff Rate | Multiplier Range | Description |\n");
    report.push_str("|------|-------------|------------------|-------------|\n");
    report.push_str("| **Premium** | 0% | ≥10.00x | Gold standard, reserve currencies |\n");
    report.push_str("| **Good** | 1% - 100% | 5.00x - 9.99x | Strong assets, commodity-backed |\n");
    report.push_str("| **Neutral** | 101% - 500% | 0.20x - 4.99x | Moderate utility, stable value |\n");
    report.push_str("| **Discounted** | 501% - 1000% | 0.10x - 0.19x | Underperforms baseline |\n");
    report.push_str("| **Poor** | 1001% - 5000% | 0.02x - 0.09x | Weak utility, high risk |\n");
    report.push_str("| **Catastrophic** | >5000% | <0.02x | Collapses with fiat, dead assets |\n\n");
    
    // Sort analyses by multiplier descending
    let mut sorted: Vec<_> = analyses.iter().collect();
    // Sort by effective tariff rate (ascending)
    sorted.sort_by(|a, b| {
        let tariff_a = calculate_tariff(a.real_valuation_multiplier, &a.symbol, a.market_cap);
        let tariff_b = calculate_tariff(b.real_valuation_multiplier, &b.symbol, b.market_cap);
        
        tariff_a.partial_cmp(&tariff_b).unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.real_valuation_multiplier.partial_cmp(&a.real_valuation_multiplier).unwrap_or(std::cmp::Ordering::Equal))
    });
    
    // Group by tier
    let premium: Vec<_> = sorted.iter().filter(|a| a.real_valuation_multiplier >= 10.0).collect();
    let good: Vec<_> = sorted.iter().filter(|a| a.real_valuation_multiplier >= 5.0 && a.real_valuation_multiplier < 10.0).collect();
    let neutral: Vec<_> = sorted.iter().filter(|a| a.real_valuation_multiplier >= 0.20 && a.real_valuation_multiplier < 5.0).collect();
    let discounted: Vec<_> = sorted.iter().filter(|a| a.real_valuation_multiplier >= 0.10 && a.real_valuation_multiplier < 0.20).collect();
    let poor: Vec<_> = sorted.iter().filter(|a| a.real_valuation_multiplier >= 0.02 && a.real_valuation_multiplier < 0.10).collect();
    let catastrophic: Vec<_> = sorted.iter().filter(|a| a.real_valuation_multiplier < 0.02).collect();
    
    report.push_str("---\n\n");
    report.push_str("## Complete Tariff Table\n\n");
    
    // Tier 1: Premium
    report.push_str("### Tier 1: Premium (0%)\n\n");
    report.push_str("The gold standard assets that achieve 10x or higher multipliers. These have no tariff friction.\n\n");
    report.push_str("| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |\n");
    report.push_str("|--------|------|------------|------------|------------------|---------------|------------|-------------|
");
    for a in &premium {
        let tariff = calculate_tariff(a.real_valuation_multiplier, &a.symbol, a.market_cap);
        let exchange_rate_display = if a.current_price > 0.0 {
            format!("{} ${} → ${}", format_trend(a.real_valuation_multiplier), format_smart_price(a.current_price), format_smart_price(a.projected_price))
        } else {
            format!("{} $1.00 → ${:.2}", format_trend(a.real_valuation_multiplier), a.real_valuation_multiplier)
        };

        report.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            a.symbol, a.name, format_risk_class(&a.risk_class), format_multiplier(a.real_valuation_multiplier), format_tariff_rate(tariff),
            exchange_rate_display, format_token_type(&a.token_type), format_ai_category(&a.ai_category)));
    }
    
    // Tier 2: Good
    report.push_str("\n### Tier 2: Good (1% - 100%)\n\n");
    report.push_str("Strong assets with 5x-9.99x multipliers. Low friction, good store of value.\n\n");
    report.push_str("| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |\n");
    report.push_str("|--------|------|------------|------------|------------------|---------------|------------|-------------|\n");
    for a in &good {
        let tariff = calculate_tariff(a.real_valuation_multiplier, &a.symbol, a.market_cap);
        let exchange_rate_display = if a.current_price > 0.0 {
            format!("{} ${} → ${}", format_trend(a.real_valuation_multiplier), format_smart_price(a.current_price), format_smart_price(a.projected_price))
        } else {
            format!("{} $1.00 → ${:.2}", format_trend(a.real_valuation_multiplier), a.real_valuation_multiplier)
        };

        report.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            a.symbol, a.name, format_risk_class(&a.risk_class), format_multiplier(a.real_valuation_multiplier), format_tariff_rate(tariff),
            exchange_rate_display, format_token_type(&a.token_type), format_ai_category(&a.ai_category)));
    }
    
    // Tier 3: Neutral
    report.push_str("\n### Tier 3: Neutral (101% - 500%)\n\n");
    report.push_str("Assets with 0.20x-4.99x multipliers. Moderate friction, reasonable utility.\n\n");
    report.push_str("| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |\n");
    report.push_str("|--------|------|------------|------------|------------------|---------------|------------|-------------|\n");
    for a in &neutral {
        let tariff = calculate_tariff(a.real_valuation_multiplier, &a.symbol, a.market_cap);
        let exchange_rate_display = if a.current_price > 0.0 {
            format!("{} ${} → ${}", format_trend(a.real_valuation_multiplier), format_smart_price(a.current_price), format_smart_price(a.projected_price))
        } else {
            format!("{} $1.00 → ${:.2}", format_trend(a.real_valuation_multiplier), a.real_valuation_multiplier)
        };

        report.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            a.symbol, a.name, format_risk_class(&a.risk_class), format_multiplier(a.real_valuation_multiplier), format_tariff_rate(tariff),
            exchange_rate_display, format_token_type(&a.token_type), format_ai_category(&a.ai_category)));
    }
    
    // Tier 4: Discounted
    report.push_str("\n### Tier 4: Discounted (501% - 1000%)\n\n");
    report.push_str("Assets with 0.10x-0.19x multipliers. Significant friction, underperforms baseline.\n\n");
    report.push_str("| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |\n");
    report.push_str("|--------|------|------------|------------|------------------|---------------|------------|-------------|\n");
    for a in &discounted {
        let tariff = calculate_tariff(a.real_valuation_multiplier, &a.symbol, a.market_cap);
        let exchange_rate_display = if a.current_price > 0.0 {
            format!("{} ${} → ${}", format_trend(a.real_valuation_multiplier), format_smart_price(a.current_price), format_smart_price(a.projected_price))
        } else {
            format!("{} $1.00 → ${:.2}", format_trend(a.real_valuation_multiplier), a.real_valuation_multiplier)
        };

        report.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            a.symbol, a.name, format_risk_class(&a.risk_class), format_multiplier(a.real_valuation_multiplier), format_tariff_rate(tariff),
            exchange_rate_display, format_token_type(&a.token_type), format_ai_category(&a.ai_category)));
    }
    
    // Tier 5: Poor
    report.push_str("\n### Tier 5: Poor (1001% - 5000%)\n\n");
    report.push_str("Assets with 0.02x-0.09x multipliers. High friction, weak utility.\n\n");
    report.push_str("| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |\n");
    report.push_str("|--------|------|------------|------------|------------------|---------------|------------|-------------|\n");
    for a in &poor {
        let tariff = calculate_tariff(a.real_valuation_multiplier, &a.symbol, a.market_cap);
        let exchange_rate_display = if a.current_price > 0.0 {
            format!("{} ${} → ${}", format_trend(a.real_valuation_multiplier), format_smart_price(a.current_price), format_smart_price(a.projected_price))
        } else {
            format!("{} $1.00 → ${:.2}", format_trend(a.real_valuation_multiplier), a.real_valuation_multiplier)
        };

        report.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            a.symbol, a.name, format_risk_class(&a.risk_class), format_multiplier(a.real_valuation_multiplier), format_tariff_rate(tariff),
            exchange_rate_display, format_token_type(&a.token_type), format_ai_category(&a.ai_category)));
    }
    
    // Tier 6: Catastrophic
    report.push_str("\n### Tier 6: Catastrophic (>5000%)\n\n");
    report.push_str("Assets with multipliers below 0.02x. Maximum friction, near-worthless in collapse.\n\n");
    report.push_str("| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |\n");
    report.push_str("|--------|------|------------|------------|------------------|---------------|------------|-------------|\n");
    for a in &catastrophic {
        let tariff = calculate_tariff(a.real_valuation_multiplier, &a.symbol, a.market_cap);
        let exchange_rate_display = if a.current_price > 0.0 {
            format!("{} ${} → ${}", format_trend(a.real_valuation_multiplier), format_smart_price(a.current_price), format_smart_price(a.projected_price))
        } else {
            format!("{} $1.00 → ${:.2}", format_trend(a.real_valuation_multiplier), a.real_valuation_multiplier)
        };

        report.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            a.symbol, a.name, format_risk_class(&a.risk_class), format_multiplier(a.real_valuation_multiplier), format_tariff_rate(tariff),
            exchange_rate_display, format_token_type(&a.token_type), format_ai_category(&a.ai_category)));
    }
    
    // Summary Statistics
    let total = analyses.len() as f64;
    let premium_count = premium.len();
    let good_count = good.len();
    let neutral_count = neutral.len();
    let discounted_count = discounted.len();
    let poor_count = poor.len();
    let catastrophic_count = catastrophic.len();
    
    let avg_tariff: f64 = analyses.iter().map(|a| calculate_tariff(a.real_valuation_multiplier, &a.symbol, a.market_cap)).sum::<f64>() / total;
    let tariffs: Vec<f64> = analyses.iter().map(|a| calculate_tariff(a.real_valuation_multiplier, &a.symbol, a.market_cap)).collect();
    let min_tariff = tariffs.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_tariff = tariffs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let mut sorted_tariffs = tariffs;
    sorted_tariffs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_tariff = sorted_tariffs[sorted_tariffs.len() / 2];
    
    report.push_str("---\n\n");
    report.push_str("## Summary Statistics\n\n");
    report.push_str("### Tariff Distribution by Tier\n\n");
    report.push_str("| Tier | Count | Percentage |\n");
    report.push_str("|------|-------|------------|\n");
    report.push_str(&format!("| Premium (0%) | {} | {:.1}% |\n", premium_count, premium_count as f64 / total * 100.0));
    report.push_str(&format!("| Good (1%-100%) | {} | {:.1}% |\n", good_count, good_count as f64 / total * 100.0));
    report.push_str(&format!("| Neutral (101%-500%) | {} | {:.1}% |\n", neutral_count, neutral_count as f64 / total * 100.0));
    report.push_str(&format!("| Discounted (501%-1000%) | {} | {:.1}% |\n", discounted_count, discounted_count as f64 / total * 100.0));
    report.push_str(&format!("| Poor (1001%-5000%) | {} | {:.1}% |\n", poor_count, poor_count as f64 / total * 100.0));
    report.push_str(&format!("| Catastrophic (>5000%) | {} | {:.1}% |\n", catastrophic_count, catastrophic_count as f64 / total * 100.0));
    
    report.push_str("\n### Overall Statistics\n\n");
    report.push_str("| Metric | Value |\n");
    report.push_str("|--------|-------|\n");
    report.push_str(&format!("| Total Tokens | {} |\n", analyses.len()));
    report.push_str(&format!("| Minimum Tariff | {:.0}% |\n", min_tariff));
    report.push_str(&format!("| Maximum Tariff | {:.0}% |\n", max_tariff));
    report.push_str(&format!("| Median Tariff | {:.0}% |\n", median_tariff));
    report.push_str(&format!("| Average Tariff | {:.0}% |\n", avg_tariff));
    
    // AI Impact Summary
    let ai_static: Vec<_> = analyses.iter().filter(|a| a.ai_category == "Static").collect();
    let ai_native: Vec<_> = analyses.iter().filter(|a| a.ai_category == "AI-Native" || a.ai_category == "AI-Enabled").collect();
    
    let avg_static: f64 = if !ai_static.is_empty() { ai_static.iter().map(|a| a.real_valuation_multiplier).sum::<f64>() / ai_static.len() as f64 } else { 0.0 };
    let avg_ai: f64 = if !ai_native.is_empty() { ai_native.iter().map(|a| a.real_valuation_multiplier).sum::<f64>() / ai_native.len() as f64 } else { 0.0 };
    
    report.push_str("\n### AI Timeline Impact Summary\n\n");
    report.push_str("| Category | Count | Average Multiplier |\n");
    report.push_str("|----------|-------|-------------------|\n");
    report.push_str(&format!("| Static Assets | {} | {:.2}x |\n", ai_static.len(), avg_static));
    report.push_str(&format!("| AI-Native/AI-Enabled | {} | {:.2}x |\n", ai_native.len(), avg_ai));
    if avg_static > 0.0 {
        report.push_str(&format!("| Multiplier Ratio (AI/Static) | - | {:.1}x |\n", avg_ai / avg_static));
    }
    
    report.push_str("\n---\n\n");
    report.push_str("## Exchange Rate Calculation\n\n");
    report.push_str("To calculate the post-fiat collapse exchange rate between any two tokens:\n\n");
    report.push_str("```\nExchange Rate = Token_A_Multiplier / Token_B_Multiplier\n```\n\n");
    report.push_str("Example: Converting USDC to tBTC\n");
    report.push_str("- USDC Multiplier: 0.01x\n");
    report.push_str("- tBTC Multiplier: 25.00x\n");
    report.push_str("- Exchange Rate: 0.01 / 25.00 = 0.0004\n");
    report.push_str("- 10,000 USDC → 4 tBTC\n\n");
    
    report.push_str("---\n\n");
    report.push_str("## Navigation\n\n");
    report.push_str("- [Main Report Index](./index.md)\n");
    report.push_str("- [Token Index](./tokens/index.md)\n");
    
    report
}


pub fn generate_token_prices_report(analyses: &[TokenAnalysisResult]) -> String {
    let mut report = String::new();
    
    report.push_str("# Token Price Report (2026)\n\n");
    report.push_str("## Overview\n\n");
    report.push_str("This report provides the **Current Price**, the **Exchange Price** (Immediate Purchasing Power after tariff friction), the **Multiplier** (the real valuation factor), and the **Tariff Rate** (the risk-based friction).\n\n");
    report.push_str("Calculations are based on the AI-Acceleration Pricing model defined in `tariffs.md`.\n\n");
    report.push_str("**Pricing Formula:** `Exchange Price = Current / (1 + Tariff/100)`\n\n");
    report.push_str("All numerical values are shown in **scientific notation** ($X.YeZ$) for precision across all scales.\n\n");
    report.push_str("## Price Table\n\n");
    report.push_str("| Symbol | Name | AI Category | Current Price ($) | Exchange Price ($) | Multiplier | Tariff Rate | Market Cap ($) |\n");
    report.push_str("|--------|------|-------------|-------------------|--------------------|------------|-------------|----------------|\n");

    let mut tokens_with_prices: Vec<_> = analyses.iter()
        .filter(|a| a.current_price > 0.0)
        .collect();

    // Sort by effective tariff rate (ascending)
    tokens_with_prices.sort_by(|a, b| {
        let tariff_a = calculate_tariff(a.real_valuation_multiplier, &a.symbol, a.market_cap);
        let tariff_b = calculate_tariff(b.real_valuation_multiplier, &b.symbol, b.market_cap);
        
        tariff_a.partial_cmp(&tariff_b).unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.real_valuation_multiplier.partial_cmp(&a.real_valuation_multiplier).unwrap_or(std::cmp::Ordering::Equal))
    });

    for t in tokens_with_prices {
        let tariff = calculate_tariff(t.real_valuation_multiplier, &t.symbol, t.market_cap);
        
        // Exchange Price is current price adjusted for tariff friction
        let exchange_price = t.current_price / (1.0 + tariff / 100.0);
        
        let cur = format_smart_price(t.current_price);
        let ex = format_smart_price(exchange_price);
        let mult = format_multiplier(t.real_valuation_multiplier);
        let tariff_display = format_tariff_rate(tariff);
        let mcap = format_compact_val(t.market_cap);
        
        // Format AI Category with emoji
        let ai_cat_display = format_ai_category(&t.ai_category);
        
        report.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            t.symbol, t.name, ai_cat_display, cur, ex, mult, tariff_display, mcap));
    }
    
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ai_category_detection() {
        let ai_tags = vec!["ai".to_string(), "agent".to_string(), "dao".to_string()];
        let cat = AIEvolutionCategory::from_tags(&ai_tags, None);
        assert!(matches!(cat, AIEvolutionCategory::AINative));
        
        let static_tags = vec!["hard-money".to_string(), "pow".to_string()];
        let cat = AIEvolutionCategory::from_tags(&static_tags, None);
        assert!(matches!(cat, AIEvolutionCategory::Static));
        
        let gpu_tags = vec!["gpu-compute".to_string(), "depin".to_string()];
        let cat = AIEvolutionCategory::from_tags(&gpu_tags, None);
        assert!(matches!(cat, AIEvolutionCategory::AIEnabled));
    }
    
    #[test]
    fn test_tariff_calculation() {
        // SOL is exempt from penalty
        assert_eq!(calculate_tariff(10.0, "SOL", 100_000_000_000.0), 0.0);
        
        // Other tokens start at 10% (if multiplier is 10.0)
        assert_eq!(calculate_tariff(10.0, "BTC", 1_000_000_000.0), 10.0);
        
        // Small market cap tokens have higher baseline
        assert_eq!(calculate_tariff(10.0, "HXRO", 5_000_000.0), 25.0);
    }
}
