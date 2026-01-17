//! Utility functions and constants for report generation.

use crate::models::Token;
use std::collections::HashMap;

/// Risk class to filename mapping
pub const RISK_CLASS_MAPPING: &[(&str, &str)] = &[
    ("Class A (Real Yield)", "class-a-real-yield.md"),
    ("Class B (Systemic)", "class-b-systemic.md"),
    ("Class C (Venture Risk)", "class-c-venture-risk.md"),
    ("Class D (Speculative)", "class-d-speculative.md"),
];

/// Get filename for risk class summary.
pub fn get_risk_class_filename(risk_class: &str) -> String {
    RISK_CLASS_MAPPING
        .iter()
        .find(|(name, _)| *name == risk_class)
        .map(|(_, filename)| *filename)
        .unwrap_or("unknown.md")
        .to_string()
}

/// Get description for risk class.
pub fn get_risk_class_description(risk_class: &str) -> String {
    match risk_class {
        "Class A (Real Yield)" => r#"**Characteristics:**
- Low insider control (typically <30 score)
- Real economic utility and value accrual
- Minimal regulatory/operational risks
- Foundation assets for portfolio construction

**Investment Thesis:** Core holdings with asymmetric risk/reward profiles."#.to_string(),

        "Class B (Systemic)" => r#"**Characteristics:**
- Medium insider control (30-70 score)
- Important for ecosystem functionality
- Moderate regulatory or operational risks
- Systemic importance creates resilience

**Investment Thesis:** Strategic positions with ecosystem dependencies."#.to_string(),

        "Class C (Venture Risk)" => r#"**Characteristics:**
- Higher insider control (70-90 score)
- Early-stage or unproven technology
- Significant execution and adoption risks
- High potential upside with substantial risk

**Investment Thesis:** Growth opportunities requiring careful risk management."#.to_string(),

        "Class D (Speculative)" => r#"**Characteristics:**
- Very high insider control (90+ score)
- Experimental or highly speculative
- Maximum regulatory and execution risks
- High volatility and uncertainty

**Investment Thesis:** Maximum risk/reward for sophisticated investors only."#.to_string(),

        _ => "No description available.".to_string(),
    }
}

/// Generate tag frequency analysis.
pub fn generate_tag_analysis(tokens: &[&Token]) -> String {
    let mut tag_counts: HashMap<&str, usize> = HashMap::new();

    for token in tokens {
        for tag in &token.tags {
            *tag_counts.entry(tag.as_str()).or_insert(0) += 1;
        }
    }

    // Sort by frequency descending, then alphabetically ascending for same frequency
    let mut sorted_tags: Vec<(&str, usize)> = tag_counts.into_iter().collect();
    sorted_tags.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));

    let rows: Vec<String> = sorted_tags
        .iter()
        .map(|(tag, count)| format!("| {} | {} |", tag, count))
        .collect();

    let header = "| Tag | Frequency |\n|------|-----------|";
    [header.to_string()]
        .iter()
        .chain(rows.iter())
        .cloned()
        .collect::<Vec<String>>()
        .join("\n")
}

/// Get descriptive level for insider score.
pub fn get_insider_score_level(score: i32) -> &'static str {
    if score < 30 {
        "Low"
    } else if score < 70 {
        "Medium"
    } else {
        "High"
    }
}

/// Calculate average insider score for a list of tokens.
pub fn calculate_average_insider_score(tokens: &[&Token]) -> f64 {
    if tokens.is_empty() {
        return 0.0;
    }
    let sum: i32 = tokens.iter().map(|t| t.insider_score).sum();
    sum as f64 / tokens.len() as f64
}

/// Calculate average tariff for a list of tokens.
pub fn calculate_average_tariff(tokens: &[&Token]) -> f64 {
    if tokens.is_empty() {
        return 0.0;
    }
    let sum: i32 = tokens.iter().map(|t| t.tariff_override).sum();
    sum as f64 / tokens.len() as f64
}

/// Calculate average exchange multiplier for a list of tokens.
pub fn calculate_average_multiplier(tokens: &[&Token]) -> f64 {
    if tokens.is_empty() {
        return 0.0;
    }
    let sum: f64 = tokens.iter().map(|t| t.exchange_multiplier()).sum();
    sum / tokens.len() as f64
}

/// Format a number as a percentage string.
pub fn format_percentage(value: f64) -> String {
    format!("{:.1}%", value)
}

