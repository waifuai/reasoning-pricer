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
| RENDER | Render Network | Class A (Real Yield) | 144.84x | 0% | $1.00 → $144.84 | Protocol Utility | AI-Enabled |
| HNT | Helium Network | Class A (Real Yield) | 143.44x | 0% | $1.00 → $143.44 | Protocol Utility | AI-Enabled |
| ORCA | Orca | Class B (Systemic) | 136.45x | 0% | $1.00 → $136.45 | Protocol Utility | AI-Enabled |
| IO | io.net | Class C (Venture Risk) | 133.88x | 0% | $1.00 → $133.88 | Protocol Utility | AI-Enabled |
| GRASS | Grass | Class C (Venture Risk) | 120.49x | 0% | $1.00 → $120.49 | Protocol Utility | AI-Enabled |
| WIFI | WiFi Map | Class A (Real Yield) | 109.33x | 0% | $1.00 → $109.33 | Protocol Utility | AI-Enabled |
| AKT | Akash Network (Wormhole) | Class A (Real Yield) | 104.96x | 0% | $1.00 → $104.96 | Protocol Utility | AI-Enabled |
| ALEPH | Aleph.im | Class A (Real Yield) | 100.58x | 0% | $1.00 → $100.58 | Protocol Utility | AI-Enabled |
| HONEY | Hivemapper | Class A (Real Yield) | 100.58x | 0% | $1.00 → $100.58 | Protocol Utility | AI-Enabled |
| SHDW | Shadow Token | Class A (Real Yield) | 97.96x | 0% | $1.00 → $97.96 | Protocol Utility | AI-Enabled |
| NOS | Nosana | Class A (Real Yield) | 96.21x | 0% | $1.00 → $96.21 | Protocol Utility | AI-Enabled |
| MOBILE | Helium Mobile | Class A (Real Yield) | 94.46x | 0% | $1.00 → $94.46 | Protocol Utility | AI-Enabled |
| IOT | Helium IOT | Class A (Real Yield) | 94.46x | 0% | $1.00 → $94.46 | Protocol Utility | AI-Enabled |
| KOII | Koii Network | Class B (Systemic) | 56.48x | 0% | $1.00 → $56.48 | Protocol Utility | AI-Enabled |
| LMR | Lumerin | Class A (Real Yield) | 41.98x | 0% | $1.00 → $41.98 | Protocol Utility | AI-Enabled |
| UPT | UpRock | Class A (Real Yield) | 40.23x | 0% | $1.00 → $40.23 | Protocol Utility | AI-Enabled |
| SYN | Synesis One | Class A (Real Yield) | 39.36x | 0% | $1.00 → $39.36 | Protocol Utility | AI-Enabled |
| NATIX | Natix Network | Class A (Real Yield) | 39.36x | 0% | $1.00 → $39.36 | Protocol Utility | AI-Enabled |
| ONDE | Onde Network | Class A (Real Yield) | 37.78x | 0% | $1.00 → $37.78 | Protocol Utility | AI-Enabled |
| ACT | Act I: The AI Prophecy | Class E (Experimental) | 35.10x | 0% | $1.00 → $35.10 | Protocol Utility | AI-Native |
| PYTH | Pyth Network | Class C (Venture Risk) | 32.14x | 0% | $1.00 → $32.14 | Protocol Utility | Passive Utility |
| W | Wormhole | Class C (Venture Risk) | 32.14x | 0% | $1.00 → $32.14 | Protocol Utility | Passive Utility |
| MEDIA | Media Network | Class B (Systemic) | 24.56x | 0% | $1.00 → $24.56 | Protocol Utility | AI-Enabled |
| GMT | STEPN | Class C (Venture Risk) | 24.11x | 0% | $1.00 → $24.11 | Protocol Utility | Passive Utility |
| UPC | UpRock | Class B (Systemic) | 22.59x | 0% | $1.00 → $22.59 | Protocol Utility | AI-Enabled |
| SOL | Solana | Class B (Systemic) | 20.15x | 0% | $1.00 → $20.15 | Protocol Utility | Passive Utility |
| RAY | Raydium | Class B (Systemic) | 18.90x | 0% | $1.00 → $18.90 | Protocol Utility | Passive Utility |
| FLUXB | FluxBot | Class C (Speculative) | 17.97x | 0% | $1.00 → $17.97 | Protocol Utility | AI-Enabled |
| wETH | Wrapped Ether (Portal) | Class B (Systemic) | 12.98x | 0% | $1.00 → $12.98 | Wrapped Bridge | Passive Utility |
| SNS | Synesis One | Class E (Experimental) | 12.67x | 0% | $1.00 → $12.67 | Protocol Utility | AI-Enabled |
| MET | Meteora | Class A (Real Yield) | 12.60x | 0% | $1.00 → $12.60 | Protocol Utility | Passive Utility |
| DRIFT | Drift Protocol | Class C (Venture Risk) | 12.56x | 0% | $1.00 → $12.56 | Protocol Utility | Passive Utility |
| SHRAP | Shrapnel | Class C (Venture Risk) | 12.56x | 0% | $1.00 → $12.56 | Protocol Utility | Passive Utility |
| CLOUD | Sanctum | Class A (Real Yield) | 11.81x | 0% | $1.00 → $11.81 | Protocol Utility | Passive Utility |
| ZETA | Zeta Markets | Class C (Venture Risk) | 11.05x | 0% | $1.00 → $11.05 | Protocol Utility | Passive Utility |
| NEON | Neon EVM | Class C (Venture Risk) | 11.03x | 0% | $1.00 → $11.03 | Protocol Utility | Passive Utility |
| DBR | deBridge | Class C (Venture Risk) | 11.03x | 0% | $1.00 → $11.03 | Protocol Utility | Passive Utility |
| wLDO | Wrapped Lido DAO (Portal) | Class B (Systemic) | 10.75x | 0% | $1.00 → $10.75 | Wrapped Bridge | Passive Utility |
| WBTC | Wrapped Bitcoin (Portal) | Class A (Real Yield) | 10.58x | 0% | $1.00 → $10.58 | Hard Money | Static |
| KMNO | Kamino | Class C (Venture Risk) | 10.04x | 0% | $1.00 → $10.04 | Protocol Utility | Passive Utility |

