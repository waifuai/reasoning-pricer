//! Summary report generator for risk class summaries.

use crate::models::Token;
use crate::utils::{
    calculate_average_insider_score, calculate_average_multiplier, calculate_average_tariff,
    format_multiplier, format_percentage, generate_tag_analysis, get_risk_class_description,
    get_risk_class_filename,
};
use std::fs;
use std::path::Path;

/// Generate summary report for a specific risk class.
pub fn generate_risk_class_summary(risk_class: &str, tokens: &[&Token], summaries_dir: &Path) -> Result<String, std::io::Error> {
    // Ensure output directory exists
    fs::create_dir_all(summaries_dir)?;

    let filename = get_risk_class_filename(risk_class);
    let filepath = summaries_dir.join(&filename);

    // Calculate statistics
    let avg_insider_score = calculate_average_insider_score(tokens);
    let avg_tariff = calculate_average_tariff(tokens);
    let avg_multiplier = calculate_average_multiplier(tokens);

    // Create table rows
    let mut sorted_tokens: Vec<&Token> = tokens.to_vec();
    sorted_tokens.sort_by(|a, b| a.insider_score.cmp(&b.insider_score));

    let table_rows: Vec<String> = sorted_tokens
        .iter()
        .map(|token| {
            format!(
                "| [{}](../tokens/{}.md) | {} | {} | {} | {} |",
                token.symbol,
                token.symbol.to_lowercase(),
                token.name,
                token.insider_score,
                format_percentage(token.tariff_override as f64),
                format_multiplier(token.exchange_multiplier())
            )
        })
        .collect();

    let content = format!(
        r#"# {risk_class} - Summary

## Overview

This report summarizes all tokens classified as **{risk_class}**.

### Statistics

| Metric | Value |
|--------|-------|
| **Total Tokens** | {token_count} |
| **Average Insider Score** | {avg_insider:.1}/100 |
| **Average Tariff** | {avg_tariff:.1}% |
| **Average Exchange Multiplier** | {avg_mult:.2}x |

## Token Details

| Symbol | Name | Insider Score | Tariff | Exchange Rate |
|--------|------|---------------|--------|---------------|
{table_rows}

## Risk Characteristics

{risk_description}

## Tag Analysis

{tag_analysis}

---

## Navigation

- [🏠 Main Index](../index.md)
- [📊 All Risk Classes](./)
- [🔗 Individual Token Reports](../tokens/)
"#,
        risk_class = risk_class,
        token_count = tokens.len(),
        avg_insider = avg_insider_score,
        avg_tariff = avg_tariff,
        avg_mult = avg_multiplier,
        table_rows = if table_rows.is_empty() {
            "| (No tokens in this category) | | | | |".to_string()
        } else {
            table_rows.join("\n")
        },
        risk_description = get_risk_class_description(risk_class),
        tag_analysis = generate_tag_analysis(tokens)
    );

    fs::write(&filepath, content)?;

    Ok(filename)
}

/// Generate summaries for multiple risk classes.
pub fn generate_multiple_summaries(
    risk_class_data: Vec<(&str, Vec<&Token>)>,
    summaries_dir: &Path,
) -> Result<Vec<String>, std::io::Error> {
    println!("\n📊 Generating risk class summaries...");

    let mut summary_files = Vec::new();
    for (risk_class, tokens) in risk_class_data {
        if !tokens.is_empty() {
            match generate_risk_class_summary(risk_class, &tokens, summaries_dir) {
                Ok(filename) => {
                    println!("  ✓ {} - {} tokens", risk_class, tokens.len());
                    summary_files.push(filename);
                }
                Err(e) => {
                    println!("  ✗ Failed to generate summary for {}: {}", risk_class, e);
                }
            }
        }
    }

    Ok(summary_files)
}
