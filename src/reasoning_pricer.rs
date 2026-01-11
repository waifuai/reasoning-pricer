//! Reasoning Pricer Module - Post-Fiat Collapse Valuation Analysis
//!
//! Implements type-acceleration valuation for calculating real token valuations after fiat collapse.
//! Includes AI-acceleration pricing based on the AI acceleration timeline.

use crate::models::{
    AIEvolutionCategory, AITimelinePhase, RiskClass, Token, 
    TokenAnalysisResult, TokenType, UncertaintyRange,
};

/// Calculates post-fiat collapse valuations for tokens with type accelerationness.
#[derive(Debug, Clone)]
pub struct ReasoningPricer {
    /// Risk class multipliers (adjust base multiplier based on token robustness)
    risk_class_multipliers: std::collections::HashMap<RiskClass, f64>,
}

impl Default for ReasoningPricer {
    fn default() -> Self {
        Self::new()
    }
}

impl ReasoningPricer {
    /// Create a new ReasoningPricer with default values.
    pub fn new() -> Self {
        let mut risk_class_multipliers = std::collections::HashMap::new();
        risk_class_multipliers.insert(RiskClass::ClassA, 1.2); // Real yield tokens are most resilient
        risk_class_multipliers.insert(RiskClass::ClassB, 1.0); // Systemic tokens maintain their value
        risk_class_multipliers.insert(RiskClass::ClassC, 0.8); // Venture risk tokens lose some value
        risk_class_multipliers.insert(RiskClass::ClassD, 0.6); // Speculative tokens lose significant value

        Self {
            risk_class_multipliers,
        }
    }

    /// Calculate risk factor based on insider control.
    /// High insider control = higher risk of value destruction during collapse.
    pub fn calculate_insider_risk_factor(&self, insider_score: i32) -> f64 {
        // Normalize insider score to 0-1 range
        let insider_factor = insider_score as f64 / 100.0;

        // Inverse relationship: higher insider control = lower multiplier
        // Range: 0.5 (max control) to 1.0 (no control)
        1.0 - (insider_factor * 0.5)
    }

    /// Calculate the "Capital Flight" factor based on token liquidity/rank.
    /// In a collapse, capital flees TO the top (BTC, ETH, SOL) and AWAY from small-cap alts.
    ///
    /// - rank < 10: Blue-chip premium (1.2x) - safety assets become "new fiat"
    /// - rank < 100: Slight drag (0.8x) - still relatively liquid
    /// - rank >= 100: Heavy penalty (0.2x) - illiquid, capital flight crushes value
    pub fn calculate_capital_flight_factor(&self, token: &Token) -> f64 {
        if token.rank < 10 {
            1.2 // Premium for being a "Blue Chip" safety asset
        } else if token.rank < 100 {
            0.8 // Slight drag for mid-tier tokens
        } else {
            0.2 // 80% penalty for illiquid alts (everyone sells to buy BTC/Food)
        }
    }

    /// Get the type adjustment factor based on token type.
    /// This is the core of the fix - different token types behave differently.
    pub fn get_type_adjustment_factor(&self, token: &Token) -> f64 {
        let token_type = token.get_token_type();
        
        // Get base multiplier range for this token type
        let (min_multiplier, max_multiplier) = token_type.base_multiplier_range();
        
        // Use midpoint as the base adjustment factor
        let base_factor = (min_multiplier + max_multiplier) / 2.0;
        
        base_factor
    }

    // ============================================================================
    // AI Timeline Calculation Methods
    // ============================================================================

    /// Get the current AI timeline phase based on date.
    /// Default: Use current date (2026-01 for this implementation).
    pub fn get_current_ai_phase(&self) -> AITimelinePhase {
        // Default to 2026 Q1 for now - this would typically come from system clock
        AITimelinePhase::for_date(2026, 1)
    }

