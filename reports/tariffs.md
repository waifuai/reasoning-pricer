# Comprehensive Tariff Table

## Overview

This document provides a complete reference of all tokens in the Predictive Reasoning Pricer system along with their **effective tariff rates**. The tariff rate externalizes the risk of each asset by showing how much friction is applied when exchanging into higher-quality assets.

### Tariff Philosophy

The tariff system is designed to be intuitive:
- **Higher multiplier** = **Lower tariff** (better asset, less friction)
- **Lower multiplier** = **Higher tariff** (worse asset, more friction)
- **Minimum tariff is 0%** - no negative tariffs allowed

### Calculation Formula

The **effective tariff rate** is calculated from the real valuation multiplier:

```
Effective Tariff Rate = max(0, (100 / Real Valuation Multiplier) - 10)
```

The `max(0, ...)` ensures no negative tariffs - assets can only have 0% or higher tariffs.

This means a 10x multiplier asset has 0% tariff (the baseline). Each 10x reduction in multiplier adds approximately 1000% to the tariff.

### AI Timeline Impact

The pricing model now incorporates AI acceleration based on the 2027 timeline:

| Phase | Year | Static Assets | AI-Native Assets |
|-------|------|---------------|------------------|
| Pre-AI Baseline | 2024 | 25.0x | 3.0x |
| Personal Architect | 2025 Q2 | 15.0x | 8.0x |
| Global Acceleration Accord | 2025 Q4 | 10.0x | 15.0x |
| **Creative Renaissance** | **2026** | **5.0x** | **25.0x** |
| Agent-4 (Great Aligner) | 2027 | 2.0x | 50.0x |

Static assets (BTC, gold) decline as AI progresses, while AI-native/AI-evolving assets appreciate.

### Tariff Tier Definitions

| Tier | Tariff Rate | Multiplier Range | Description |
|------|-------------|------------------|-------------|
| **Premium** | 0% | ≥10.00x | Gold standard, reserve currencies |
| **Good** | 1% - 100% | 5.00x - 9.99x | Strong assets, commodity-backed |
| **Neutral** | 101% - 500% | 0.20x - 4.99x | Moderate utility, stable value |
| **Discounted** | 501% - 1000% | 0.10x - 0.19x | Underperforms baseline |
| **Poor** | 1001% - 5000% | 0.02x - 0.09x | Weak utility, high risk |
| **Catastrophic** | >5000% | <0.02x | Collapses with fiat, dead assets |

---

## Complete Tariff Table

### Tier 1: Premium (0%)

The gold standard assets that achieve 10x or higher multipliers. These have no tariff friction.

| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |
|--------|------|------------|------------|------------------|---------------|------------|-------------|
| SOL | Solana | 🏦 Class B (Systemic) | 37.3x | 0% | 📈 $143.20 → $5348.00 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| RENDER | Render Network | 🛡️ Class A (Real Yield) | 79.7x | 10% | 📈 $2.43 → $193.66 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| HNT | Helium Network | 🛡️ Class A (Real Yield) | 79.8x | 15% | 📈 $1.39 → $110.92 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| GRASS | Grass | 🚀 Class C (Venture Risk) | 76.6x | 15% | 📈 $0.3468 → $26.57 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| W | Wormhole | 🚀 Class C (Venture Risk) | 58.6x | 15% | 📈 $0.0377 → $2.21 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| PYTH | Pyth Network | 🚀 Class C (Venture Risk) | 56.7x | 15% | 📈 $0.0687 → $3.90 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| AKT | Akash Network (Wormhole) | 🛡️ Class A (Real Yield) | 53.9x | 15% | 📈 $0.4996 → $26.91 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| RAY | Raydium | 🏦 Class B (Systemic) | 32.3x | 15% | 📈 $1.19 → $38.46 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| KMNO | Kamino | 🚀 Class C (Venture Risk) | 18.9x | 15% | 📈 $0.0578 → $1.09 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| 1INCH | 1inch (Wormhole) | 🏦 Class B (Systemic) | 17.8x | 15% | 📈 $0.1540 → $2.74 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| ORCA | Orca | 🏦 Class B (Systemic) | 73.7x | 20% | 📈 $1.20 → $88.45 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| ACT | Act I: The AI Prophecy | 🧪 Class E (Experimental) | 53.7x | 20% | 📈 $0.0247 → $1.33 | ⚙️ Protocol Utility | 🤖 AI-Native |
| HONEY | Hivemapper | 🛡️ Class A (Real Yield) | 48.9x | 20% | 📈 $0.0078 → $0.3826 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| MOBILE | Helium Mobile | 🛡️ Class A (Real Yield) | 48.2x | 20% | 📈 $0.00019 → $0.0089 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| NOS | Nosana | 🛡️ Class A (Real Yield) | 48.1x | 20% | 📈 $0.2967 → $14.26 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| GMT | STEPN | 🚀 Class C (Venture Risk) | 43.5x | 20% | 📈 $0.0191 → $0.8316 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| DRIFT | Drift Protocol | 🚀 Class C (Venture Risk) | 21.7x | 20% | 📈 $0.1614 → $3.50 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| SYN | Synesis One | 🛡️ Class A (Real Yield) | 20.7x | 20% | 📈 $0.0691 → $1.43 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| NATIX | Natix Network | 🛡️ Class A (Real Yield) | 20.7x | 20% | 📈 $0.00029 → $0.0060 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| NEON | Neon EVM | 🚀 Class C (Venture Risk) | 20.6x | 20% | 📈 $0.0583 → $1.20 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| ZETA | Zeta Markets | 🚀 Class C (Venture Risk) | 20.0x | 20% | 📈 $0.0678 → $1.35 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| DBR | deBridge | 🚀 Class C (Venture Risk) | 18.7x | 20% | 📈 $0.0190 → $0.3563 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| IOT | Helium IOT | 🛡️ Class A (Real Yield) | 48.2x | 25% | 📈 $0.00021 → $0.0101 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| KOII | Koii Network | 🏦 Class B (Systemic) | 31.0x | 25% | 📈 $0.00017 → $0.0053 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| UPT | UpRock | 🛡️ Class A (Real Yield) | 20.6x | 25% | 📈 $0.0064 → $0.1332 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| LMR | Lumerin | 🛡️ Class A (Real Yield) | 20.4x | 25% | 📈 $0.00089 → $0.0181 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| ATLAS | Star Atlas | 🚀 Class C (Venture Risk) | 15.8x | 25% | 📈 $0.00030 → $0.0048 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| KIN | Kin | 🏦 Class B (Systemic) | 13.6x | 25% | 📈 $0.77μ → $10μ | ⚙️ Protocol Utility | 🔋 Passive Utility |

### Tier 2: Good (1% - 100%)

Strong assets with 5x-9.99x multipliers. Low friction, good store of value.

| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |
|--------|------|------------|------------|------------------|---------------|------------|-------------|
| WBTC | Wrapped Bitcoin (Portal) | 🛡️ Class A (Real Yield) | 9.5x | 10% | 📈 $92341.00 → $8.8e5 | 🪙 Hard Money | 🗿 Static |
| ZEUS | Zeus Network | 🛡️ Class A (Real Yield) | 7.1x | 24% | 📈 $0.0198 → $0.1414 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| PRCL | Parcl | 🚀 Class C (Venture Risk) | 7.0x | 24% | 📈 $0.0289 → $0.2032 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| ACS | Access Protocol | 🚀 Class C (Venture Risk) | 7.0x | 24% | 📈 $0.00030 → $0.0021 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| FIDA | Bonfida | 🛡️ Class A (Real Yield) | 6.8x | 25% | 📈 $0.0357 → $0.2442 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| FLUXB | FluxBot | 🚀 Class C (Speculative) | 9.4x | 26% | 📈 $0.0046 → $0.0436 | ⚙️ Protocol Utility | 🧠 AI-Enabled |
| GUAC | Guacamole | 🛡️ Class A (Real Yield) | 8.1x | 27% | 📈 $0.01μ → $0.10μ | ⚙️ Protocol Utility | 🔋 Passive Utility |
| GENE | Genopets | 🛡️ Class A (Real Yield) | 7.9x | 28% | 📈 $0.0025 → $0.0197 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| HXRO | Hxro Network | 🚀 Class C (Venture Risk) | 7.5x | 28% | 📈 $0.0049 → $0.0365 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| SLIM | Solanium | 🚀 Class C (Venture Risk) | 7.3x | 29% | 📈 $0.0247 → $0.1790 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| STEP | Step Finance | 🛡️ Class A (Real Yield) | 7.2x | 29% | 📈 $0.0233 → $0.1687 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| AURY | Aurory | 🛡️ Class A (Real Yield) | 7.2x | 29% | 📈 $0.0712 → $0.5150 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| ZIG | Zignaly | 🏦 Class B (Systemic) | 5.1x | 30% | 📈 $0.0591 → $0.2989 | ⚙️ Protocol Utility | 🔋 Passive Utility |

