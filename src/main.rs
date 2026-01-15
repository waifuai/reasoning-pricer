//! Main entry point for the Solana Token Analysis Report Generator.

use crate::data_loader::load_tokens;
use crate::index_generator::generate_all_indices;
use crate::models::TokenAnalysis;
use crate::summary_reporter::generate_multiple_summaries;
use crate::tariff_generator::{analyze_token, generate_tariffs_report, generate_token_prices_report, load_tokens as load_tokens_for_tariff};
use crate::token_reporter::generate_multiple_reports;
use std::path::Path;

mod data_loader;
mod index_generator;
mod models;
mod reasoning_pricer;
mod summary_reporter;
mod tariff_generator;
mod token_reporter;
mod utils;

fn main() {
    println!("🚀 Starting markdown report generation...");

    // Default paths
    let data_dir = "data";
    let output_dir = "reports";

    // Load data
    println!("Loading data...");
    let tokens = match load_tokens(data_dir) {
        Ok(tokens) => {
            println!("✅ Loaded {} tokens", tokens.len());
            tokens
        }
        Err(e) => {
            eprintln!("❌ Error loading data: {}", e);
            std::process::exit(1);
        }
    };

    // Create token analysis
    let analysis = TokenAnalysis::new(tokens);

    // Create output directories
    let output_path = Path::new(output_dir);
    let tokens_dir = output_path.join("tokens");
    let summaries_dir = output_path.join("summaries");

    // Generate individual token reports
    println!("\n📄 Generating individual token reports...");
    match generate_multiple_reports(&analysis.tokens, &tokens_dir) {
        Ok(token_files) => {
            println!("✅ Generated {} token reports", token_files.len());
        }
        Err(e) => {
            eprintln!("❌ Error generating token reports: {}", e);
            std::process::exit(1);
        }
    }

    // Generate risk class summaries
    let risk_class_data = vec![
        ("Class A (Real Yield)", analysis.class_a_tokens()),
        ("Class B (Systemic)", analysis.class_b_tokens()),
        ("Class C (Venture Risk)", analysis.class_c_tokens()),
        ("Class D (Speculative)", analysis.class_d_tokens()),
    ];

    match generate_multiple_summaries(risk_class_data, &summaries_dir) {
        Ok(summary_files) => {
            println!("✅ Generated {} risk class summaries", summary_files.len());
        }
        Err(e) => {
            eprintln!("❌ Error generating summaries: {}", e);
            std::process::exit(1);
        }
    }

    // Generate index files
    match generate_all_indices(&analysis, output_path) {
        Ok(_) => {
            println!("✅ Generated index files");
        }
        Err(e) => {
            eprintln!("❌ Error generating indices: {}", e);
            std::process::exit(1);
        }
    }

    // Generate tariffs report using new AI-acceleration pricing model
    println!("\n📊 Generating AI-acceleration tariffs report...");
    let tariff_tokens = load_tokens_for_tariff(data_dir);
    let analyses: Vec<_> = tariff_tokens.iter().map(analyze_token).collect();
    let tariffs_report = generate_tariffs_report(&analyses);
    
    let tariffs_path = output_path.join("predicted_price.md");
    match std::fs::write(&tariffs_path, &tariffs_report) {
        Ok(_) => {
            println!("✅ Generated AI-acceleration tariffs report");
        }
        Err(e) => {
            eprintln!("❌ Error writing tariffs report: {}", e);
            std::process::exit(1);
        }
    }

    // Generate token prices report
    println!("📊 Generating token price report...");
    let prices_report = generate_token_prices_report(&analyses);
    let prices_path = output_path.join("tariff_rate.md");
    match std::fs::write(&prices_path, &prices_report) {
        Ok(_) => {
            println!("✅ Generated token price report");
        }
        Err(e) => {
            eprintln!("❌ Error writing token price report: {}", e);
            std::process::exit(1);
        }
    }

    // Print summary statistics
    println!("\n📊 Summary Statistics:");
    println!("  Total tokens: {}", analysis.total_tokens);
    println!(
        "  Class A (Real Yield): {}",
        analysis.class_a_tokens().len()
    );
    println!(
        "  Class B (Systemic): {}",
        analysis.class_b_tokens().len()
    );
    println!(
        "  Class C (Venture Risk): {}",
        analysis.class_c_tokens().len()
    );
    println!(
        "  Class D (Speculative): {}",
        analysis.class_d_tokens().len()
    );

    let avg_insider: f64 = analysis.tokens.iter().map(|t| t.insider_score as f64).sum::<f64>() / analysis.total_tokens as f64;
    let avg_tariff: f64 = analysis.tokens.iter().map(|t| t.tariff_override as f64).sum::<f64>() / analysis.total_tokens as f64;
    println!("  Average insider score: {:.1}/100", avg_insider);
    println!("  Average tariff: {:.1}%", avg_tariff);

    println!("\n🎉 Report generation complete!");
    println!("📁 Reports saved to: {}", output_dir);
}