### Tier 2: Good (1% - 100%)

Strong assets with 5x-9.99x multipliers. Low friction, good store of value.

| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |
|--------|------|------------|------------|------------------|---------------|------------|-------------|
| 1INCH | 1inch (Wormhole) | Class B (Systemic) | 9.92x | 0% | $1.00 → $9.92 | Protocol Utility | Passive Utility |
| wUNI | Wrapped Uniswap (Portal) | Class A (Real Yield) | 9.73x | 0% | $1.00 → $9.73 | Wrapped Bridge | Passive Utility |
| wBNB | Wrapped BNB (Portal) | Class B (Systemic) | 9.43x | 1% | $1.00 → $9.43 | Wrapped Bridge | Passive Utility |
| SLND | Solend (Save) | Class B (Systemic) | 9.21x | 1% | $1.00 → $9.21 | Protocol Utility | Passive Utility |
| ATLAS | Star Atlas | Class C (Venture Risk) | 9.04x | 1% | $1.00 → $9.04 | Protocol Utility | Passive Utility |
| zBTC | Zeus Bitcoin | Class A (Real Yield) | 8.57x | 2% | $1.00 → $8.57 | Hard Money | Static |
| tBTC | Threshold Bitcoin | Class A (Real Yield) | 8.40x | 2% | $1.00 → $8.40 | Hard Money | Static |
| KIN | Kin | Class B (Systemic) | 8.19x | 2% | $1.00 → $8.19 | Protocol Utility | Passive Utility |
| wLINK | Wrapped Chainlink (Portal) | Class B (Systemic) | 7.30x | 4% | $1.00 → $7.30 | Wrapped Bridge | Passive Utility |
| wMATIC | Wrapped Matic (Portal) | Class B (Systemic) | 6.72x | 5% | $1.00 → $6.72 | Wrapped Bridge | Passive Utility |
| wAVAX | Wrapped AVAX (Portal) | Class B (Systemic) | 6.72x | 5% | $1.00 → $6.72 | Wrapped Bridge | Passive Utility |
| wNEAR | Wrapped NEAR (Portal) | Class C (Speculative) | 6.03x | 7% | $1.00 → $6.03 | Wrapped Bridge | Passive Utility |
| wCRV | Wrapped Curve DAO (Portal) | Class B (Systemic) | 5.95x | 7% | $1.00 → $5.95 | Wrapped Bridge | Passive Utility |
| GENE | Genopets | Class A (Real Yield) | 5.29x | 9% | $1.00 → $5.29 | Protocol Utility | Passive Utility |
| DUAL | Dual Finance | Class A (Real Yield) | 5.25x | 9% | $1.00 → $5.25 | Protocol Utility | Passive Utility |
| PICA | Picasso | Class A (Real Yield) | 5.25x | 9% | $1.00 → $5.25 | Protocol Utility | Passive Utility |
| GUAC | Guacamole | Class A (Real Yield) | 5.25x | 9% | $1.00 → $5.25 | Protocol Utility | Passive Utility |
| LFG | Less Fn Gas | Class D (Speculative) | 5.25x | 9% | $1.00 → $5.25 | Protocol Utility | Passive Utility |

