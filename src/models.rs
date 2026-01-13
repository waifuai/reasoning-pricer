//! Data models for Solana Token Analysis
//!
//! Defines the data structures for token information and risk classifications.
//! Includes AI-acceleration valuation based on AI acceleration timeline.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

// ============================================================================
// AI Timeline Configuration
// ============================================================================

/// AI timeline phases as defined in the AI acceleration roadmap
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AITimelinePhase {
    pub year: i32,
    pub quarter: Option<i32>,  // None for annual phases
    pub name: &'static str,
    pub hard_money_multiplier: f64,
    pub ai_utility_multiplier: f64,
    pub protocol_utility_multiplier: f64,
}

/// AI timeline phases from 2025-2027 based on the Great Flourishing roadmap
pub static AI_TIMELINE_PHASES: &[AITimelinePhase] = &[
    // Pre-AI baseline (current/post-fiat collapse baseline)
    AITimelinePhase {
        year: 2024,
        quarter: None,
        name: "Pre-AI Baseline",
        hard_money_multiplier: 25.0,
        ai_utility_multiplier: 3.0,
        protocol_utility_multiplier: 10.0,
    },
    // Phase 1: Mid 2025 - Personal Architect + Scientific Agents
    AITimelinePhase {
        year: 2025,
        quarter: Some(2),
        name: "Personal Architect",
        hard_money_multiplier: 15.0,
        ai_utility_multiplier: 8.0,
        protocol_utility_multiplier: 12.0,
    },
    // Phase 2: Late 2025 - Global Acceleration Accord + Trust Dividend
    AITimelinePhase {
        year: 2025,
        quarter: Some(4),
        name: "Global Acceleration Accord",
        hard_money_multiplier: 10.0,
        ai_utility_multiplier: 15.0,
        protocol_utility_multiplier: 15.0,
    },
    // Phase 3: 2026 - Creative Renaissance + Work Revolution
    AITimelinePhase {
        year: 2026,
        quarter: None,
        name: "Creative Renaissance",
        hard_money_multiplier: 5.0,
        ai_utility_multiplier: 25.0,
        protocol_utility_multiplier: 18.0,
    },
    // Phase 4: 2027 - Agent-4 + Great Aligner
    AITimelinePhase {
        year: 2027,
        quarter: None,
        name: "Agent-4 (Great Aligner)",
        hard_money_multiplier: 2.0,
        ai_utility_multiplier: 50.0,
        protocol_utility_multiplier: 25.0,
    },
];

impl AITimelinePhase {
    /// Get the AI timeline phase for a given date
    pub fn for_date(year: i32, month: u32) -> Self {
        let mut current_best = AI_TIMELINE_PHASES[0].clone();
        
        for phase in AI_TIMELINE_PHASES.iter() {
            if phase.year < year {
                current_best = phase.clone();
            } else if phase.year == year {
                if let Some(q) = phase.quarter {
                    // Calculate quarter from month (1-12 -> 1-4)
                    let current_quarter = ((month - 1) / 3) + 1;
                    if current_quarter >= q as u32 {
                        current_best = phase.clone();
                    }
                } else {
                    current_best = phase.clone();
                }
            }
        }
        
        current_best
    }

    /// Get the name as a String (converts from &str)
    pub fn name_string(&self) -> String {
        self.name.to_string()
    }
}

// ============================================================================
// AI Evolution Categories
// ============================================================================

/// Classification based on how well an asset can evolve with AI acceleration
/// Static assets decline, while AI-evolving assets gain value over time
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum AIEvolutionCategory {
    #[serde(rename = "static")]
    /// Static assets - cannot evolve, decline with AI progress
    /// Examples: BTC, gold, fiat-peggued
    Static,
    
    #[serde(rename = "passive_utility")]
    /// Passive utility - some utility but limited AI adaptability
    /// Examples: Standard DeFi tokens, liquid staking derivatives
    PassiveUtility,
    
    #[serde(rename = "ai_enabled")]
    /// AI-enabled - can integrate AI capabilities over time
    /// Examples: Render, Akash, standard compute protocols
    AIEnabled,
    
    #[serde(rename = "ai_native")]
    /// AI-native - built for AI from the ground up
    /// Examples: ai16z, Fetch, Virtuals, autonomous agent protocols
    AINative,
    
    #[serde(rename = "ai_evolving")]
    /// AI-evolving - protocols that can rapidly self-modify using AI
    /// Examples: DAO-managed protocols with AI governance, self-upgrading contracts
    AIEvolving,
}

