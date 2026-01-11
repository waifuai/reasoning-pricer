#  Reasoning Pricer

A Rust-based token valuation analysis tool that generates comprehensive markdown reports for Solana tokens using a AI Acceleration reasoning model with AI acceleration dynamics and configurable pricing parameters.

## Overview

This project analyzes token data and generates detailed valuation reports based on a novel pricing framework that models token value under extreme economic scenarios, incorporating **AI acceleration impact** on asset valuations.

## The AI Timeline

The key insight is that **AI progress affects different asset types differently**:
- **Static assets** (BTC, gold, fiat-pegged) decline in relative value as AI accelerates
- **AI-evolving assets** (utility tokens, AI-native protocols) appreciate as AI enables rapid evolution

### Timeline Phases

| Phase | Timeframe | AI Capability | Impact |
|-------|-----------|---------------|--------|
| Global Acceleration Accord | 2026 | AI acceleration frameworks | Acceleration impacts, protocol utility rises |
| Creative Renaissance | 2027 | AI as creative partner | Peak AI acceleration |
| Agentic | 2028+ | Autonomous AI agents | Full AI ecosystem integration |

### AI Impact by Asset Type

| Asset Type | AI Sensitivity | 2025 | 2027 | Trend |
|------------|---------------|------|------|-------|
| **Fiat-Pegged** | Negative | 0.05x | 0.02x | Declines faster with AI |
| **Hard Money** | Low/Negative | 25x | 15x | Static assets lose relative value |
| **Protocol Utility** | High Positive | 12x | 40x | Benefits from AI evolution |
| **AI-Native** | Extreme Positive | 15x | 100x | Rapid appreciation |
| **Liquid Staking** | Moderate Positive | 10x | 25x | Benefits from efficiency |
| **Real Yield** | Moderate | 8x | 12x | Moderate appreciation |
| **Meme** | Volatile | 0.3x | 0.5x | Uncertain |

## Key Features

### Configurable Pricing Parameters

Edit `pricing_config.json` to adjust all valuation parameters without code changes:

```json
{
  "ai_progress_factor": 0.15,
  "timeline_phases": {...},
  "token_type_adjustments": {...},
  "risk_class_multipliers": {...},
  "capital_flight_factors": {...},
  "tariff_calculation": {
    "formula": "max(0, (100 / multiplier) - 10)",
    "min_tariff": 0,
    "max_tariff": 5000
  }
}
```

### AI Category Classifications

| Category | Description | AI Timeline Factor Range |
|----------|-------------|--------------------------|
| **AI-Native** | Built for AI (ai16z, Fetch, Render) | 2.0x - 5.0x |
| **AI-Enabled** | Benefits from AI (Orca, Jito) | 1.5x - 2.5x |
| **Passive Utility** | Standard utility (SOL) | 1.2x - 1.8x |
| **Static Store** | BTC, gold | 0.8x - 1.0x |
| **Fiat-Pegged** | USDC, USDT | 0.3x - 0.6x |
| **Speculative** | Meme tokens | 1.0x - 1.5x |

## Valuation Model

### Core Formula

```
Real Multiplier = Base_Type_Multiplier × AI_Timeline_Factor × Risk_Class_Adjustment × Insider_Risk_Factor × Capital_Flight_Factor
```

### Components

- **Base Type Multiplier**: Midpoint of the token type's range (e.g., 35x for Hard Money)
- **AI Timeline Factor**: Multiplier based on AI acceleration phase and token's AI category
- **Risk Class Adjustment**: 0.6x - 1.2x based on token's risk classification
- **Insider Risk Factor**: 0.5x - 1.0x (inverse of insider concentration score)
- **Capital Flight Factor**: 0.2x - 1.2x based on token liquidity/rank

### Tariff Calculation

The tariff represents trading friction, derived from the real valuation multiplier:

```
tariff = max(0, (100 / real_multiplier) - 10)
```

Examples:
| Real Multiplier | Tariff |
|-----------------|--------|
| 0.01x (USDT) | 14,687% |
| 20.15x (SOL) | 0% |
| 8.40x (tBTC) | 2% |
| 144.84x (RENDER) | 0% |

### Exchange Multiplier