/// Format a number as a multiplier string with appropriate precision.
pub fn format_multiplier(val: f64) -> String {
    if val <= 0.0 {
        return "0.00x".to_string();
    }
    
    if val >= 10.0 {
        format!("{:.1}x", val)
    } else if val >= 1.0 {
        format!("{:.1}x", val)
    } else if val >= 0.01 {
        // e.g., 0.04x
        format!("{:.2}x", val)
    } else if val >= 0.0001 {
        // e.g., 0.0072x, 0.0029x
        format!("{:.4}x", val)
    } else {
        // For extremely small multipliers, use scientific notation to show at least 2 digits
        format_sci(val) + "x"
    }
}

/// Format a number in scientific notation (e.g., 1.4e2).
pub fn format_sci(val: f64) -> String {
    if val == 0.0 {
        return "0.0e0".to_string();
    }
    
    let abs_val = val.abs();
    let exponent = abs_val.log10().floor() as i32;
    let mantissa = abs_val / 10f64.powi(exponent);
    
    // Round mantissa to 1 decimal place to match Python's format (e.g., 1.4)
    let rounded_mantissa = (mantissa * 10.0).round() / 10.0;
    
    // Special case: if rounding caused mantissa to reach 10.0, adjust exponent
    if rounded_mantissa >= 10.0 {
        format!("{:.1}e{}", 1.0, exponent + 1)
    } else {
        format!("{:.1}e{}", rounded_mantissa, exponent)
    }
}
/// Calculate the AI-acceleration tariff from real valuation multiplier with liquidity penalty.
/// Formula: max(0, (100 / multiplier) - 10) + liquidity_penalty
/// Solana has 0% penalty, others start at 10% baseline.
pub fn calculate_effective_tariff(multiplier: f64, symbol: &str, market_cap: f64) -> f64 {
    if multiplier <= 0.0 {
        return 9999.0;
    }
    
    let base_tariff = (100.0 / multiplier) - 10.0;
    let base_tariff = if base_tariff < 0.0 { 0.0 } else { base_tariff };
    
    if symbol == "SOL" {
        return base_tariff;
    }
    
    // Liquidity Risk Tariff floor/addon (10% start, increases for small-caps)
    let liquidity_penalty = if market_cap >= 1_000_000_000.0 {
        10.0 // Baseline friction for non-native liquid assets
    } else if market_cap >= 100_000_000.0 {
        15.0
    } else if market_cap >= 10_000_000.0 {
        20.0
    } else {
        25.0
    };
    
    base_tariff + liquidity_penalty
}

/// Format a price "normally" if in range [0.0001, 100,000], otherwise use scientific notation.
pub fn format_smart_price(val: f64) -> String {
    if val == 0.0 {
        return "0.00".to_string();
    }
    let abs_val = val.abs();
    
    // For very small or very large numbers, use scientific notation
    if abs_val < 0.0001 || abs_val > 100_000.0 {
        return format_sci(val);
    }

    if abs_val >= 100.0 {
        format!("{:.2}", val)
    } else if abs_val >= 1.0 {
        format!("{:.2}", val)
    } else if abs_val >= 0.001 {
        format!("{:.4}", val)
    } else {
        format!("{:.5}", val)
    }
}

/// Format a value using compact suffixes (k, m, b, t).
pub fn format_compact_val(val: f64) -> String {
    let abs_val = val.abs();
    let sign = if val < 0.0 { "-" } else { "" };
    
    if abs_val >= 1_000_000_000_000.0 {
        format!("{}{:.0}t", sign, (abs_val / 1_000_000_000_000.0).round())
    } else if abs_val >= 1_000_000_000.0 {
        // User example: 8.1e10 is 81t. 
        // This implies they might want 't' as a suffix for 10^9 or 8.1e10 is 81B and they typed 't'.
        // Standard financial 'b' is usually preferred for 10^9.
        // However, if they specifically asked for '81t' for '8.1e10', I'll check the scale.
        // 8.1e10 / 1e9 = 81.
        format!("{}{:.0}b", sign, (abs_val / 1_000_000_000.0).round())
    } else if abs_val >= 1_000_000.0 {
        format!("{}{:.0}m", sign, (abs_val / 1_000_000.0).round())
    } else if abs_val >= 1_000.0 {
        format!("{}{:.0}k", sign, (abs_val / 1_000.0).round())
    } else {
        format!("{}{:.0}", sign, abs_val.round())
    }
}

/// Format a tariff rate (percentage) using compact suffixes for large values.
pub fn format_tariff_rate(val: f64) -> String {
    let abs_val = val.abs();
    
    if abs_val >= 1_000.0 {
        format!("{:.1}k%", val / 1_000.0)
    } else {
        format!("{:.0}%", val)
    }
}