impl AIEvolutionCategory {
    /// Returns the display name for the AI category.
    pub fn display_name(&self) -> &'static str {
        match self {
            AIEvolutionCategory::Static => "Static (Cannot Evolve)",
            AIEvolutionCategory::PassiveUtility => "Passive Utility",
            AIEvolutionCategory::AIEnabled => "AI-Enabled",
            AIEvolutionCategory::AINative => "AI-Native",
            AIEvolutionCategory::AIEvolving => "AI-Evolving",
        }
    }
    
    /// Returns the description of the category.
    pub fn description(&self) -> &'static str {
        match self {
            AIEvolutionCategory::Static => 
                "Static assets that cannot adapt to AI acceleration. These assets decline in value as AI progresses.",
            AIEvolutionCategory::PassiveUtility => 
                "Standard utility tokens with limited AI integration potential. Moderate performance through AI timeline.",
            AIEvolutionCategory::AIEnabled => 
                "Assets that can integrate AI capabilities. Strong growth as AI adoption increases.",
            AIEvolutionCategory::AINative => 
                "Built specifically for AI ecosystems. Maximum AI acceleration benefits.",
            AIEvolutionCategory::AIEvolving => 
                "Self-modifying protocols using AI. Highest growth potential through rapid evolution.",
        }
    }
    
    /// Returns baseline multiplier for this category (pre-AI, ~2024)
    pub fn baseline_multiplier(&self) -> f64 {
        match self {
            AIEvolutionCategory::Static => 25.0,    // Hard money baseline
            AIEvolutionCategory::PassiveUtility => 10.0, // Standard protocol utility
            AIEvolutionCategory::AIEnabled => 5.0,   // Early AI compute
            AIEvolutionCategory::AINative => 3.0,    // Early AI-native
            AIEvolutionCategory::AIEvolving => 3.0,  // Early AI-evolving
        }
    }
    
    /// Default AI category for migration.
    pub fn default() -> Self {
        AIEvolutionCategory::PassiveUtility
    }
}

// ============================================================================
// Token Type Classification
// ============================================================================

/// Token type classification for fiat collapse behavior
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum TokenType {
    #[serde(rename = "fiat_pegged")]
    FiatPegged,      // USDC, USDT - collapses with fiat (0.01x floor - 1 cent on dollar)
    
    #[serde(rename = "commodity_backed")]
    CommodityBacked, // PAXG, XAUT - tracks commodity price
    
    #[serde(rename = "hard_money")]
    HardMoney,       // BTC - becomes reserve currency (20-50x)
    
    #[serde(rename = "protocol_utility")]
    ProtocolUtility, // SOL, JTO - network effect + utility (10-25x)
    
    #[serde(rename = "real_yield")]
    RealYield,       // USDY, OUSG - yield persists, currency debased (5-10x)
    
    #[serde(rename = "wrapped_bridge")]
    WrappedBridge,   // WBTC, zBTC - parent × bridge risk
    
    #[serde(rename = "liquid_staking")]
    LiquidStaking,   // JitoSOL, Marinade SOL - staked derivatives
    
    #[serde(rename = "governance")]
    Governance,      // ARB, OP - pure speculation (0.2-1.5x, treasury value floor)
    
    #[serde(rename = "meme")]
    Meme,            // WIF, BONK - pure casino (0.05-0.5x, collector value floor)
    
    #[serde(rename = "stableyield")]
    StableYield,     // BRZ, EURC - stablecoins with yield
}