```
exchange_multiplier = 1.0 + (tariff / 100.0)
```

This represents the effective exchange rate adjustment for the token.

### Trading Signals

| Token Type | BUY | HOLD | SELL |
|------------|-----|------|------|
| Hard Money | ≥15x | 8x-15x | <8x |
| Protocol Utility | ≥10x | 3x-10x | <3x |
| AI-Native | ≥20x | 10x-20x | <10x |
| Real Yield | ≥8x | 4x-8x | <4x |
| Liquid Staking | ≥12x | 5x-12x | <5x |
| Fiat-Pegged | - | - | <0.1x |
| Meme | ≥0.5x | 0.2x-0.5x | <0.2x |

## Project Structure

```
reasoning-pricer/
├── Cargo.toml              # Rust project manifest
├── pricing_config.json     # Configurable pricing parameters
├── data/                   # Input token data (JSON)
│   └── *.json             # Individual token files
├── reports/                # Generated output
│   ├── index.md           # Main index
│   ├── tariffs.md         # AI-acceleration tariffs report
│   ├── tokens/            # Individual token reports
│   └── summaries/         # Risk class summaries
├── src/
│   ├── lib.rs             # Library module exports
│   ├── main.rs            # Entry point
│   ├── models.rs          # Data models (Token, TokenType, RiskClass, AITimeline)
│   ├── data_loader.rs     # JSON file loading
│   ├── pricing_config.rs  # Configuration loader
│   ├── reasoning_pricer.rs # Core AI-acceleration valuation logic
│   ├── token_reporter.rs  # Token report generation
│   ├── tariff_generator.rs # Tariff report generation
│   ├── summary_reporter.rs # Risk class summary generation
│   └── index_generator.rs # Index file generation
└── README.md              # This file
```

## Usage

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build and Run

```bash
# Build the project
cargo build --release

# Run the report generator
cargo run --release
```

### Configuration

Edit `pricing_config.json` to adjust:

1. **AI Progress Factor** - How much AI acceleration impacts valuations (0.0 - 0.3)
2. **Timeline Phases** - Timeline multipliers for each AI phase
3. **Token Type Adjustments** - Base multipliers per token type
4. **Risk Class Multipliers** - Risk class impact on valuation
5. **Capital Flight Factors** - Liquidity-based adjustments
6. **Tariff Calculation** - Formula and bounds

### Input Data Format

Token data files in `data/` should be JSON arrays with the following structure:

```json
[
  {
    "symbol": "SOL",
    "name": "Solana",
    "archetype": "Class B (Systemic)",
    "insider_score": 48,
    "rank": 5,
    "token_type": "ProtocolUtility",
    "ai_category": "PassiveUtility",
    "tags": ["native", "gas", "layer-1"],
    "tariff_override": 30,
    "reason": "The Currency of the Realm."
  }
]
```

### Token Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `symbol` | string | Yes | Token ticker symbol |
| `name` | string | Yes | Full token name |
| `archetype` | string | Yes | Risk class (e.g., "Class A (Real Yield)") |
| `insider_score` | number | Yes | 0-100 insider concentration (higher = more centralized) |
| `tags` | array | Yes | Array of tags for auto-detection |
| `tariff_override` | number | No | Manual tariff friction percentage |
| `reason` | string | No | Analyst notes |
| `rank` | number | No | Market cap rank (1-999) for capital flight calculation |
| `token_type` | string | No | Explicit type override (see below) |
| `ai_category` | string | No | Explicit AI category override (see below) |
| `parent_token` | string | No | Parent token for wrapped/bridged assets |

### Token Types

| Value | Base Multiplier | Description |
|-------|----------------|-------------|
| `HardMoney` | 20x - 50x | BTC, decentralized PoW |
| `CommodityBacked` | 5x - 10x | Gold, silver tokenized |
| `WrappedBridge` | 10x - 25x | Cross-chain wrapped assets |
| `RealYield` | 0.5x - 1.5x | Revenue-generating protocols |
| `FiatPegged` | 0.01x - 0.05x | USDC, USDT, stablecoins |
| `LiquidStaking` | 0.5x - 1.5x | LSTs like JitoSOL, mSOL |
| `ProtocolUtility` | 10x - 25x | General utility tokens |
| `Governance` | 0.1x - 1.0x | Pure governance tokens |
| `Meme` | 0.01x - 0.2x | Meme/culture tokens |
| `AINative` | 15x - 40x | AI agent tokens |
| `AIEnabled` | 10x - 30x | DePIN/GPU compute |

