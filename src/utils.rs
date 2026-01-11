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

    // Sort by frequency
    let mut sorted_tags: Vec<(&str, usize)> = tag_counts.into_iter().collect();
    sorted_tags.sort_by(|a, b| b.1.cmp(&a.1));

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

/// Format a number as a multiplier string.
pub fn format_multiplier(value: f64) -> String {
    format!("{:.2}x", value)
}
