//! Token report generator for individual token analysis reports.

use crate::reasoning_pricer::ReasoningPricer;
use crate::models::Token;
use crate::utils::{get_insider_score_level, get_risk_class_filename, format_sci, calculate_effective_tariff};
use std::fs;
use std::path::Path;

/// Format a multiplier with appropriate precision based on magnitude
fn format_multiplier(value: f64) -> String {
    if value < 0.1 {
        format!("{:.2}x", value)
    } else if value < 1.0 {
        format!("{:.1}x", value)
    } else {
        format!("{:.0}x", value)
    }
}


/// Generate individual token report with reasoning pricer analysis.
pub fn generate_token_report(token: &Token, tokens_dir: &Path) -> Result<String, std::io::Error> {
    // Ensure output directory exists
    fs::create_dir_all(tokens_dir)?;

    let filename = format!("{}.md", token.symbol.to_lowercase());
    let filepath = tokens_dir.join(&filename);

    // Generate reasoning pricer analysis
    let pricer = ReasoningPricer::new();
    let analysis = pricer.analyze_token(token);
    
    // Calculate the AI-acceleration tariff from real valuation multiplier
    let calculated_tariff = calculate_effective_tariff(analysis.real_valuation_multiplier, &token.symbol, token.market_cap);
    
    // Exchange multiplier should be based on calculated tariff (1 + tariff/100)
    let exchange_multiplier = 1.0 + (calculated_tariff / 100.0);

    let content = format!(
        r#"# {} - {}

## Overview

| Field | Value |
|-------|-------|
| **Symbol** | `{}` |
| **Name** | {} |
| **Price** | `${}` |
| **Market Cap** | `${}` |
| **Token Type** | {} |
| **Risk Class** | {} |
| **Insider Score** | {}/100 |
| **Tariff Override** | {}% |
| **Calculated Tariff** | {:.0}% |
| **Exchange Multiplier** | {:.1}x |

## Risk Analysis

### Token Type
{}

### Archetype
{}

### Insider Score
**{}/100** - {} insider control

### Exchange Rate
**{:.1}x** (Tariff: {:.0}%)

## Tags

{}

## Analyst Notes

{}

---

## Reasoning Pricer Analysis

### Post-Fiat Collapse Valuation

| Metric | Value |
|--------|-------|
| **Token Type** | {} |
| **Base Multiplier Range** | {} |
| **Type Adjustment Factor** | {} |
| **Risk Class Adjustment** | {:.1}x |
| **Insider Risk Factor** | {:.2}x |
| **AI Timeline Factor** | {:.2}x |
| **AI Category** | {} |
| **Current AI Phase** | {} |
| **Liquidity Risk Factor** | {:.2}x |
| **Real Valuation Multiplier** | {} |
| **Uncertainty Range** | {} - {} |
| **Current vs Real Price** | `${}` → `${}` |
| **Trading Signal** | {} |

### Analysis
{}

---

## Navigation

- [📊 Risk Class Summary](../summaries/{})
- [🏠 Main Index](../index.md)
- [📈 All Tokens](../tokens/index.md)
"#,
        token.symbol,
        token.name,
        token.symbol,
        token.name,
        format_sci(token.price),
        format_sci(token.market_cap),
        analysis.token_type_display,
        token.archetype.display_name(),
        token.insider_score,
        token.tariff_override,
        calculated_tariff,
        exchange_multiplier,
        analysis.token_type_display,
        token.archetype.display_name(),
        token.insider_score,
        get_insider_score_level(token.insider_score),
        exchange_multiplier,
        calculated_tariff,
        token.tags.iter().map(|t| format!("- {}", t)).collect::<Vec<_>>().join("\n"),
        token.reason,
        analysis.token_type,
        analysis.base_multiplier_range,
        format_multiplier(analysis.type_adjustment_factor),
        analysis.risk_class_adjustment,
        analysis.insider_risk_factor,
        analysis.ai_timeline_factor,
        analysis.ai_category_display,
        analysis.current_ai_phase,
        analysis.liquidity_risk_factor,
        format_multiplier(analysis.real_valuation_multiplier),
        format_multiplier(analysis.uncertainty_range.lower),
        format_multiplier(analysis.uncertainty_range.upper),
        format_sci(token.price),
        format_sci(analysis.real_valuation_multiplier * token.price),
        analysis.trading_signal,
        analysis.reasoning,
        get_risk_class_filename(token.archetype.display_name())
    );

    fs::write(&filepath, content)?;

    Ok(filename)
}

/// Generate reports for multiple tokens.
pub fn generate_multiple_reports(tokens: &[Token], tokens_dir: &Path) -> Result<Vec<String>, std::io::Error> {
    println!("📄 Generating individual token reports...");

    let mut token_files = Vec::new();
    for token in tokens {
        match generate_token_report(token, tokens_dir) {
            Ok(filename) => {
                println!("  ✓ {} - {}", token.symbol, token.name);
                token_files.push(filename);
            }
            Err(e) => {
                println!("  ✗ Failed to generate report for {}: {}", token.symbol, e);
            }
        }
    }

    Ok(token_files)
}