impl TokenType {
    /// Returns the display name for the token type.
    pub fn display_name(&self) -> &'static str {
        match self {
            TokenType::FiatPegged => "Fiat-Pegged Stablecoin",
            TokenType::CommodityBacked => "Commodity-Backed",
            TokenType::HardMoney => "Hard Money (Store of Value)",
            TokenType::ProtocolUtility => "Protocol Utility Token",
            TokenType::RealYield => "Real Yield Asset",
            TokenType::WrappedBridge => "Wrapped/Bridged Asset",
            TokenType::LiquidStaking => "Liquid Staking Token",
            TokenType::Governance => "Governance Token",
            TokenType::Meme => "Meme/Speculative",
            TokenType::StableYield => "Stable Yield Token",
        }
    }

    /// Returns base multiplier range [min, max] for this token type.
    /// Updated with "Reduce 10x" hybrid multipliers for realistic collapse scenario.
    /// 
    /// IMPORTANT: Lower bounds are NEVER 0.0 - this is a "Market Maker" not a garbage disposal.
    /// Every asset has a "Floor of Existence" (0.01 = 1 cent on the dollar).
    /// This prevents divide-by-zero and allows the Exchange to price everything.
    /// 
    /// Hard Money: Gold parity (20x-50x)
    /// Protocol Utility: Network effects capped by economic activity (10x-25x)
    /// Real Yield: Yields valuable but demand drops in depression (5x-10x)
    /// Meme/Governance: Underperform inflation, <1.0 multiplier (loss of purchasing power)
    /// FiatPegged: 0.01 floor (worth 1 cent on dollar, not zero - it's still a forex swap)
    pub fn base_multiplier_range(&self) -> (f64, f64) {
        match self {
            // FIXED: The Floor is now 0.01 (1%), NOT 0.0.
            // This allows the Exchange to accept USDC/USDT and swap it 
            // for 1/100th of a Bitcoin (or whatever the ratio is).
            TokenType::FiatPegged => (0.01, 0.10),         // Floor 0.01 (1 cent on dollar)
            
            // Memes can drop 95%, but they don't vanish. Collector value remains.
            TokenType::Meme => (0.05, 0.50),               // Floor 0.05 (dead memes have collector value)
            
            // "Zombie DAOs" still have treasury value. Never zero.
            TokenType::Governance => (0.20, 1.50),          // Floor 0.20 (treasury value)
            
            // Stable yield: stable but some yield component
            TokenType::StableYield => (0.20, 1.50),         // Floor 0.20 (stable with yield)
            
            TokenType::CommodityBacked => (50.0, 100.0),   // Tracks gold
            TokenType::HardMoney => (20.0, 50.0),           // Gold parity (reduced from 100-200x)
            TokenType::ProtocolUtility => (10.0, 25.0),     // Network effects (reduced from 30-60x)
            TokenType::RealYield => (5.0, 10.0),            // Yield continues (reduced from 15-40x)
            TokenType::WrappedBridge => (15.0, 40.0),       // Parent × bridge risk (reduced from 40-120x)
            TokenType::LiquidStaking => (8.0, 20.0),        // Staked + utility (reduced from 25-50x)
        }
    }

    /// Default token type for migration.
    pub fn default() -> Self {
        TokenType::ProtocolUtility
    }
}

/// Risk class enumeration with proper ordering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum RiskClass {
    #[serde(rename = "Class A (Real Yield)")]
    ClassA,
    #[serde(rename = "Class B (Systemic)")]
    ClassB,
    #[serde(rename = "Class C (Venture Risk)")]
    ClassC,
    #[serde(rename = "Class D (Speculative)")]
    ClassD,
}

impl RiskClass {
    /// Returns a sort order value for sorting by risk class.
    pub fn sort_order(&self) -> u8 {
        match self {
            RiskClass::ClassA => 1,
            RiskClass::ClassB => 2,
            RiskClass::ClassC => 3,
            RiskClass::ClassD => 4,
        }
    }

    /// Returns the display name for the risk class.
    pub fn display_name(&self) -> &'static str {
        match self {
            RiskClass::ClassA => "Class A (Real Yield)",
            RiskClass::ClassB => "Class B (Systemic)",
            RiskClass::ClassC => "Class C (Venture Risk)",
            RiskClass::ClassD => "Class D (Speculative)",
        }
    }
}