### Tier 3: Neutral (101% - 500%)

Assets with 0.20x-4.99x multipliers. Moderate friction, reasonable utility.

| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |
|--------|------|------------|------------|------------------|---------------|------------|-------------|
| PHOENIX | Phoenix (Ellipsis) | Class A (Real Yield) | 4.91x | 10% | $1.00 → $4.91 | Protocol Utility | Passive Utility |
| STEP | Step Finance | Class A (Real Yield) | 4.83x | 11% | $1.00 → $4.83 | Protocol Utility | Passive Utility |
| AURY | Aurory | Class A (Real Yield) | 4.73x | 11% | $1.00 → $4.73 | Protocol Utility | Passive Utility |
| STRM | Streamflow | Class A (Real Yield) | 4.62x | 12% | $1.00 → $4.62 | Protocol Utility | Passive Utility |
| ZEUS | Zeus Network | Class A (Real Yield) | 4.54x | 12% | $1.00 → $4.54 | Protocol Utility | Passive Utility |
| HXRO | Hxro Network | Class C (Venture Risk) | 4.41x | 13% | $1.00 → $4.41 | Protocol Utility | Passive Utility |
| FIDA | Bonfida | Class A (Real Yield) | 4.35x | 13% | $1.00 → $4.35 | Protocol Utility | Passive Utility |
| TULIP | Tulip Protocol | Class B (Systemic) | 4.25x | 14% | $1.00 → $4.25 | Protocol Utility | Passive Utility |
| cbBTC | Coinbase Wrapped BTC | Class B (Systemic) | 4.16x | 14% | $1.00 → $4.16 | Hard Money | Static |
| PRCL | Parcl | Class C (Venture Risk) | 4.02x | 15% | $1.00 → $4.02 | Protocol Utility | Passive Utility |
| ACS | Access Protocol | Class C (Venture Risk) | 4.02x | 15% | $1.00 → $4.02 | Protocol Utility | Passive Utility |
| SLIM | Solanium | Class C (Venture Risk) | 4.02x | 15% | $1.00 → $4.02 | Protocol Utility | Passive Utility |
| OPEN | OpenBook | Class B (Systemic) | 3.28x | 21% | $1.00 → $3.28 | Protocol Utility | Passive Utility |
| HADES | Hadeswap | Class C (Speculative) | 3.00x | 23% | $1.00 → $3.00 | Protocol Utility | Passive Utility |
| ZIG | Zignaly | Class B (Systemic) | 2.98x | 24% | $1.00 → $2.98 | Protocol Utility | Passive Utility |
| ABR | Allbridge | Class B (Systemic) | 2.95x | 24% | $1.00 → $2.95 | Protocol Utility | Passive Utility |
| CHAT | Solchat | Class B (Systemic) | 2.95x | 24% | $1.00 → $2.95 | Protocol Utility | Passive Utility |
| SONAR | SonarWatch | Class B (Systemic) | 2.92x | 24% | $1.00 → $2.92 | Protocol Utility | Passive Utility |
| RIN | Aldrin | Class B (Systemic) | 2.71x | 27% | $1.00 → $2.71 | Protocol Utility | Passive Utility |
| MEAN | Mean DAO | Class B (Systemic) | 2.71x | 27% | $1.00 → $2.71 | Protocol Utility | Passive Utility |
| IVN | Investin | Class B (Systemic) | 2.71x | 27% | $1.00 → $2.71 | Protocol Utility | Passive Utility |
| CRP | Cropper | Class C (Speculative) | 2.70x | 27% | $1.00 → $2.70 | Protocol Utility | Passive Utility |
| ZBCN | Zebec Network | Class B (Systemic) | 2.69x | 27% | $1.00 → $2.69 | Protocol Utility | Passive Utility |
| GRAPE | Grape Network | Class B (Systemic) | 2.69x | 27% | $1.00 → $2.69 | Protocol Utility | Passive Utility |
| AART | ALL.ART | Class B (Systemic) | 2.69x | 27% | $1.00 → $2.69 | Protocol Utility | Passive Utility |
| DUST | Dust Protocol | Class C (Speculative) | 2.67x | 27% | $1.00 → $2.67 | Protocol Utility | Passive Utility |
| YAKU | Yaku | Class C (Speculative) | 2.67x | 27% | $1.00 → $2.67 | Protocol Utility | Passive Utility |
| CAVE | CaveWorld | Class C (Speculative) | 2.48x | 30% | $1.00 → $2.48 | Protocol Utility | Passive Utility |
| FCON | Space Falcon | Class C (Speculative) | 2.46x | 31% | $1.00 → $2.46 | Protocol Utility | Passive Utility |
| DIO | Decimated | Class C (Speculative) | 2.46x | 31% | $1.00 → $2.46 | Protocol Utility | Passive Utility |
| GOFX | GooseFX | Class C (Speculative) | 2.46x | 31% | $1.00 → $2.46 | Protocol Utility | Passive Utility |
| HAWK | Hawk Protocol | Class C (Speculative) | 2.46x | 31% | $1.00 → $2.46 | Protocol Utility | Passive Utility |
| SOLR | SolRazr | Class C (Speculative) | 2.46x | 31% | $1.00 → $2.46 | Protocol Utility | Passive Utility |
| LARIX | Larix | Class C (Speculative) | 2.43x | 31% | $1.00 → $2.43 | Protocol Utility | Passive Utility |
| SOLC | SolCard | Class C (Speculative) | 2.43x | 31% | $1.00 → $2.43 | Protocol Utility | Passive Utility |
| LIKE | Only1 | Class C (Speculative) | 2.43x | 31% | $1.00 → $2.43 | Protocol Utility | Passive Utility |
| CRWNY | Crowny | Class C (Speculative) | 2.41x | 31% | $1.00 → $2.41 | Protocol Utility | Passive Utility |
| LIQ | Liq Protocol | Class B (Systemic) | 2.40x | 32% | $1.00 → $2.40 | Protocol Utility | Passive Utility |
| PIP | Pip | Class B (Systemic) | 2.36x | 32% | $1.00 → $2.36 | Protocol Utility | Passive Utility |
| SBR | Saber | Class C (Speculative) | 2.16x | 36% | $1.00 → $2.16 | Protocol Utility | Passive Utility |
| REAL | Realy Metaverse | Class C (Speculative) | 2.16x | 36% | $1.00 → $2.16 | Protocol Utility | Passive Utility |
| YAW | Yawww | Class C (Speculative) | 2.16x | 36% | $1.00 → $2.16 | Protocol Utility | Passive Utility |
| SAROS | Saros | Class C (Speculative) | 2.14x | 37% | $1.00 → $2.14 | Protocol Utility | Passive Utility |
| BLOCK | Blockasset | Class C (Speculative) | 2.14x | 37% | $1.00 → $2.14 | Protocol Utility | Passive Utility |
| MBS | MonkeyLeague | Class C (Speculative) | 2.14x | 37% | $1.00 → $2.14 | Protocol Utility | Passive Utility |
| CYS | Cyclos | Class C (Speculative) | 2.14x | 37% | $1.00 → $2.14 | Protocol Utility | Passive Utility |
| GST | Green Satoshi Token | Class E (Experimental) | 1.91x | 42% | $1.00 → $1.91 | Protocol Utility | Passive Utility |
| SOLS | SPL20 Inscriptions | Class E (Experimental) | 1.91x | 42% | $1.00 → $1.91 | Protocol Utility | Passive Utility |
| SRM | Serum | Class C (Speculative) | 1.88x | 43% | $1.00 → $1.88 | Protocol Utility | Passive Utility |
| GH0ST | Gh0st | Class E (Experimental) | 1.86x | 44% | $1.00 → $1.86 | Protocol Utility | Passive Utility |
| MAPS | Maps.me | Class C (Speculative) | 1.85x | 44% | $1.00 → $1.85 | Protocol Utility | Passive Utility |
| OXY | Oxygen | Class C (Speculative) | 1.85x | 44% | $1.00 → $1.85 | Protocol Utility | Passive Utility |
| BERN | BonkEarn | Class E (Experimental) | 1.67x | 50% | $1.00 → $1.67 | Protocol Utility | Passive Utility |
| BOZO | Bozo Hybrid | Class E (Experimental) | 1.67x | 50% | $1.00 → $1.67 | Protocol Utility | Passive Utility |
| DFL | DeFi Land | Class E (Experimental) | 1.65x | 50% | $1.00 → $1.65 | Protocol Utility | Passive Utility |
| XAUT | Tether Gold (Portal) | Class A (Real Yield) | 1.56x | 54% | $1.00 → $1.56 | Commodity Backed | Static |
| ZERO | Analysoor | Class E (Experimental) | 1.55x | 55% | $1.00 → $1.55 | Protocol Utility | Passive Utility |
| SNY | Synthetify | Class E (Experimental) | 1.54x | 55% | $1.00 → $1.54 | Protocol Utility | Passive Utility |
| PAXG | Paxos Gold (Portal) | Class A (Real Yield) | 1.51x | 56% | $1.00 → $1.51 | Commodity Backed | Static |
| MAIL | SolMail | Class E (Experimental) | 1.51x | 56% | $1.00 → $1.51 | Protocol Utility | Passive Utility |
| ROPE | Rope | Class E (Experimental) | 1.36x | 63% | $1.00 → $1.36 | Protocol Utility | Passive Utility |
| WLKN | Walken | Class E (Experimental) | 1.35x | 64% | $1.00 → $1.35 | Protocol Utility | Passive Utility |
| COPE | Cope | Class E (Experimental) | 1.35x | 64% | $1.00 → $1.35 | Protocol Utility | Passive Utility |
| JLP | Jupiter Perps LP | Class A (Real Yield) | 1.32x | 66% | $1.00 → $1.32 | Real Yield | Passive Utility |
| GARI | Gari Network | Class E (Experimental) | 1.13x | 78% | $1.00 → $1.13 | Protocol Utility | Passive Utility |
| JitoSOL | Jito Staked SOL | Class B (Systemic) | 0.96x | 94% | $1.00 → $0.96 | Liquid Staking | Passive Utility |
| mSOL | Marinade Staked SOL | Class B (Systemic) | 0.96x | 94% | $1.00 → $0.96 | Liquid Staking | Passive Utility |
| AI16Z | ai16z | Class E (Experimental) | 0.88x | 103% | $1.00 → $0.88 | Governance | AI-Native |
| bSOL | Blaze Staked SOL | Class B (Systemic) | 0.77x | 120% | $1.00 → $0.77 | Liquid Staking | Passive Utility |
| INF | Sanctum Infinity | Class B (Systemic) | 0.77x | 120% | $1.00 → $0.77 | Liquid Staking | Passive Utility |
| JTO | Jito | Class C (Venture Risk) | 0.76x | 122% | $1.00 → $0.76 | Governance | Passive Utility |
| LFNTY | Lifinity | Class A (Real Yield) | 0.66x | 142% | $1.00 → $0.66 | Real Yield | Passive Utility |
| USDY | Ondo US Dollar Yield | Class A (Real Yield) | 0.61x | 153% | $1.00 → $0.61 | Real Yield | Passive Utility |
| OUSG | Ondo Short-Term US Gov | Class A (Real Yield) | 0.61x | 153% | $1.00 → $0.61 | Real Yield | Passive Utility |
| jSOL | JPool Staked SOL | Class B (Systemic) | 0.48x | 198% | $1.00 → $0.48 | Liquid Staking | Passive Utility |
| hSOL | Helius Staked SOL | Class B (Systemic) | 0.48x | 198% | $1.00 → $0.48 | Liquid Staking | Passive Utility |
| JUP | Jupiter | Class C (Venture Risk) | 0.47x | 201% | $1.00 → $0.47 | Governance | Passive Utility |
| MNDE | Marinade | Class B (Systemic) | 0.31x | 311% | $1.00 → $0.31 | Governance | Passive Utility |
| CROWN | Photo Finish LIVE | Class A (Real Yield) | 0.26x | 369% | $1.00 → $0.26 | Real Yield | Passive Utility |
| RAIN | Rain.fi | Class A (Real Yield) | 0.23x | 431% | $1.00 → $0.23 | Real Yield | Passive Utility |
| FRKT | Frakt | Class A (Real Yield) | 0.23x | 431% | $1.00 → $0.23 | Real Yield | Passive Utility |
| HBB | Hubble Protocol | Class A (Real Yield) | 0.22x | 435% | $1.00 → $0.22 | Real Yield | Passive Utility |
| MPLX | Metaplex | Class B (Systemic) | 0.21x | 468% | $1.00 → $0.21 | Governance | Passive Utility |
| GOAT | Goatseus Maximus | Class E (Experimental) | 0.21x | 469% | $1.00 → $0.21 | Meme | AI-Native |
| ZEREBRO | Zerebro | Class E (Experimental) | 0.21x | 469% | $1.00 → $0.21 | Meme | AI-Native |