    /// Calculate the AI timeline adjustment factor for a token.
    /// This is the key factor that causes static assets to decline and AI assets to grow.
    ///
    /// Formula: Current_Multiplier / Baseline_Multiplier
    ///
    /// - Static assets: 25.0 → 15.0 → 10.0 → 5.0 → 2.0 (decline 92% by 2027)
    /// - AI-Native assets: 3.0 → 8.0 → 15.0 → 25.0 → 50.0 (grow 1567% by 2027)
    pub fn calculate_ai_timeline_factor(&self, token: &Token, phase: &AITimelinePhase) -> f64 {
        let ai_category = token.get_ai_category();
        let baseline = ai_category.baseline_multiplier();
        
        // Get the appropriate current multiplier based on AI category
        let current_multiplier = match ai_category {
            AIEvolutionCategory::Static => phase.hard_money_multiplier,
            
            AIEvolutionCategory::PassiveUtility => {
                // Passive utility follows protocol utility multiplier
                phase.protocol_utility_multiplier
            }
            
            AIEvolutionCategory::AIEnabled => {
                // AI-enabled is between AI utility and protocol utility
                // Starts closer to protocol, moves toward AI utility over time
                let base = phase.protocol_utility_multiplier;
                let target = phase.ai_utility_multiplier;
                // Linear interpolation weighted toward target as AI progresses
                let progress = self.get_ai_progress_weight(phase.year);
                base + (target - base) * progress
            }
            
            AIEvolutionCategory::AINative => phase.ai_utility_multiplier,
            
            AIEvolutionCategory::AIEvolving => {
                // AI-evolving has a premium over AI-native
                // Can self-modify, so gets extra 20-30% boost
                phase.ai_utility_multiplier * 1.25
            }
        };
        
        // Calculate the factor (current / baseline)
        // This shows how much the asset has appreciated/depreciated due to AI timeline
        if baseline > 0.0 {
            current_multiplier / baseline
        } else {
            1.0
        }
    }

    /// Get AI progress weight (0.0 to 1.0) based on year.
    /// Used for interpolating between categories.
    fn get_ai_progress_weight(&self, year: i32) -> f64 {
        match year {
            y if y < 2025 => 0.0,   // Pre-AI
            2025 => 0.25,           // Early AI
            2026 => 0.6,            // Mid AI
            y if y >= 2027 => 1.0,  // Full AI
            _ => 0.0,
        }
    }

    /// Calculate the AI-acceleration real valuation multiplier.
    ///
    /// Formula:
    /// Base_Type_Multiplier × AI_Timeline_Factor × Risk_Class_Multiplier × Insider_Risk_Factor × Capital_Flight_Factor
    ///
    /// The AI Timeline Factor is the key addition - it:
    /// - Reduces static assets (BTC) as AI progresses
    /// - Increases AI-native assets as AI accelerates
    pub fn calculate_ai_acceleration_multiplier(&self, token: &Token, phase: &AITimelinePhase) -> f64 {
        let token_type = token.get_token_type();
        let (min_multiplier, max_multiplier) = token_type.base_multiplier_range();
        
        // Start with the midpoint of the type's base multiplier range
        let base_multiplier = (min_multiplier + max_multiplier) / 2.0;

        // NEW: AI Timeline Factor - the core of AI-acceleration pricing
        let ai_timeline_factor = self.calculate_ai_timeline_factor(token, phase);

        // Adjust for token's risk class
        let risk_multiplier = self.risk_class_multipliers
            .get(&token.archetype)
            .copied()
            .unwrap_or(1.0);

        // Adjust for insider control
        let insider_risk_factor = self.calculate_insider_risk_factor(token.insider_score);

        // Capital Flight Factor - rewards liquidity and penalizes illiquid alts
        let capital_flight_factor = self.calculate_capital_flight_factor(token);

        // Calculate final multiplier with AI timeline factor
        base_multiplier * ai_timeline_factor * risk_multiplier * insider_risk_factor * capital_flight_factor
    }

