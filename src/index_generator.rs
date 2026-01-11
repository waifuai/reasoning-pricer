//! Index generator for creating index files.

use crate::models::{Token, TokenAnalysis};
use crate::utils::get_risk_class_filename;
use std::fs;
use std::path::Path;

/// Generate main index file.
pub fn generate_main_index(analysis: &TokenAnalysis, output_dir: &Path) -> Result<String, std::io::Error> {
    // Ensure output directory exists
    fs::create_dir_all(output_dir)?;

    let index_path = output_dir.join("index.md");

    let total_tokens = analysis.total_tokens;
    let class_counts = [
        ("Class A (Real Yield)", analysis.class_a_tokens().len()),
        ("Class B (Systemic)", analysis.class_b_tokens().len()),
        ("Class C (Venture Risk)", analysis.class_c_tokens().len()),
        ("Class D (Speculative)", analysis.class_d_tokens().len()),
    ];

    let avg_insider: f64 = analysis.tokens.iter().map(|t| t.insider_score as f64).sum::<f64>() / total_tokens as f64;
    let avg_tariff: f64 = analysis.tokens.iter().map(|t| t.tariff_override as f64).sum::<f64>() / total_tokens as f64;

    let risk_class_sections: Vec<String> = class_counts
        .iter()
        .filter(|(_, count)| *count > 0)
        .map(|(risk_class, count)| {
            format!("- [{}](./summaries/{}) - {} tokens", risk_class, get_risk_class_filename(risk_class), count)
        })
        .collect();

    // Calculate AI-acceleration statistics
    let ai_native_count = analysis.tokens.iter().filter(|t| {
        t.tags.iter().any(|tag| {
            let tag_lower = tag.to_lowercase();
            tag_lower.contains("ai") || tag_lower.contains("agent") || tag_lower.contains("dao") || 
            tag_lower.contains("gpu-compute") || tag_lower.contains("depin")
        })
    }).count();
    
    let static_count = analysis.tokens.iter().filter(|t| {
        t.tags.iter().any(|tag| {
            let tag_lower = tag.to_lowercase();
            tag_lower.contains("hard-money") || tag_lower.contains("pow") || tag_lower.contains("gold")
        })
    }).count();

    let content = format!(r#"# Solana Token Analysis Reports

## Overview

This repository contains comprehensive analysis reports for {total_tokens} Solana tokens, organized by risk classification and individual token details.

## 📊 Risk Class Summary

| Risk Class | Token Count | Description |
|------------|-------------|-------------|
| [**Class A (Real Yield)**](./summaries/class-a-real-yield.md) | {class_a_count} | Low-risk foundation assets |
| [**Class B (Systemic)**](./summaries/class-b-systemic.md) | {class_b_count} | Medium-risk ecosystem tokens |
| [**Class C (Venture Risk)**](./summaries/class-c-venture-risk.md) | {class_c_count} | Higher-risk growth opportunities |
| [**Class D (Speculative)**](./summaries/class-d-speculative.md) | {class_d_count} | Maximum-risk speculative assets |

## 📈 Quick Stats

- **Total Tokens Analyzed:** {total_tokens}
- **Data Source:** JSON files in `data/` directory
- **Average Insider Score:** {avg_insider:.1}/100
- **Average Tariff:** {avg_tariff:.1}%
- **AI-Native/Enabled Tokens:** {ai_native_count}
- **Static/Hard-Money Tokens:** {static_count}

## 📁 Report Structure

### By Risk Class
{risk_classes}

### AI-Acceleration Pricing
- [📊 Tariffs Report](./tariffs.md) - AI timeline impact analysis and exchange rates

### Individual Token Reports
- [📋 All Token Reports](./tokens/index.md)

## 🔍 How to Use These Reports

1. **Start with Risk Class Summaries** - Understand the landscape by reviewing each risk class
2. **Review AI-Acceleration Tariffs** - Understand how AI acceleration impacts asset valuations
3. **Drill Down to Individual Tokens** - Click through to detailed token analysis
4. **Use Navigation Links** - Jump between related reports using the navigation sections
5. **Compare Exchange Rates** - Review tariff impacts and exchange multipliers

## 📊 Methodology

### Risk Classification
Tokens are classified into four risk classes based on:
- **Insider Score** (0-100): Degree of insider control
- **Economic Utility**: Real value accrual mechanisms
- **Regulatory Risk**: Compliance and legal considerations
- **Operational Risk**: Technical and execution risks

### AI-Acceleration Pricing Model
As AI progresses rapidly (2025-2027), asset valuations shift based on adaptability:

| Asset Type | AI Timeline Impact | Description |
|------------|-------------------|-------------|
| **Static (BTC, Gold)** | -92% by 2027 | Cannot evolve with AI; becomes obsolete |
| **AI-Native/Enabled** | +1500% by 2027 | Can rapidly evolve using AI programming |

**Timeline Phases:**
- **Q2 2025:** Personal Architect - AI tokens start appreciating
- **Q4 2025:** Global Acceleration Accord - Cross-over point (AI > Static)
- **2026:** Creative Renaissance - AI-native assets surge
- **2027:** Agent-4 (Great Aligner) - Peak AI advantage

### Exchange Rate Calculation
```
Exchange Rate = 1 + (Tariff Override / 100)
Effective Tariff = max(0, (100 / Real Valuation Multiplier) - 10)
```

### Example: 5% tariff = 1.05x exchange rate

---

## 🔗 Quick Links

- [📊 Class A (Real Yield)](./summaries/class-a-real-yield.md)
- [📊 Class B (Systemic)](./summaries/class-b-systemic.md)
- [📊 Class C (Venture Risk)](./summaries/class-c-venture-risk.md)
- [📊 Class D (Speculative)](./summaries/class-d-speculative.md)
- [📊 AI-Acceleration Tariffs](./tariffs.md)
- [📋 All Tokens](./tokens/index.md)

*For questions about methodology or data sources, see the main project documentation.*
"#,
        total_tokens = total_tokens,
        class_a_count = class_counts[0].1,
        class_b_count = class_counts[1].1,
        class_c_count = class_counts[2].1,
        class_d_count = class_counts[3].1,
        avg_insider = avg_insider,
        avg_tariff = avg_tariff,
        ai_native_count = ai_native_count,
        static_count = static_count,
        risk_classes = if risk_class_sections.is_empty() {
            "- (No tokens loaded)".to_string()
        } else {
            risk_class_sections.join("\n")
        }
    );

    fs::write(&index_path, content)?;

    Ok("index.md".to_string())
}