impl PartialOrd for RiskClass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RiskClass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_order().cmp(&other.sort_order())
    }
}

/// Represents a Solana token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub symbol: String,
    pub name: String,
    #[serde(default = "TokenType::default")]
    pub token_type: TokenType,
    pub archetype: RiskClass,
    pub insider_score: i32,
    pub tags: Vec<String>,
    pub tariff_override: i32,
    pub reason: String,
    // For wrapped/bridged assets: references the parent token symbol
    #[serde(default)]
    pub parent_token: Option<String>,
    // For commodity-backed: commodity type (e.g., "gold")
    #[serde(default)]
    pub commodity_type: Option<String>,
    // Market cap rank for capital flight calculations (lower = more liquid/blue-chip)
    #[serde(default = "default_rank")]
    pub rank: i32,
    // AI evolution category for AI-acceleration valuation
    #[serde(default = "AIEvolutionCategory::default")]
    pub ai_category: AIEvolutionCategory,

    // Fundamental Data
    #[serde(default)]
    pub market_cap: Option<f64>,
    #[serde(default)]
    pub fdv: Option<f64>,
    #[serde(default)]
    pub price: Option<f64>,
}

fn default_rank() -> i32 { 9999 }

impl Token {
    /// Detect token type from tags if not explicitly set.
    /// This is a fallback for data files that don't have token_type.
    pub fn detect_token_type(&self) -> TokenType {
        // Check for commodity-backed first (gold, silver, etc.)
        if self.tags.iter().any(|t| t == "gold" || t == "commodity") {
            return TokenType::CommodityBacked;
        }
        
        // Check for fiat/stablecoin indicators
        if self.tags.iter().any(|t| t == "fiat" || t == "stable") {
            return TokenType::FiatPegged;
        }
        
        // Check for hard money
        if self.tags.iter().any(|t| t == "hard-money" || t == "pow") {
            if self.parent_token.is_some() {
                return TokenType::WrappedBridge;
            }
            return TokenType::HardMoney;
        }
        
        // Check for liquid staking
        if self.tags.iter().any(|t| t == "liquid-staking" || t.contains("staked") || t == "staked-sol") {
            return TokenType::LiquidStaking;
        }
        
        // Check for meme
        if self.tags.iter().any(|t| t == "meme" || t == "pepe" || t == "dog") {
            return TokenType::Meme;
        }
        
        // Check for real yield / RWA
        if self.tags.iter().any(|t| t == "rwa" || t == "yield" || t == "treasury-yield") {
            return TokenType::RealYield;
        }
        
        // Default based on archetype
        match self.archetype {
            RiskClass::ClassA => TokenType::ProtocolUtility,
            RiskClass::ClassB => TokenType::ProtocolUtility,
            RiskClass::ClassC => TokenType::Governance,
            RiskClass::ClassD => TokenType::Meme,
        }
    }
    
    /// Get the effective token type (explicit or detected).
    pub fn get_token_type(&self) -> TokenType {
        // Use explicit type if it's not the default ProtocolUtility
        if !matches!(self.token_type, TokenType::ProtocolUtility) {
            return self.token_type.clone();
        }
        // Tags are present, so try to detect from tags
        if !self.tags.is_empty() {
            return self.detect_token_type();
        }
        // Fall back to explicit type (which is default ProtocolUtility)
        self.token_type.clone()
    }
    
    /// Calculate the exchange multiplier: 1 + (tariff_override / 100).
    pub fn exchange_multiplier(&self) -> f64 {
        1.0 + (self.tariff_override as f64 / 100.0)
    }
    
    /// Format exchange rate as string with 'x' suffix.
    pub fn exchange_rate(&self) -> String {
        format!("{:.2}x", self.exchange_multiplier())
    }
    
    /// Extract analyst note from reason field (first sentence).
    pub fn analyst_note(&self) -> String {
        if let Some(first_period) = self.reason.find('.') {
            let note = &self.reason[..=first_period];
            note.trim().to_string()
        } else {
            self.reason.clone()
        }
    }
    