### Tier 4: Discounted (501% - 1000%)

Assets with 0.10x-0.19x multipliers. Significant friction, underperforms baseline.

| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |
|--------|------|------------|------------|------------------|---------------|------------|-------------|
| daosol | daoSOL | Class B (Systemic) | 0.19x | 511% | $1.00 → $0.19 | Liquid Staking | Passive Utility |
| TNSR | Tensor | Class B (Systemic) | 0.19x | 524% | $1.00 → $0.19 | Governance | Passive Utility |
| ME | Magic Eden | Class B (Systemic) | 0.19x | 530% | $1.00 → $0.19 | Governance | Passive Utility |
| MNGO | Mango Markets | Class C (Venture Risk) | 0.13x | 782% | $1.00 → $0.13 | Governance | Passive Utility |
| POLIS | Star Atlas DAO | Class C (Venture Risk) | 0.13x | 782% | $1.00 → $0.13 | Governance | Passive Utility |

### Tier 5: Poor (1001% - 5000%)

Assets with 0.02x-0.09x multipliers. High friction, weak utility.

| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |
|--------|------|------------|------------|------------------|---------------|------------|-------------|
| DEAN | Dean's List | Class B (Systemic) | 0.09x | 1069% | $1.00 → $0.09 | Governance | Passive Utility |
| FORGE | Blocksmith Labs Forge | Class C (Speculative) | 0.08x | 1181% | $1.00 → $0.08 | Governance | Passive Utility |
| SUNNY | Sunny Aggregator | Class C (Speculative) | 0.08x | 1273% | $1.00 → $0.08 | Governance | Passive Utility |
| PORT | Port Finance | Class C (Speculative) | 0.08x | 1297% | $1.00 → $0.08 | Governance | Passive Utility |
| TAPS | Tap Fantasy | Class C (Speculative) | 0.08x | 1297% | $1.00 → $0.08 | Governance | Passive Utility |
| PRISM | Prism | Class C (Speculative) | 0.08x | 1297% | $1.00 → $0.08 | Governance | Passive Utility |
| ISC | International Stable Currency | Class A (Real Yield) | 0.08x | 1313% | $1.00 → $0.08 | StableYield | Passive Utility |
| PAR | Parallel | Class A (Real Yield) | 0.07x | 1338% | $1.00 → $0.07 | StableYield | Passive Utility |
| USDH | Hubble USD | Class A (Real Yield) | 0.07x | 1419% | $1.00 → $0.07 | StableYield | Passive Utility |
| BONK | Bonk | Class D (Memetic) | 0.05x | 1898% | $1.00 → $0.05 | Meme | Passive Utility |
| UXD | UXD Stablecoin | Class C (Venture Risk) | 0.05x | 1926% | $1.00 → $0.05 | StableYield | Passive Utility |
| WIF | dogwifhat | Class D (Memetic) | 0.05x | 2090% | $1.00 → $0.05 | Meme | Passive Utility |
| SLERF | Slerf | Class D (Memetic) | 0.03x | 2957% | $1.00 → $0.03 | Meme | Passive Utility |
| SAMO | Samoyedcoin | Class D (Memetic) | 0.03x | 3043% | $1.00 → $0.03 | Meme | Passive Utility |
| WEN | Wen | Class D (Memetic) | 0.03x | 3072% | $1.00 → $0.03 | Meme | Passive Utility |
| POPCAT | Popcat | Class D (Memetic) | 0.03x | 3318% | $1.00 → $0.03 | Meme | Passive Utility |
| BOME | BOOK OF MEME | Class D (Memetic) | 0.02x | 4073% | $1.00 → $0.02 | Meme | Passive Utility |
| PONKE | Ponke | Class D (Memetic) | 0.02x | 4111% | $1.00 → $0.02 | Meme | Passive Utility |
| MEW | cat in a dogs world | Class D (Memetic) | 0.02x | 4150% | $1.00 → $0.02 | Meme | Passive Utility |