    /// Calculate uncertainty factor based on token type.
    /// Higher-risk token types have wider uncertainty bands.
    pub fn get_uncertainty_factor(&self, token: &Token) -> f64 {
        let token_type = token.get_token_type();
        
        match token_type {
            // Low uncertainty for hard money and stable assets
            TokenType::HardMoney => 0.20,
            TokenType::FiatPegged => 0.10,
            TokenType::CommodityBacked => 0.25,
            
            // Medium uncertainty for protocol assets
            TokenType::ProtocolUtility => 0.30,
            TokenType::RealYield => 0.30,
            TokenType::WrappedBridge => 0.35,
            TokenType::LiquidStaking => 0.35,
            
            // High uncertainty for speculative assets
            TokenType::Governance => 0.50,
            TokenType::Meme => 0.80,
            TokenType::StableYield => 0.40,
        }
    }

    /// Calculate the real valuation multiplier for a token post-fiat collapse.
    /// Now uses AI-acceleration calculation for proper AI timeline pricing.
    ///
    /// Formula: Base_Type_Multiplier × AI_Timeline_Factor × Risk_Class_Multiplier × Insider_Risk_Factor × Capital_Flight_Factor
    ///
    /// The AI Timeline Factor rewards AI-evolving assets and penalizes static assets.
    pub fn calculate_real_valuation_multiplier(&self, token: &Token) -> f64 {
        let phase = self.get_current_ai_phase();
        self.calculate_ai_acceleration_multiplier(token, &phase)
    }

    /// Calculate uncertainty bands around the real valuation.
    pub fn calculate_uncertainty_range(&self, real_multiplier: f64, token: &Token) -> UncertaintyRange {
        let uncertainty_factor = self.get_uncertainty_factor(token);

        let lower = real_multiplier * (1.0 - uncertainty_factor);
        let upper = real_multiplier * (1.0 + uncertainty_factor);

        // Use higher precision for small multipliers
        if real_multiplier < 0.1 {
            UncertaintyRange {
                lower: (lower * 100.0).round() / 100.0,
                upper: (upper * 100.0).round() / 100.0,
            }
        } else {
            UncertaintyRange {
                lower: (lower * 10.0).round() / 10.0,
                upper: (upper * 10.0).round() / 10.0,
            }
        }
    }

    /// Generate trading signal based on reasoning pricer analysis.
    ///
    /// Signal logic varies by token type (adjusted for new "Reduce 10x" multipliers):
    /// - Hard money: > 20x = BUY (Gold parity scenario)
    /// - Fiat-pegged: SELL (collapses with fiat)
    /// - Protocol utility: > 15x = BUY, 8-15x = HOLD, < 8x = SELL
    /// - Meme: Only buy if any positive multiplier (extremely speculative in collapse)
    pub fn generate_trading_signal(&self, real_multiplier: f64, token: &Token) -> &'static str {
        let token_type = token.get_token_type();

        // Fiat-pegged always SELL in fiat collapse scenario
        if matches!(token_type, TokenType::FiatPegged) {
            return "SELL";
        }

        // Use calculated tariff instead of manual override for signal decision