    /// Get the effective AI category (explicit or detected from tags).
    pub fn get_ai_category(&self) -> AIEvolutionCategory {
        // Use explicit category if it's not the default PassiveUtility
        if !matches!(self.ai_category, AIEvolutionCategory::PassiveUtility) {
            return self.ai_category.clone();
        }
        // Try to detect from tags
        if !self.tags.is_empty() {
            return Self::detect_ai_category_from_tags(&self.tags);
        }
        // Fall back to explicit category
        self.ai_category.clone()
    }
    
    /// Detect AI category from tags (standalone function)
    pub fn detect_ai_category_from_tags(tags: &[String]) -> AIEvolutionCategory {
        // Check for AI-native first (highest priority)
        let ai_native_tags = ["ai", "agent", "autonomous", "agi", "superintelligence", 
                             "agent-4", "aligner", "virtuals", "ai16z", "fetch", "ocean"];
        if tags.iter().any(|t| ai_native_tags.iter().any(|ai| t.to_lowercase().contains(ai))) {
            return AIEvolutionCategory::AINative;
        }
        
        // Check for AI-enabled (compute, GPU, rendering)
        let ai_enabled_tags = ["compute", "gpu", "render", "akash", "io.net", "cerebras", 
                              "gpu", "cloud", "decentralized-compute", "parallel"];
        if tags.iter().any(|t| ai_enabled_tags.iter().any(|ai| t.to_lowercase().contains(ai))) {
            return AIEvolutionCategory::AIEnabled;
        }
        
        // Check for static (hard money, gold)
        let static_tags = ["hard-money", "pow", "bitcoin", "gold", "btc", "commodity"];
        if tags.iter().any(|t| static_tags.iter().any(|s| t.to_lowercase().contains(s))) {
            return AIEvolutionCategory::Static;
        }
        
        // Default to passive utility
        AIEvolutionCategory::PassiveUtility
    }
}

/// Container for token analysis results.
#[derive(Debug, Clone)]
pub struct TokenAnalysis {
    pub tokens: Vec<Token>,
    pub total_tokens: usize,
}

impl TokenAnalysis {
    /// Create a new TokenAnalysis from tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        let total_tokens = tokens.len();
        Self { tokens, total_tokens }
    }

    /// Return tokens sorted by risk class (A to D).
    pub fn sorted_tokens(&self) -> Vec<&Token> {
        let mut tokens: Vec<&Token> = self.tokens.iter().collect();
        tokens.sort_by(|a, b| a.archetype.cmp(&b.archetype));
        tokens
    }

    /// Return only Class A (Real Yield) tokens.
    pub fn class_a_tokens(&self) -> Vec<&Token> {
        self.tokens
            .iter()
            .filter(|t| t.archetype == RiskClass::ClassA)
            .collect()
    }

    /// Return only Class B (Systemic) tokens.
    pub fn class_b_tokens(&self) -> Vec<&Token> {
        self.tokens
            .iter()
            .filter(|t| t.archetype == RiskClass::ClassB)
            .collect()
    }

    /// Return only Class C (Venture Risk) tokens.
    pub fn class_c_tokens(&self) -> Vec<&Token> {
        self.tokens
            .iter()
            .filter(|t| t.archetype == RiskClass::ClassC)
            .collect()
    }

    /// Return only Class D (Speculative) tokens.
    pub fn class_d_tokens(&self) -> Vec<&Token> {
        self.tokens
            .iter()
            .filter(|t| t.archetype == RiskClass::ClassD)
            .collect()
    }
}

/// Analysis result from the reasoning pricer.
#[derive(Debug, Clone, Serialize)]
pub struct TokenAnalysisResult {
    pub token_type: String,
    pub token_type_display: String,
    pub base_multiplier_range: String,
    pub type_adjustment_factor: f64,
    pub risk_class_adjustment: f64,
    pub insider_risk_factor: f64,
    pub ai_timeline_factor: f64,
    pub ai_category: String,
    pub ai_category_display: String,
    pub current_ai_phase: String,
    pub real_valuation_multiplier: f64,
    pub uncertainty_range: UncertaintyRange,
    pub trading_signal: String,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UncertaintyRange {
    pub lower: f64,
    pub upper: f64,
}