### Tier 6: Catastrophic (>5000%)

Assets with multipliers below 0.02x. Maximum friction, near-worthless in collapse.

| Symbol | Name | Risk Class | Multiplier | Effective Tariff | Exchange Rate | Token Type | AI Category |
|--------|------|------------|------------|------------------|---------------|------------|-------------|
| SC | Shark Cat | Class D (Memetic) | 0.01x | 7549% | $1.00 → $0.01 | Meme | Passive Utility |
| BILLY | Billy | Class D (Memetic) | 0.01x | 8309% | $1.00 → $0.01 | Meme | Passive Utility |
| MANEKI | Maneki-neko | Class D (Memetic) | 0.01x | 8388% | $1.00 → $0.01 | Meme | Passive Utility |
| MUMU | Mumu the Bull | Class D (Memetic) | 0.01x | 8388% | $1.00 → $0.01 | Meme | Passive Utility |
| MICHI | Michi | Class D (Memetic) | 0.01x | 8388% | $1.00 → $0.01 | Meme | Passive Utility |
| ANALOS | Analos | Class D (Memetic) | 0.01x | 9035% | $1.00 → $0.01 | Meme | Passive Utility |
| LOCKIN | Lock In | Class D (Memetic) | 0.01x | 9035% | $1.00 → $0.01 | Meme | Passive Utility |
| CWIF | Catwifhat | Class E (Experimental) | 0.01x | 9060% | $1.00 → $0.01 | Meme | Passive Utility |
| MYRO | Myro | Class D (Memetic) | 0.01x | 9121% | $1.00 → $0.01 | Meme | Passive Utility |
| SILLY | Silly Dragon | Class D (Memetic) | 0.01x | 9121% | $1.00 → $0.01 | Meme | Passive Utility |
| LOAF | Loaf Cat | Class E (Experimental) | 0.01x | 10947% | $1.00 → $0.01 | Meme | Passive Utility |
| USDC | USD Coin | Class B (Systemic) | 0.01x | 14687% | $1.00 → $0.01 | FiatPegged | Static |
| USDT | Tether USD | Class B (Systemic) | 0.01x | 14687% | $1.00 → $0.01 | FiatPegged | Static |
| EURC | Euro Coin | Class B (Systemic) | 0.00x | 35263% | $1.00 → $0.00 | FiatPegged | Static |
| PYUSD | PayPal USD | Class B (Systemic) | 0.00x | 35263% | $1.00 → $0.00 | FiatPegged | Static |
| USDS | Sky Dollar (Solana) | Class B (Systemic) | 0.00x | 35263% | $1.00 → $0.00 | FiatPegged | Static |
| TRYB | BiLira | Class B (Systemic) | 0.00x | 99874% | $1.00 → $0.00 | FiatPegged | Static |
| BRZ | BRZ Token | Class B (Systemic) | 0.00x | 99874% | $1.00 → $0.00 | FiatPegged | Static |
---

## Summary Statistics

### Tariff Distribution by Tier

| Tier | Count | Percentage |
|------|-------|------------|
| Premium (0%) | 40 | 21.6% |
| Good (1%-100%) | 18 | 9.7% |
| Neutral (101%-500%) | 85 | 45.9% |
| Discounted (501%-1000%) | 5 | 2.7% |
| Poor (1001%-5000%) | 19 | 10.3% |
| Catastrophic (>5000%) | 18 | 9.7% |

### Overall Statistics

| Metric | Value |
|--------|-------|
| Total Tokens | 185 |
| Minimum Tariff | 0% |
| Maximum Tariff | 99874% |
| Median Tariff | 31% |
| Average Tariff | 2621% |

### AI Timeline Impact Summary

| Category | Count | Average Multiplier |
|----------|-------|-------------------|
| Static Assets | 13 | 2.68x |
| AI-Native/AI-Enabled | 27 | 68.41x |
| Multiplier Ratio (AI/Static) | - | 25.6x |

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