### Tier 3: Neutral (101% - 500%)

Assets with 0.20x-4.99x multipliers. Moderate friction, reasonable utility.

| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |
|--------|------|------------|------------|------------------|---------------|------------|-------------|
| ZBCN | Zebec Network | 🏦 Class B (Systemic) | 4.8x | 26% | 📈 $0.0029 → $0.0140 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| ABR | Allbridge | 🏦 Class B (Systemic) | 4.9x | 36% | 📈 $0.0699 → $0.3400 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| CHAT | Solchat | 🏦 Class B (Systemic) | 4.9x | 36% | 📈 $0.0788 → $0.3836 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| AART | ALL.ART | 🏦 Class B (Systemic) | 4.6x | 37% | 📈 $51μ → $0.00023 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| LIQ | Liq Protocol | 🏦 Class B (Systemic) | 4.2x | 39% | 📈 $0.0649 → $0.2729 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| CRP | Cropper | 🚀 Class C (Speculative) | 4.1x | 39% | 📈 $0.0403 → $0.1664 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| YAKU | Yaku | 🚀 Class C (Speculative) | 4.1x | 39% | 📈 $0.0011 → $0.0046 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| DIO | Decimated | 🚀 Class C (Speculative) | 3.9x | 41% | 📈 $0.00085 → $0.0033 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| CYS | Cyclos | 🚀 Class C (Speculative) | 3.2x | 41% | 📈 $0.4680 → $1.52 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| LIKE | Only1 | 🚀 Class C (Speculative) | 3.8x | 41% | 📈 $0.0018 → $0.0070 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| HAWK | Hawk Protocol | 🚀 Class C (Speculative) | 3.6x | 42% | 📈 $36μ → $0.00013 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| CRWNY | Crowny | 🚀 Class C (Speculative) | 3.6x | 43% | 📈 $0.0015 → $0.0052 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| SAROS | Saros | 🚀 Class C (Speculative) | 3.5x | 44% | 📈 $0.0026 → $0.0089 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| JLP | Jupiter Perps LP | 🛡️ Class A (Real Yield) | 2.3x | 44% | 📈 $4.85 → $10.95 | 💸 Real Yield | 🔋 Passive Utility |
| REAL | Realy Metaverse | 🚀 Class C (Speculative) | 3.4x | 44% | 📈 $0.0108 → $0.0367 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| SBR | Saber | 🚀 Class C (Speculative) | 3.3x | 46% | 📈 $0.00063 → $0.0021 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| OXY | Oxygen | 🚀 Class C (Speculative) | 2.9x | 50% | 📈 $0.0073 → $0.0210 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| SRM | Serum | 🚀 Class C (Speculative) | 2.8x | 51% | 📈 $0.0110 → $0.0308 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| XAUT | Tether Gold (Portal) | 🛡️ Class A (Real Yield) | 1.4x | 70% | 📈 $4608.07 → $6628.99 | 🧈 Commodity Backed | 🗿 Static |
| PAXG | Paxos Gold (Portal) | 🛡️ Class A (Real Yield) | 1.4x | 70% | 📈 $4621.66 → $6638.55 | 🧈 Commodity Backed | 🗿 Static |
| JTO | Jito | 🚀 Class C (Venture Risk) | 1.4x | 74% | 📈 $0.4498 → $0.6498 | 🗳️ Governance | 🔋 Passive Utility |
| DFL | DeFi Land | 🧪 Class E (Experimental) | 1.5x | 83% | 📈 $50μ → $74μ | ⚙️ Protocol Utility | 🔋 Passive Utility |
| AI16Z | ai16z | 🧪 Class E (Experimental) | 1.4x | 84% | 📈 $0.0015 → $0.0021 | 🗳️ Governance | 🤖 AI-Native |
| MAIL | SolMail | 🧪 Class E (Experimental) | 1.4x | 85% | 📈 $0.00054 → $0.00077 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| WLKN | Walken | 🧪 Class E (Experimental) | 1.3x | 91% | 📈 $0.00020 → $0.00026 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| GARI | Gari Network | 🧪 Class E (Experimental) | 1.2x | 99% | 📈 $0.0012 → $0.0014 | ⚙️ Protocol Utility | 🔋 Passive Utility |
| USDY | Ondo US Dollar Yield | 🛡️ Class A (Real Yield) | 1.0x | 103% | 📈 $1.11 → $1.13 | 💸 Real Yield | 🔋 Passive Utility |
| JUP | Jupiter | 🚀 Class C (Venture Risk) | 0.85x | 123% | 📉 $0.2164 → $0.1832 | 🗳️ Governance | 🔋 Passive Utility |
| MNDE | Marinade | 🏦 Class B (Systemic) | 0.53x | 199% | 📉 $0.0462 → $0.0245 | 🗳️ Governance | 🔋 Passive Utility |
| MPLX | Metaplex | 🏦 Class B (Systemic) | 0.37x | 282% | 📉 $0.0545 → $0.0200 | 🗳️ Governance | 🔋 Passive Utility |
| RAIN | Rain.fi | 🛡️ Class A (Real Yield) | 0.34x | 312% | 📉 $4.39 → $1.48 | 💸 Real Yield | 🔋 Passive Utility |
| GOAT | Goatseus Maximus | 🧪 Class E (Experimental) | 0.33x | 314% | 📉 $0.0388 → $0.0128 | 🤣 Meme | 🤖 AI-Native |
| HBB | Hubble Protocol | 🛡️ Class A (Real Yield) | 0.33x | 315% | 📉 $0.0048 → $0.0016 | 💸 Real Yield | 🔋 Passive Utility |
| TNSR | Tensor | 🏦 Class B (Systemic) | 0.33x | 315% | 📉 $0.0712 → $0.0233 | 🗳️ Governance | 🔋 Passive Utility |
| ZEREBRO | Zerebro | 🧪 Class E (Experimental) | 0.31x | 333% | 📉 $0.0193 → $0.0060 | 🤣 Meme | 🤖 AI-Native |
| MNGO | Mango Markets | 🚀 Class C (Venture Risk) | 0.23x | 448% | 📉 $0.0206 → $0.0047 | 🗳️ Governance | 🔋 Passive Utility |
| POLIS | Star Atlas DAO | 🚀 Class C (Venture Risk) | 0.21x | 483% | 📉 $0.0270 → $0.0058 | 🗳️ Governance | 🔋 Passive Utility |