### AI Categories

| Value | Timeline Factor | Description |
|-------|----------------|-------------|
| `Static` | 0.20x | Cannot evolve (BTC, gold) |
| `PassiveUtility` | 1.00x | Limited AI adaptation |
| `AIEnabled` | 8.33x | Can integrate AI (DePIN) |
| `AINative` | 8.33x | Built for AI ecosystems |
| `AIEvolving` | 12.50x | Self-modifying with AI |

### Rank-Based Capital Flight Factor

| Rank Range | Factor | Description |
|------------|--------|-------------|
| 1-9 | 1.2x | Blue-chip premium |
| 10-49 | 1.0x | No adjustment |
| 50-99 | 0.8x | Slight penalty |
| 100-299 | 0.5x | Moderate penalty |
| 300+ | 0.2x | Heavy penalty for illiquid alts |

### Tag-Based Auto-Detection

If `token_type` is not explicitly set, it's auto-detected from tags:

| Tag Pattern | Detected Type |
|-------------|---------------|
| `hard-money`, `pow` | HardMoney |
| `commodity`, `gold` | CommodityBacked |
| `fiat-pegged` | FiatPegged |
| `liquid-staking`, `lst` | LiquidStaking |
| `real-yield` | RealYield |
| `governance` | Governance |
| `meme` | Meme |
| `bridged` | WrappedBridge |

If `ai_category` is not explicitly set, it's auto-detected from tags:

| Tag Pattern | AI Category |
|-------------|-------------|
| `ai` + `agent` | AINative |
| `gpu-compute`, `depin` | AIEnabled |
| `hard-money`, `pow`, `gold`, `commodity` | Static |
| (default) | PassiveUtility |

### Output

Generated reports are saved to the `reports/` directory:
- `reports/index.md` - Main index with all tokens
- `reports/tariffs.md` - AI-acceleration tariffs summary
- `reports/tokens/{symbol}.md` - Individual token analysis
- `reports/summaries/{class}.md` - Risk class summary

## Example Output

### SOL (Protocol Utility)
```
Token Type: ProtocolUtility
AI Category: Passive Utility
Rank: 5 (Blue-chip premium: 1.2x)
Base Multiplier Range: 10x - 25x
AI Timeline Factor: 1.00x
Real Valuation Multiplier: 20.15x
Tariff: 0%
Trading Signal: HOLD
```

### USDT (Fiat-Pegged)
```
Token Type: FiatPegged
AI Category: Static
Rank: 4 (Blue-chip: 1.2x)
Base Multiplier Range: 0.01x - 0.05x
AI Timeline Factor: 0.20x
Real Valuation Multiplier: 0.01x
Tariff: 14,687%
Trading Signal: AVOID
Reasoning: Fiat-pegged assets collapse under AI acceleration.
```

### RENDER (AI-Enabled DePIN)
```
Token Type: ProtocolUtility
AI Category: AI-Enabled
Rank: 50 (Mid-tier: 1.0x)
Base Multiplier Range: 10x - 25x
AI Timeline Factor: 8.33x
Real Valuation Multiplier: 144.84x
Tariff: 0%
Trading Signal: HOLD
Reasoning: AI-Enabled DePIN tokens benefit enormously from AI acceleration.
```

### tBTC (Hard Money)
```
Token Type: HardMoney
AI Category: Static
Rank: 10 (Blue-chip: 1.2x)
Base Multiplier Range: 20x - 50x
AI Timeline Factor: 0.20x
Real Valuation Multiplier: 8.40x
Tariff: 2%
Trading Signal: BUY
Reasoning: Hard money assets retain value but don't benefit from AI.
```

## Dependencies

- `serde` / `serde_json` - JSON serialization
- `chrono` - Date/time handling
- `clap` - Command-line argument parsing
- `anyhow` - Error handling
- `thiserror` - Custom error types
