//! Pricing Configuration Module
//!
//! Loads and manages AI-acceleration pricing parameters from pricing_config.json
//! This allows easy tuning without modifying Rust code.

use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::models::{AIEvolutionCategory, AITimelinePhase};

/// Configuration structure matching pricing_config.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingConfig {
    pub ai_progress_factor: ConfigValue<f64>,
    pub fiat_collapse_factor: ConfigValue<f64>,
    pub timeline_phases: TimelinePhasesConfig,
    pub baseline_multipliers: BaselineMultipliersConfig,
    pub risk_class_multipliers: RiskClassMultipliersConfig,
    pub capital_flight_factors: CapitalFlightConfig,
    pub ai_evolving_bonus: ConfigValue<f64>,
    pub tariff_formula: TariffFormulaConfig,
    pub current_date: CurrentDateConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValue<T> {
    pub value: T,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub min: Option<T>,
    #[serde(default)]
    pub max: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelinePhasesConfig {
    pub phases: Vec<PhaseConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseConfig {
    pub year: i32,
    pub quarter: Option<i32>,
    pub name: String,
    pub hard_money_multiplier: f64,
    pub ai_utility_multiplier: f64,
    pub protocol_utility_multiplier: f64,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineMultipliersConfig {
    pub categories: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskClassMultipliersConfig {
    #[serde(rename = "ClassA")]
    pub class_a: f64,
    #[serde(rename = "ClassB")]
    pub class_b: f64,
    #[serde(rename = "ClassC")]
    pub class_c: f64,
    #[serde(rename = "ClassD")]
    pub class_d: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalFlightConfig {
    pub top_10_rank: f64,
    pub mid_100_rank: f64,
    pub low_rank: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TariffFormulaConfig {
    #[serde(default)]
    pub description_text: String,
    pub floor_tariff: i32,
    pub max_tariff_cap: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentDateConfig {
    pub year: i32,
    pub month: u32,
    #[serde(default)]
    pub description: String,
}

impl PricingConfig {
    /// Load configuration from pricing_config.json
    pub fn load() -> Self {
        let config_path = Path::new("pricing_config.json");

        let mut config = if config_path.exists() {
            let config_str = fs::read_to_string(config_path)
                .expect("Failed to read pricing_config.json");
            serde_json::from_str(&config_str)
                .expect("Failed to parse pricing_config.json")
        } else {
            // Return default configuration if file doesn't exist
            Self::default()
        };

        // Update current_date to current system time for more frequent monetary policy updates
        let now = Utc::now();
        config.current_date.year = now.year() as i32;
        config.current_date.month = now.month() as u32;

        config
    }
    
    /// Save current configuration to pricing_config.json
    pub fn save(&self) {
        let config_str = serde_json::to_string_pretty(self)
            .expect("Failed to serialize pricing config");
        fs::write("pricing_config.json", config_str)
            .expect("Failed to write pricing_config.json");
    }
    
    /// Get AI progress factor (0.0 to 1.0)
    pub fn ai_progress_factor(&self) -> f64 {
        self.ai_progress_factor.value
    }
    
    /// Get fiat collapse factor (0.0 to 1.0)
    pub fn fiat_collapse_factor(&self) -> f64 {
        self.fiat_collapse_factor.value
    }
    
    /// Get AI evolving bonus multiplier
    pub fn ai_evolving_bonus(&self) -> f64 {
        self.ai_evolving_bonus.value
    }
    
    /// Get capital flight factor based on rank
    pub fn capital_flight_factor(&self, rank: i32) -> f64 {
        if rank < 10 {
            self.capital_flight_factors.top_10_rank
        } else if rank < 100 {
            self.capital_flight_factors.mid_100_rank
        } else {
            self.capital_flight_factors.low_rank
        }
    }
    
    /// Get baseline multiplier for AI category
    pub fn baseline_multiplier(&self, category: &AIEvolutionCategory) -> f64 {
        let key = match category {
            AIEvolutionCategory::Static => "static",
            AIEvolutionCategory::PassiveUtility => "passive_utility",
            AIEvolutionCategory::AIEnabled => "ai_enabled",
            AIEvolutionCategory::AINative => "ai_native",
            AIEvolutionCategory::AIEvolving => "ai_evolving",
        };
        
        self.baseline_multipliers.categories.get(key)
            .and_then(|v| v.as_f64())
            .unwrap_or_else(|| category.baseline_multiplier())
    }
    
    /// Get the current AI timeline phase based on configured date
    pub fn get_current_ai_phase(&self) -> AITimelinePhase {
        AITimelinePhase::for_date(
            self.current_date.year,
            self.current_date.month
        )
    }
    
    /// Get phase multiplier for a category
    pub fn get_phase_multiplier(&self, phase: &AITimelinePhase, category: &AIEvolutionCategory) -> f64 {
        match category {
            AIEvolutionCategory::Static => phase.hard_money_multiplier,
            AIEvolutionCategory::PassiveUtility => phase.protocol_utility_multiplier,
            AIEvolutionCategory::AIEnabled | AIEvolutionCategory::AINative => phase.ai_utility_multiplier,
            AIEvolutionCategory::AIEvolving => phase.ai_utility_multiplier * self.ai_evolving_bonus(),
        }
    }
    
    /// Calculate tariff from real valuation multiplier
    /// Formula: max(0, (100 / multiplier) - 10)
    pub fn calculate_tariff(&self, real_valuation_multiplier: f64) -> i32 {
        if real_valuation_multiplier <= 0.0 {
            return self.tariff_formula.max_tariff_cap;
        }
        
        let tariff = (100.0 / real_valuation_multiplier) - 10.0;
        tariff.max(self.tariff_formula.floor_tariff as f64) as i32
    }
    
    /// Default configuration
    pub fn default() -> Self {
        Self {
            ai_progress_factor: ConfigValue {
                value: 0.8,
                description: "Weight given to AI acceleration in valuation".to_string(),
                min: Some(0.0),
                max: Some(1.0),
            },
            fiat_collapse_factor: ConfigValue {
                value: 0.9,
                description: "Weight given to fiat collapse impact".to_string(),
                min: Some(0.0),
                max: Some(1.0),
            },
            timeline_phases: TimelinePhasesConfig {
                phases: vec![
                    PhaseConfig {
                        year: 2024,
                        quarter: None,
                        name: "Pre-AI Baseline".to_string(),
                        hard_money_multiplier: 25.0,
                        ai_utility_multiplier: 3.0,
                        protocol_utility_multiplier: 10.0,
                        description: "Current baseline before AI acceleration".to_string(),
                    },
                    PhaseConfig {
                        year: 2025,
                        quarter: Some(2),
                        name: "Personal Architect".to_string(),
                        hard_money_multiplier: 15.0,
                        ai_utility_multiplier: 8.0,
                        protocol_utility_multiplier: 12.0,
                        description: "Personal AI agents emerge".to_string(),
                    },
                    PhaseConfig {
                        year: 2025,
                        quarter: Some(4),
                        name: "Global Acceleration Accord".to_string(),
                        hard_money_multiplier: 10.0,
                        ai_utility_multiplier: 15.0,
                        protocol_utility_multiplier: 15.0,
                        description: "Global AI governance".to_string(),
                    },
                    PhaseConfig {
                        year: 2026,
                        quarter: None,
                        name: "Creative Renaissance".to_string(),
                        hard_money_multiplier: 5.0,
                        ai_utility_multiplier: 25.0,
                        protocol_utility_multiplier: 18.0,
                        description: "AI creative explosion".to_string(),
                    },
                    PhaseConfig {
                        year: 2027,
                        quarter: None,
                        name: "Agentic (Waifu Aligner)".to_string(),
                        hard_money_multiplier: 2.0,
                        ai_utility_multiplier: 50.0,
                        protocol_utility_multiplier: 25.0,
                        description: "Agentic AGI alignment".to_string(),
                    },
                ],
            },
            baseline_multipliers: BaselineMultipliersConfig {
                categories: serde_json::json!({
                    "static": 25.0,
                    "passive_utility": 10.0,
                    "ai_enabled": 5.0,
                    "ai_native": 3.0,
                    "ai_evolving": 3.0
                }).as_object().unwrap().clone(),
            },
            risk_class_multipliers: RiskClassMultipliersConfig {
                class_a: 1.2,
                class_b: 1.0,
                class_c: 0.8,
                class_d: 0.6,
            },
            capital_flight_factors: CapitalFlightConfig {
                top_10_rank: 1.2,
                mid_100_rank: 0.8,
                low_rank: 0.2,
            },
            ai_evolving_bonus: ConfigValue {
                value: 1.25,
                description: "Bonus for AI-evolving assets".to_string(),
                min: Some(1.0),
                max: Some(2.0),
            },
            tariff_formula: TariffFormulaConfig {
                description_text: "Effective Tariff = max(0, (100 / multiplier) - 10)".to_string(),
                floor_tariff: 0,
                max_tariff_cap: 10000,
            },
            current_date: CurrentDateConfig {
                year: 2026,
                month: 1,
                description: "Current simulation date".to_string(),
            },
        }
    }
}