### Tier 4: Discounted (501% - 1000%)

Assets with 0.10x-0.19x multipliers. Significant friction, underperforms baseline.

| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |
|--------|------|------------|------------|------------------|---------------|------------|-------------|
| PRISM | Prism | 🚀 Class C (Speculative) | 0.12x | 869% | 📉 $0.00063 → $74μ | 🗳️ Governance | 🔋 Passive Utility |
| USDH | Hubble USD | 🛡️ Class A (Real Yield) | 0.11x | 922% | 📉 $1.00 → $0.1102 | 🛡️ Stable Yield | 🔋 Passive Utility |
| ISC | International Stable Currency | 🛡️ Class A (Real Yield) | 0.11x | 922% | 📉 $2.19 → $0.2414 | 🛡️ Stable Yield | 🔋 Passive Utility |

### Tier 5: Poor (1001% - 5000%)

Assets with 0.02x-0.09x multipliers. High friction, weak utility.

| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |
|--------|------|------------|------------|------------------|---------------|------------|-------------|
| UXD | UXD Stablecoin | 🚀 Class C (Venture Risk) | 0.09x | 1.1k% | 📉 $1.02 → $0.0951 | 🛡️ Stable Yield | 🔋 Passive Utility |
| BONK | Bonk | 🤣 Class D (Memetic) | 0.07x | 1.5k% | 📉 $11μ → $0.75μ | 🤣 Meme | 🔋 Passive Utility |
| WIF | dogwifhat | 🤣 Class D (Memetic) | 0.06x | 1.6k% | 📉 $0.3941 → $0.0245 | 🤣 Meme | 🔋 Passive Utility |
| SAMO | Samoyedcoin | 🤣 Class D (Memetic) | 0.04x | 2.4k% | 📉 $0.00080 → $34μ | 🤣 Meme | 🔋 Passive Utility |
| POPCAT | Popcat | 🤣 Class D (Memetic) | 0.04x | 2.5k% | 📉 $0.1008 → $0.0040 | 🤣 Meme | 🔋 Passive Utility |
| PONKE | Ponke | 🤣 Class D (Memetic) | 0.03x | 3.0k% | 📉 $0.0566 → $0.0019 | 🤣 Meme | 🔋 Passive Utility |
| MEW | cat in a dogs world | 🤣 Class D (Memetic) | 0.03x | 3.2k% | 📉 $0.0010 → $32μ | 🤣 Meme | 🔋 Passive Utility |
| BOME | BOOK OF MEME | 🤣 Class D (Memetic) | 0.03x | 3.2k% | 📉 $0.00071 → $22μ | 🤣 Meme | 🔋 Passive Utility |