        match token_type {
            TokenType::FiatPegged => "SELL", // Already handled above, but compiler requires it
            TokenType::HardMoney => {
                if real_multiplier >= 20.0 { "BUY" } else { "HOLD" }
            }
            TokenType::CommodityBacked => {
                if real_multiplier >= 30.0 { "BUY" } else { "HOLD" }
            }
            TokenType::ProtocolUtility | TokenType::LiquidStaking => {
                if real_multiplier >= 10.0 { "BUY" }
                else if real_multiplier >= 3.0 { "HOLD" }
                else { "SELL" }
            }
            TokenType::RealYield => {
                if real_multiplier >= 8.0 { "BUY" }
                else if real_multiplier >= 4.0 { "HOLD" }
                else { "SELL" }
            }
            TokenType::WrappedBridge => {
                if real_multiplier >= 15.0 { "BUY" }
                else if real_multiplier >= 8.0 { "HOLD" }
                else { "SELL" }
            }
            TokenType::Governance => {
                if real_multiplier >= 2.0 { "BUY" }
                else if real_multiplier >= 1.0 { "HOLD" }
                else { "SELL" }
            }
            TokenType::Meme => {
                if real_multiplier >= 0.5 { "BUY" } // Even positive multiplier is a BUY for memes
                else { "SELL" }
            }
            TokenType::StableYield => {
                if real_multiplier >= 2.0 { "BUY" }
                else { "SELL" }
            }
        }
    }

    /// Perform complete reasoning pricer analysis on a token.
    pub fn analyze_token(&self, token: &Token) -> TokenAnalysisResult {
        let token_type = token.get_token_type();
        let (min_multiplier, max_multiplier) = token_type.base_multiplier_range();
        
        // Get AI timeline phase and calculate AI-related factors
        let phase = self.get_current_ai_phase();
        let ai_category = token.get_ai_category();
        let ai_timeline_factor = self.calculate_ai_timeline_factor(token, &phase);
        
        // Calculate core metrics
        let real_multiplier = self.calculate_ai_acceleration_multiplier(token, &phase);
        let uncertainty_range = self.calculate_uncertainty_range(real_multiplier, token);
        let trading_signal = self.generate_trading_signal(real_multiplier, token);
        
        // Generate detailed reasoning
        let reasoning = self.generate_ai_acceleration_reasoning(token, real_multiplier, &trading_signal, &phase, ai_timeline_factor);

        let type_adjustment = self.get_type_adjustment_factor(token);
        let risk_class_adjustment = self.risk_class_multipliers
            .get(&token.archetype)
            .copied()
            .unwrap_or(1.0);

        TokenAnalysisResult {
            token_type: format!("{:?}", token_type),
            token_type_display: token_type.display_name().to_string(),
            // Use dynamic formatting based on multiplier magnitude
            base_multiplier_range: if min_multiplier < 0.1 {
                format!("{:.2}x - {:.2}x", min_multiplier, max_multiplier)
            } else if min_multiplier < 1.0 {
                format!("{:.1}x - {:.1}x", min_multiplier, max_multiplier)
            } else {
                format!("{:.0}x - {:.0}x", min_multiplier, max_multiplier)
            },
            type_adjustment_factor: (type_adjustment * 10.0).round() / 10.0,
            risk_class_adjustment,
            insider_risk_factor: self.calculate_insider_risk_factor(token.insider_score),
            ai_timeline_factor: (ai_timeline_factor * 100.0).round() / 100.0,
            ai_category: format!("{:?}", ai_category),
            ai_category_display: ai_category.display_name().to_string(),
            current_ai_phase: phase.name.to_string(),
            // Use higher precision for small multipliers
            real_valuation_multiplier: if real_multiplier < 0.1 {
                (real_multiplier * 100.0).round() / 100.0
            } else {
                (real_multiplier * 10.0).round() / 10.0
            },
            uncertainty_range,
            trading_signal: trading_signal.to_string(),
            reasoning,
        }
    }

    /// Generate detailed AI-acceleration reasoning for the analysis.
    fn generate_ai_acceleration_reasoning(&self, token: &Token, real_multiplier: f64, signal: &str, phase: &AITimelinePhase, ai_timeline_factor: f64) -> String {
        let token_type = token.get_token_type();
        let ai_category = token.get_ai_category();
        let risk_class = &token.archetype;
        let risk_adj = self.risk_class_multipliers
            .get(risk_class)
            .copied()
            .unwrap_or(1.0);
        let insider_factor = self.calculate_insider_risk_factor(token.insider_score);
        let capital_flight_factor = self.calculate_capital_flight_factor(token);
        let (min_multiplier, max_multiplier) = token_type.base_multiplier_range();

        let mut reasoning_parts = Vec::new();

        // AI Timeline Context
        reasoning_parts.push(format!(
            "[AI Timeline: {}] Asset classified as {} ({}).",
            phase.name,
            ai_category.display_name(),
            ai_category.description()
        ));

        // AI Timeline Factor explanation
        if ai_timeline_factor > 1.5 {
            reasoning_parts.push(format!(
                "AI Timeline Factor: {:.2}x (AI acceleration boosts this asset significantly).",
                ai_timeline_factor
            ));
        } else if ai_timeline_factor < 0.5 {
            reasoning_parts.push(format!(
                "AI Timeline Factor: {:.2}x (AI acceleration penalizes this static asset).",
                ai_timeline_factor
            ));
        } else {
            reasoning_parts.push(format!(
                "AI Timeline Factor: {:.2}x (neutral to moderate AI impact).",
                ai_timeline_factor
            ));
        }

        // Token type explanation
        let range_str = if min_multiplier < 0.1 {
            format!("{:.2}x - {:.2}x", min_multiplier, max_multiplier)
        } else if min_multiplier < 1.0 {
            format!("{:.1}x - {:.1}x", min_multiplier, max_multiplier)
        } else {
            format!("{:.0}x - {:.0}x", min_multiplier, max_multiplier)
        };
        reasoning_parts.push(format!(
            "{}: {} (base range: {}).",
            token_type.display_name(),
            self.get_type_description(&token_type),
            range_str
        ));

        // Risk class adjustment
        if (risk_adj - 1.0).abs() > f64::EPSILON {
            if risk_adj > 1.0 {
                reasoning_parts.push(format!(
                    "{} receives {:.1}x risk class boost - resilient during market stress.",
                    risk_class.display_name(),
                    risk_adj
                ));
            } else {
                reasoning_parts.push(format!(
                    "{} receives {:.1}x risk class reduction - higher uncertainty.",
                    risk_class.display_name(),
                    risk_adj
                ));
            }
        }

        // Insider control adjustment
        if insider_factor < 1.0 {
            reasoning_parts.push(format!(
                "Insider control ({}/100) reduces multiplier by {:.2}x due to centralization risks.",
                token.insider_score, insider_factor
            ));
        }

        // Capital Flight Factor explanation
        let capital_flight_desc = if capital_flight_factor > 1.0 {
            format!(
                "Rank #{} gives {:.1}x premium (blue-chip safety asset).",
                token.rank, capital_flight_factor
            )
        } else if capital_flight_factor < 0.5 {
            format!(
                "Rank #{} imposes {:.1}x penalty (illiquid, capital flight crushes value).",
                token.rank, capital_flight_factor
            )
        } else {
            format!(
                "Rank #{} applies {:.1}x adjustment (mid-tier liquidity).",
                token.rank, capital_flight_factor
            )
        };
        reasoning_parts.push(capital_flight_desc);

        // Final multiplier explanation with AI context
        let ai_context = if ai_timeline_factor > 1.2 {
            "AI acceleration provides strong tailwinds."
        } else if ai_timeline_factor < 0.8 {
            "AI acceleration creates headwinds for this asset class."
        } else {
            "AI acceleration has neutral impact on this asset."
        };
        
        if real_multiplier < 0.1 {
            if matches!(token_type, TokenType::FiatPegged) {
                reasoning_parts.push(format!(
                    "Final real valuation multiplier: {:.2}x. Token trading at $1.00 today would be worth ${:.2} post-fiat collapse ({}). {}",
                    real_multiplier, real_multiplier, "collapses with fiat", ai_context
                ));
            } else {
                reasoning_parts.push(format!(
                    "Final real valuation multiplier: {:.2}x. Token trading at $1.00 today would be worth ${:.2} post-fiat collapse ({}). {}",
                    real_multiplier, real_multiplier, "loss of purchasing power", ai_context
                ));
            }
        } else {
            if real_multiplier < 1.0 {
                reasoning_parts.push(format!(
                    "Final real valuation multiplier: {:.1}x. Token trading at $1.00 today would be worth ${:.2} post-fiat collapse ({}). {}",
                    real_multiplier, real_multiplier,
                    if matches!(token_type, TokenType::FiatPegged) { "collapses with fiat" } else { "loss of purchasing power" },
                    ai_context
                ));
            } else {
                reasoning_parts.push(format!(
                    "Final real valuation multiplier: {:.1}x. Token trading at $1.00 today would be worth ${:.2} post-fiat collapse. {}",
                    real_multiplier, real_multiplier, ai_context
                ));
            }
        }

        // Trading signal explanation
        let signal_explanation = match signal {
            "BUY" => match token_type {
                TokenType::FiatPegged => unreachable!(),
                TokenType::Meme => "High upside potential despite extreme risk. Position small.",
                TokenType::HardMoney | TokenType::CommodityBacked => "Strong store of value. Accumulate before scarcity kicks in.",
                _ => "Undervalued post-collapse. Accumulate before systemic shift.",
            },
            "HOLD" => "Moderate value post-collapse. Maintain current position.",
            _ => match token_type {
                TokenType::FiatPegged => "Collapses with fiat currency. Exit immediately.",
                _ => "Overvalued or high risk post-collapse. Reduce exposure.",
            },
        };

        reasoning_parts.push(format!("Trading signal ({}): {}", signal, signal_explanation));

        reasoning_parts.join(" ")
    }

    /// Generate detailed reasoning for the analysis.
    fn generate_reasoning(&self, token: &Token, real_multiplier: f64, signal: &str) -> String {
        let token_type = token.get_token_type();
        let risk_class = &token.archetype;
        let risk_adj = self.risk_class_multipliers
            .get(risk_class)
            .copied()
            .unwrap_or(1.0);
        let insider_factor = self.calculate_insider_risk_factor(token.insider_score);
        let capital_flight_factor = self.calculate_capital_flight_factor(token);
        let (min_multiplier, max_multiplier) = token_type.base_multiplier_range();

        let mut reasoning_parts = Vec::new();

        // Token type explanation
        // Use dynamic formatting based on multiplier magnitude
        let range_str = if min_multiplier < 0.1 {
            format!("{:.2}x - {:.2}x", min_multiplier, max_multiplier)
        } else if min_multiplier < 1.0 {
            format!("{:.1}x - {:.1}x", min_multiplier, max_multiplier)
        } else {
            format!("{:.0}x - {:.0}x", min_multiplier, max_multiplier)
        };
        reasoning_parts.push(format!(
            "{}: {} (base range: {}).",
            token_type.display_name(),
            self.get_type_description(&token_type),
            range_str
        ));

        // Risk class adjustment
        if (risk_adj - 1.0).abs() > f64::EPSILON {
            if risk_adj > 1.0 {
                reasoning_parts.push(format!(
                    "{} receives {:.1}x risk class boost - resilient during market stress.",
                    risk_class.display_name(),
                    risk_adj
                ));
            } else {
                reasoning_parts.push(format!(
                    "{} receives {:.1}x risk class reduction - higher uncertainty.",
                    risk_class.display_name(),
                    risk_adj
                ));
            }
        }

        // Insider control adjustment
        if insider_factor < 1.0 {
            reasoning_parts.push(format!(
                "Insider control ({}/100) reduces multiplier by {:.2}x due to centralization risks.",
                token.insider_score, insider_factor
            ));
        }

        // Capital Flight Factor explanation
        let capital_flight_desc = if capital_flight_factor > 1.0 {
            format!(
                "Rank #{} gives {:.1}x premium (blue-chip safety asset).",
                token.rank, capital_flight_factor
            )
        } else if capital_flight_factor < 0.5 {
            format!(
                "Rank #{} imposes {:.1}x penalty (illiquid, capital flight crushes value).",
                token.rank, capital_flight_factor
            )
        } else {
            format!(
                "Rank #{} applies {:.1}x adjustment (mid-tier liquidity).",
                token.rank, capital_flight_factor
            )
        };
        reasoning_parts.push(capital_flight_desc);

        // Final multiplier explanation
        // Use higher precision for small multipliers
        if real_multiplier < 0.1 {
            if matches!(token_type, TokenType::FiatPegged) {
                reasoning_parts.push(format!(
                    "Final real valuation multiplier: {:.2}x. Token trading at $1.00 today would be worth ${:.2} post-fiat collapse ({}).",
                    real_multiplier, real_multiplier, "collapses with fiat"
                ));
            } else {
                reasoning_parts.push(format!(
                    "Final real valuation multiplier: {:.2}x. Token trading at $1.00 today would be worth ${:.2} post-fiat collapse ({}).",
                    real_multiplier, real_multiplier, "loss of purchasing power"
                ));
            }
        } else {
            if real_multiplier < 1.0 {
                reasoning_parts.push(format!(
                    "Final real valuation multiplier: {:.1}x. Token trading at $1.00 today would be worth ${:.2} post-fiat collapse ({}).",
                    real_multiplier, real_multiplier,
                    if matches!(token_type, TokenType::FiatPegged) { "collapses with fiat" } else { "loss of purchasing power" }
                ));
            } else {
                reasoning_parts.push(format!(
                    "Final real valuation multiplier: {:.1}x. Token trading at $1.00 today would be worth ${:.2} post-fiat collapse.",
                    real_multiplier, real_multiplier
                ));
            }
        }

        // Trading signal explanation
        let signal_explanation = match signal {
            "BUY" => match token_type {
                TokenType::FiatPegged => unreachable!(),
                TokenType::Meme => "High upside potential despite extreme risk. Position small.",
                TokenType::HardMoney | TokenType::CommodityBacked => "Strong store of value. Accumulate before scarcity kicks in.",
                _ => "Undervalued post-collapse. Accumulate before systemic shift.",
            },
            "HOLD" => "Moderate value post-collapse. Maintain current position.",
            _ => match token_type {
                TokenType::FiatPegged => "Collapses with fiat currency. Exit immediately.",
                _ => "Overvalued or high risk post-collapse. Reduce exposure.",
            },
        };

        reasoning_parts.push(format!("Trading signal ({}): {}", signal, signal_explanation));

        reasoning_parts.join(" ")
    }

    /// Get a description of the token type's behavior.
    /// Updated to reflect the "Universal Non-Zero Doctrine" - every asset has a floor of existence.
    fn get_type_description(&self, token_type: &TokenType) -> &'static str {
        match token_type {
            // Floor is 0.01 (1 cent on dollar) - worth something, not zero
            TokenType::FiatPegged => "directly tied to fiat collapse (0.01x floor - 1 cent on dollar)",
            
            // Commodity-backed tracks the underlying
            TokenType::CommodityBacked => "tracks commodity price discovery",
            
            // Hard money becomes reserve currency
            TokenType::HardMoney => "becomes the new reserve currency",
            
            // Protocol utility benefits from network effects
            TokenType::ProtocolUtility => "benefits from network effects and protocol utility",
            
            // Real yield persists but currency debasement affects it
            TokenType::RealYield => "yield persists but currency debasement affects valuation",
            
            // Wrapped bridge inherits parent minus risk
            TokenType::WrappedBridge => "inherits parent asset value minus bridge/custody risk",
            
            // Liquid staking is staked derivative
            TokenType::LiquidStaking => "staked derivative with validator rewards",
            
            // Governance has treasury value floor (0.20x)
            TokenType::Governance => "speculative value based on protocol governance (zombie DAOs have treasury value)",
            
            // Meme has collector value floor (0.05x) - dead memes don't vanish
            TokenType::Meme => "pure speculation with minimal fundamental value (0.05x floor - collector value)",
            
            // Stable yield has some yield component
            TokenType::StableYield => "stable value with yield-bearing characteristics",
        }
    }
}
