# The Mathematics of Predicted Price and Tariff Rates

How does the Reasoning Pricer determine the **real value** of a token in an AI-accelerating economy? This post breaks down the core formulas that power the system.

## The Predicted Price Formula

The **Predicted Price** represents what a token should be worth after adjusting for AI acceleration. It's calculated as:

```
Predicted Price = Current Price × Real Valuation Multiplier
```

The **Real Valuation Multiplier** is the heart of the system—a composite score that captures a token's true worth:

```
Real Valuation Multiplier = Base_Type × AI_Timeline × Risk_Class × Insider_Risk × Capital_Flight × Liquidity_Risk
```

Let's examine each component.

---

## Component 1: Base Type Multiplier

Every token type has an inherent value range based on its fundamental utility:

| Token Type | Base Range | Rationale |
|------------|------------|-----------|
| Hard Money (BTC) | 20x - 50x | Scarcity-backed store of value |
| Protocol Utility | 10x - 25x | Network effect driven |
| AI-Native | 15x - 40x | Maximum AI acceleration benefits |
| Governance | 0.1x - 1.0x | Value derived from protocol control |
| Meme | 0.01x - 0.2x | Purely speculative |
| Fiat-Pegged | 0.01x - 0.05x | Collapses with fiat currency |

The Base Multiplier is the midpoint of each range:

```
Base_Multiplier = (Min_Range + Max_Range) / 2
```

---

## Component 2: AI Timeline Factor

This is **the key innovation**—different assets respond differently to AI progress. The factor is calculated as:

```
AI_Timeline_Factor = Current_Phase_Multiplier / Baseline_Multiplier
```

### Timeline Phases (2024-2027)

| Phase | Year | Static Assets | AI-Native | Protocol Utility |
|-------|------|---------------|-----------|------------------|
| Pre-AI Baseline | 2024 | 25.0x | 3.0x | 10.0x |
| Personal Architect | 2025 Q2 | 15.0x | 8.0x | 12.0x |
| Global Accord | 2025 Q4 | 10.0x | 15.0x | 15.0x |
| **Creative Renaissance** | **2026** | **5.0x** | **25.0x** | **18.0x** |
| Agentic | 2027 | 2.0x | 50.0x | 25.0x |

### AI Category Multipliers (2026)

For the current phase (Creative Renaissance), each AI category has a specific timeline multiplier:

| AI Category | Timeline Multiplier | Interpretation |
|-------------|---------------------|----------------|
| 🗿 Static | 0.20x | Declines 80% as AI progresses |
| 🔋 Passive Utility | 1.80x | Modest gains from AI |
| 🧠 AI-Enabled | 5.00x | Significant AI benefits |
| 🤖 AI-Native | 25.00x | Maximum AI leverage |
| 🧬 AI-Evolving | 40.00x | Self-modifying premium |

**Example**: A Static asset (like BTC) in 2026:
- Current Phase Multiplier: 5.0x
- Baseline Multiplier: 25.0x
- AI Timeline Factor: 5.0 / 25.0 = **0.20x** (80% decline)

**Example**: An AI-Native token in 2026:
- Current Phase Multiplier: 25.0x
- Baseline Multiplier: 3.0x
- AI Timeline Factor: 25.0 / 3.0 = **8.33x** (733% gain)

---

## Component 3: Risk Class Adjustment

Tokens are classified by their risk profile:

| Risk Class | Multiplier | Description |
|------------|------------|-------------|
| 🛡️ Class A (Real Yield) | 1.2x | Generates real returns |
| 🏦 Class B (Systemic) | 1.0x | Core infrastructure |
| 🚀 Class C (Venture Risk) | 0.8x | Speculative upside |
| 🤣 Class D (Memetic) | 0.5x | Meme-driven volatility |
| 🧪 Class E (Experimental) | 0.3x | Extreme risk |

---

## Component 4: Other Risk Factors

**Insider Risk Factor** (0.5 to 1.0):
```
Insider_Risk = 1.0 - (Insider_Score / 100) × 0.5
```
Higher insider control = higher centralization risk.

**Capital Flight Factor**: Based on market cap rank.
- Top 10: 1.2x (blue-chip premium)
- Top 50: 1.0x (neutral)
- Top 100: 0.8x (slight penalty)
- Lower: 0.2x - 0.5x (liquidity penalty)

**Liquidity Risk Factor**: SOL is exempt; others receive 0.90x - 1.0x based on market cap.

---

## The Tariff Rate Formula

The **Tariff Rate** represents transaction friction—a tax on moving between assets. Higher-value assets have lower tariffs.

### Base Tariff Calculation

```
Base_Tariff = max(0, (100 / Real_Valuation_Multiplier) - 10)
```

This formula ensures:
- 10x multiplier → 0% tariff (baseline)
- 5x multiplier → 10% tariff
- 1x multiplier → 90% tariff
- 0.1x multiplier → 990% tariff

### Effective Tariff (with Liquidity Penalty)

Non-SOL tokens receive an additional liquidity penalty:

```
Effective_Tariff = Base_Tariff + Liquidity_Penalty
```

| Market Cap | Liquidity Penalty |
|------------|-------------------|
| ≥ $1B | +10% |
| ≥ $100M | +15% |
| ≥ $10M | +20% |
| < $10M | +25% |

SOL is exempt from the liquidity penalty as the native liquidity hub.

---

## Exchange Price Formula

The **Exchange Price** represents immediate purchasing power after tariff friction:

```
Exchange_Price = Current_Price / (1 + Tariff_Rate / 100)
```

**Example**: A token with 100% tariff:
- Current Price: $10.00
- Exchange Price: $10.00 / (1 + 100/100) = $10.00 / 2.0 = **$5.00**

---

## Tariff Tier System

| Tier | Tariff Range | Multiplier Range | Quality |
|------|--------------|------------------|---------|
| **Premium** | 0% | ≥10.00x | Gold standard |
| **Good** | 1% - 100% | 5.00x - 9.99x | Strong assets |
| **Neutral** | 101% - 500% | 0.20x - 4.99x | Moderate utility |
| **Discounted** | 501% - 1000% | 0.10x - 0.19x | Underperforms |
| **Poor** | 1001% - 5000% | 0.02x - 0.09x | High risk |
| **Catastrophic** | >5000% | <0.02x | Near-worthless |

---

## Worked Example: RENDER vs USDC

### RENDER (AI-Enabled Token)
- Base Type: Protocol Utility → 17.5x midpoint
- AI Category: AI-Enabled → 5.00x timeline multiplier
- Risk Class: Class A → 1.2x
- **Real Multiplier**: ~79.7x
- **Tariff**: max(0, 100/79.7 - 10) + 10 = ~10%

### USDC (Fiat-Pegged Stablecoin)
- Base Type: Fiat-Pegged → 0.03x midpoint
- AI Category: Static → 0.20x timeline multiplier
- Risk Class: Class B → 1.0x
- **Real Multiplier**: ~0.0072x
- **Tariff**: max(0, 100/0.0072 - 10) + 10 = ~13,900%

The math reveals the fundamental truth: **AI-native assets gain value as AI accelerates**, while **static assets—especially fiat-pegged—collapse**.

---

## Key Takeaways

1. **Predicted Price** = Current Price × Real Valuation Multiplier
2. **Multiplier** is a composite of type, AI timeline, risk class, and liquidity factors
3. **Tariff Rate** = max(0, 100/Multiplier - 10) + liquidity penalty
4. **Exchange Price** = Current Price / (1 + Tariff/100)
5. The AI Timeline Factor is the differentiating variable—it's what separates 80x tokens from 0.007x tokens