### Tier 6: Catastrophic (>5000%)

Assets with multipliers below 0.02x. Maximum friction, near-worthless in collapse.

| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |
|--------|------|------------|------------|------------------|---------------|------------|-------------|
| BILLY | Billy | 🤣 Class D (Memetic) | 0.02x | 6.5k% | 📉 $0.0011 → $16μ | 🤣 Meme | 🔋 Passive Utility |
| MUMU | Mumu the Bull | 🤣 Class D (Memetic) | 0.02x | 6.6k% | 📉 $4.3e-10 → $6.6e-12 | 🤣 Meme | 🔋 Passive Utility |
| MANEKI | Maneki-neko | 🤣 Class D (Memetic) | 0.01x | 6.8k% | 📉 $0.00055 → $8 | 🤣 Meme | 🔋 Passive Utility |
| LOCKIN | Lock In | 🤣 Class D (Memetic) | 0.01x | 6.9k% | 📉 $0.0026 → $39μ | 🤣 Meme | 🔋 Passive Utility |
| MYRO | Myro | 🤣 Class D (Memetic) | 0.01x | 7.2k% | 📉 $0.0060 → $84μ | 🤣 Meme | 🔋 Passive Utility |
| USDC | USD Coin | 🏦 Class B (Systemic) | 0.0072x | 13.9k% | 📉 $0.9997 → $0.0072 | 💵 Fiat Pegged | 🗿 Static |
| USDT | Tether USD | 🏦 Class B (Systemic) | 0.0068x | 14.7k% | 📉 $0.9990 → $0.0068 | 💵 Fiat Pegged | 🗿 Static |
| PYUSD | PayPal USD | 🏦 Class B (Systemic) | 0.0029x | 34.3k% | 📉 $1.0000 → $0.0029 | 💵 Fiat Pegged | 🗿 Static |
| EURC | Euro Coin | 🏦 Class B (Systemic) | 0.0028x | 35.2k% | 📉 $1.17 → $0.0033 | 💵 Fiat Pegged | 🗿 Static |
| USDS | Sky Dollar (Solana) | 🏦 Class B (Systemic) | 0.0027x | 37.1k% | 📉 $1.00 → $0.0027 | 💵 Fiat Pegged | 🗿 Static |
| TRYB | BiLira | 🏦 Class B (Systemic) | 0.0010x | 102.8k% | 📉 $0.0232 → $23μ | 💵 Fiat Pegged | 🗿 Static |
---

## Summary Statistics

### Tariff Distribution by Tier

| Tier | Count | Percentage |
|------|-------|------------|
| Premium (0%) | 28 | 28.0% |
| Good (1%-100%) | 13 | 13.0% |
| Neutral (101%-500%) | 37 | 37.0% |
| Discounted (501%-1000%) | 3 | 3.0% |
| Poor (1001%-5000%) | 8 | 8.0% |
| Catastrophic (>5000%) | 11 | 11.0% |

### Overall Statistics

| Metric | Value |
|--------|-------|
| Total Tokens | 100 |
| Minimum Tariff | 0% |
| Maximum Tariff | 102753% |
| Median Tariff | 41% |
| Average Tariff | 2988% |

### AI Timeline Impact Summary

| Category | Count | Average Multiplier |
|----------|-------|-------------------|
| Static Assets | 9 | 1.38x |
| AI-Native/AI-Enabled | 19 | 38.72x |
| Multiplier Ratio (AI/Static) | - | 28.0x |

---

## Exchange Rate Calculation

To calculate the post-fiat collapse exchange rate between any two tokens:

```
Exchange Rate = Token_A_Multiplier / Token_B_Multiplier
```

Example: Converting USDC to tBTC
- USDC Multiplier: 0.01x
- tBTC Multiplier: 25.00x
- Exchange Rate: 0.01 / 25.00 = 0.0004
- 10,000 USDC → 4 tBTC

---

## Navigation

- [Main Report Index](./index.md)
- [Token Index](./tokens/index.md)