/// Generate index of all token reports.
pub fn generate_tokens_index(analysis: &TokenAnalysis, tokens_dir: &Path) -> Result<String, std::io::Error> {
    // Ensure output directory exists
    fs::create_dir_all(tokens_dir)?;

    let index_path = tokens_dir.join("index.md");

    let class_sections: Vec<String> = vec![
        ("Class A (Real Yield)", analysis.class_a_tokens()),
        ("Class B (Systemic)", analysis.class_b_tokens()),
        ("Class C (Venture Risk)", analysis.class_c_tokens()),
        ("Class D (Speculative)", analysis.class_d_tokens()),
    ]
    .iter()
    .filter(|(_, tokens)| !tokens.is_empty())
    .map(|(risk_class, tokens)| {
        let mut sorted_tokens: Vec<&Token> = tokens.iter().copied().collect();
        sorted_tokens.sort_by(|a, b| a.symbol.cmp(&b.symbol));

        let token_links: Vec<String> = sorted_tokens
            .iter()
            .map(|token| {
                format!("- [{}](./{}.md) - {}", token.symbol, token.symbol.to_lowercase(), token.name)
            })
            .collect();

        format!(
            "## {} ({} tokens)\n\n{}",
            risk_class,
            tokens.len(),
            token_links.join("\n")
        )
    })
    .collect();

    let content = format!(
        r#"# All Token Reports

Complete list of all {total_tokens} token analysis reports.

{class_sections}

---

## Navigation

- [🏠 Main Index](../index.md)
- [📊 Risk Class Summaries](../summaries/)

"#,
        total_tokens = analysis.total_tokens,
        class_sections = if class_sections.is_empty() {
            "(No tokens loaded)".to_string()
        } else {
            class_sections.join("\n\n")
        }
    );

    fs::write(&index_path, content)?;

    Ok("index.md".to_string())
}

/// Generate all index files.
pub fn generate_all_indices(
    analysis: &TokenAnalysis,
    output_dir: &Path,
) -> Result<(String, String), std::io::Error> {
    println!("\n📋 Generating index files...");

    let tokens_dir = output_dir.join("tokens");

    let main_index = generate_main_index(analysis, output_dir)?;
    let tokens_index = generate_tokens_index(analysis, &tokens_dir)?;

    println!("  ✓ Main index: {}", main_index);
    println!("  ✓ Tokens index: {}", tokens_index);

    Ok((main_index, tokens_index))
}
