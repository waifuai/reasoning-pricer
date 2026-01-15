# Tariff Rate Report (2026)

## Overview

This report provides the **Current Price**, the **Exchange Price** (Immediate Purchasing Power after tariff friction), the **Multiplier** (the real valuation factor), and the **Tariff Rate** (the risk-based friction).

Calculations are based on the AI-Acceleration Pricing model defined in `predicted_price.md`.

**Pricing Formula:** `Exchange Price = Current / (1 + Tariff/100)`

All numerical values are shown in **scientific notation** ($X.YeZ$) for precision across all scales.

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

### Tariff Tier Definitions

| Tier | Tariff Rate | Multiplier Range | Description |
|------|-------------|------------------|-------------|
| **Premium** | 0% | ≥10.00x | Gold standard, reserve currencies |
| **Good** | 1% - 100% | 5.00x - 9.99x | Strong assets, commodity-backed |
| **Neutral** | 101% - 500% | 0.20x - 4.99x | Moderate utility, stable value |
| **Discounted** | 501% - 1000% | 0.10x - 0.19x | Underperforms baseline |
| **Poor** | 1001% - 5000% | 0.02x - 0.09x | Weak utility, high risk |
| **Catastrophic** | >5000% | <0.02x | Collapses with fiat, dead assets |

## Price Table

| Symbol | Name | AI Category | Current Price ($) | Exchange Price ($) | Multiplier | Tariff Rate |
|--------|------|-------------|-------------------|--------------------|------------|-------------|
| SOL | Solana | 🔋 Passive Utility | 143.20 | 143.20 | 37.3x | 0% |
| RENDER | Render Network | 🧠 AI-Enabled | 2.43 | 2.21 | 79.7x | 10% |
| WBTC | Wrapped Bitcoin (Portal) | 🗿 Static | 92341.00 | 83568.01 | 9.5x | 10% |
| HNT | Helium Network | 🧠 AI-Enabled | 1.39 | 1.21 | 79.8x | 15% |
| GRASS | Grass | 🧠 AI-Enabled | 0.3468 | 0.3016 | 76.6x | 15% |
| W | Wormhole | 🔋 Passive Utility | 0.0377 | 0.0327 | 58.6x | 15% |
| PYTH | Pyth Network | 🔋 Passive Utility | 0.0687 | 0.0598 | 56.7x | 15% |
| AKT | Akash Network (Wormhole) | 🧠 AI-Enabled | 0.4996 | 0.4344 | 53.9x | 15% |
| RAY | Raydium | 🔋 Passive Utility | 1.19 | 1.03 | 32.3x | 15% |
| KMNO | Kamino | 🔋 Passive Utility | 0.0578 | 0.0503 | 18.9x | 15% |
| 1INCH | 1inch (Wormhole) | 🔋 Passive Utility | 0.1540 | 0.1339 | 17.8x | 15% |
| ORCA | Orca | 🧠 AI-Enabled | 1.20 | 1.00 | 73.7x | 20% |
| ACT | Act I: The AI Prophecy | 🤖 AI-Native | 0.0247 | 0.0206 | 53.7x | 20% |
| HONEY | Hivemapper | 🧠 AI-Enabled | 0.0078 | 0.0065 | 48.9x | 20% |
| MOBILE | Helium Mobile | 🧠 AI-Enabled | 0.00019 | 0.00015 | 48.2x | 20% |
| NOS | Nosana | 🧠 AI-Enabled | 0.2967 | 0.2473 | 48.1x | 20% |
| GMT | STEPN | 🔋 Passive Utility | 0.0191 | 0.0159 | 43.5x | 20% |
| DRIFT | Drift Protocol | 🔋 Passive Utility | 0.1614 | 0.1345 | 21.7x | 20% |
| SYN | Synesis One | 🧠 AI-Enabled | 0.0691 | 0.0576 | 20.7x | 20% |
| NATIX | Natix Network | 🧠 AI-Enabled | 0.00029 | 0.00024 | 20.7x | 20% |
| NEON | Neon EVM | 🔋 Passive Utility | 0.0583 | 0.0486 | 20.6x | 20% |
| ZETA | Zeta Markets | 🔋 Passive Utility | 0.0678 | 0.0565 | 20.0x | 20% |
| DBR | deBridge | 🔋 Passive Utility | 0.0190 | 0.0159 | 18.7x | 20% |
| ZEUS | Zeus Network | 🔋 Passive Utility | 0.0198 | 0.0160 | 7.1x | 24% |
| PRCL | Parcl | 🔋 Passive Utility | 0.0289 | 0.0233 | 7.0x | 24% |
| ACS | Access Protocol | 🔋 Passive Utility | 0.00030 | 0.00024 | 7.0x | 24% |
| FIDA | Bonfida | 🔋 Passive Utility | 0.0357 | 0.0286 | 6.8x | 25% |
| IOT | Helium IOT | 🧠 AI-Enabled | 0.00021 | 0.00017 | 48.2x | 25% |
| KOII | Koii Network | 🧠 AI-Enabled | 0.00017 | 0.00014 | 31.0x | 25% |
| UPT | UpRock | 🧠 AI-Enabled | 0.0064 | 0.0052 | 20.6x | 25% |
| LMR | Lumerin | 🧠 AI-Enabled | 0.00089 | 0.00071 | 20.4x | 25% |
| ATLAS | Star Atlas | 🔋 Passive Utility | 0.00030 | 0.00024 | 15.8x | 25% |
| KIN | Kin | 🔋 Passive Utility | 0.77μ | 0.61μ | 13.6x | 25% |
| FLUXB | FluxBot | 🧠 AI-Enabled | 0.0046 | 0.0037 | 9.4x | 26% |
| ZBCN | Zebec Network | 🔋 Passive Utility | 0.0029 | 0.0023 | 4.8x | 26% |
| GUAC | Guacamole | 🔋 Passive Utility | 0.01μ | 0.01μ | 8.1x | 27% |
| GENE | Genopets | 🔋 Passive Utility | 0.0025 | 0.0020 | 7.9x | 28% |
| HXRO | Hxro Network | 🔋 Passive Utility | 0.0049 | 0.0038 | 7.5x | 28% |
| SLIM | Solanium | 🔋 Passive Utility | 0.0247 | 0.0192 | 7.3x | 29% |
| STEP | Step Finance | 🔋 Passive Utility | 0.0233 | 0.0181 | 7.2x | 29% |
| AURY | Aurory | 🔋 Passive Utility | 0.0712 | 0.0553 | 7.2x | 29% |
| ZIG | Zignaly | 🔋 Passive Utility | 0.0591 | 0.0455 | 5.1x | 30% |
| ABR | Allbridge | 🔋 Passive Utility | 0.0699 | 0.0516 | 4.9x | 36% |
| CHAT | Solchat | 🔋 Passive Utility | 0.0788 | 0.0582 | 4.9x | 36% |
| AART | ALL.ART | 🔋 Passive Utility | 51μ | 37μ | 4.6x | 37% |
| LIQ | Liq Protocol | 🔋 Passive Utility | 0.0649 | 0.0467 | 4.2x | 39% |
| CRP | Cropper | 🔋 Passive Utility | 0.0403 | 0.0290 | 4.1x | 39% |
| YAKU | Yaku | 🔋 Passive Utility | 0.0011 | 0.00081 | 4.1x | 39% |
| DIO | Decimated | 🔋 Passive Utility | 0.00085 | 0.00061 | 3.9x | 41% |
| CYS | Cyclos | 🔋 Passive Utility | 0.4680 | 0.3323 | 3.2x | 41% |
| LIKE | Only1 | 🔋 Passive Utility | 0.0018 | 0.0013 | 3.8x | 41% |
| HAWK | Hawk Protocol | 🔋 Passive Utility | 36μ | 26μ | 3.6x | 42% |
| CRWNY | Crowny | 🔋 Passive Utility | 0.0015 | 0.0010 | 3.6x | 43% |
| SAROS | Saros | 🔋 Passive Utility | 0.0026 | 0.0018 | 3.5x | 44% |
| JLP | Jupiter Perps LP | 🔋 Passive Utility | 4.85 | 3.36 | 2.3x | 44% |
| REAL | Realy Metaverse | 🔋 Passive Utility | 0.0108 | 0.0075 | 3.4x | 44% |
| SBR | Saber | 🔋 Passive Utility | 0.00063 | 0.00043 | 3.3x | 46% |
| OXY | Oxygen | 🔋 Passive Utility | 0.0073 | 0.0049 | 2.9x | 50% |
| SRM | Serum | 🔋 Passive Utility | 0.0110 | 0.0073 | 2.8x | 51% |
| XAUT | Tether Gold (Portal) | 🗿 Static | 4608.07 | 2718.40 | 1.4x | 70% |
| PAXG | Paxos Gold (Portal) | 🗿 Static | 4621.66 | 2724.74 | 1.4x | 70% |
| JTO | Jito | 🔋 Passive Utility | 0.4498 | 0.2582 | 1.4x | 74% |
| DFL | DeFi Land | 🔋 Passive Utility | 50μ | 28μ | 1.5x | 83% |
| AI16Z | ai16z | 🤖 AI-Native | 0.0015 | 0.00080 | 1.4x | 84% |
| MAIL | SolMail | 🔋 Passive Utility | 0.00054 | 0.00029 | 1.4x | 85% |
| WLKN | Walken | 🔋 Passive Utility | 0.00020 | 0.00010 | 1.3x | 91% |
| GARI | Gari Network | 🔋 Passive Utility | 0.0012 | 0.00058 | 1.2x | 99% |
| USDY | Ondo US Dollar Yield | 🔋 Passive Utility | 1.11 | 0.5468 | 1.0x | 103% |
| JUP | Jupiter | 🔋 Passive Utility | 0.2164 | 0.0970 | 0.85x | 123% |
| MNDE | Marinade | 🔋 Passive Utility | 0.0462 | 0.0155 | 0.53x | 199% |
| MPLX | Metaplex | 🔋 Passive Utility | 0.0545 | 0.0143 | 0.37x | 282% |
| RAIN | Rain.fi | 🔋 Passive Utility | 4.39 | 1.07 | 0.34x | 312% |
| GOAT | Goatseus Maximus | 🤖 AI-Native | 0.0388 | 0.0094 | 0.33x | 314% |
| HBB | Hubble Protocol | 🔋 Passive Utility | 0.0048 | 0.0012 | 0.33x | 315% |
| TNSR | Tensor | 🔋 Passive Utility | 0.0712 | 0.0171 | 0.33x | 315% |
| ZEREBRO | Zerebro | 🤖 AI-Native | 0.0193 | 0.0045 | 0.31x | 333% |
| MNGO | Mango Markets | 🔋 Passive Utility | 0.0206 | 0.0038 | 0.23x | 448% |
| POLIS | Star Atlas DAO | 🔋 Passive Utility | 0.0270 | 0.0046 | 0.21x | 483% |
| PRISM | Prism | 🔋 Passive Utility | 0.00063 | 65μ | 0.12x | 869% |
| USDH | Hubble USD | 🔋 Passive Utility | 1.00 | 0.0978 | 0.11x | 922% |
| ISC | International Stable Currency | 🔋 Passive Utility | 2.19 | 0.2142 | 0.11x | 922% |
| UXD | UXD Stablecoin | 🔋 Passive Utility | 1.02 | 0.0859 | 0.09x | 1.1k% |
| BONK | Bonk | 🔋 Passive Utility | 11μ | 0.70μ | 0.07x | 1.5k% |
| WIF | dogwifhat | 🔋 Passive Utility | 0.3941 | 0.0230 | 0.06x | 1.6k% |
| SAMO | Samoyedcoin | 🔋 Passive Utility | 0.00080 | 32μ | 0.04x | 2.4k% |
| POPCAT | Popcat | 🔋 Passive Utility | 0.1008 | 0.0038 | 0.04x | 2.5k% |
| PONKE | Ponke | 🔋 Passive Utility | 0.0566 | 0.0018 | 0.03x | 3.0k% |
| MEW | cat in a dogs world | 🔋 Passive Utility | 0.0010 | 31μ | 0.03x | 3.2k% |
| BOME | BOOK OF MEME | 🔋 Passive Utility | 0.00071 | 21μ | 0.03x | 3.2k% |
| BILLY | Billy | 🔋 Passive Utility | 0.0011 | 16μ | 0.02x | 6.5k% |
| MUMU | Mumu the Bull | 🔋 Passive Utility | 4.3e-10 | 6.5e-12 | 0.02x | 6.6k% |
| MANEKI | Maneki-neko | 🔋 Passive Utility | 0.00055 | 7.9μ | 0.01x | 6.8k% |
| LOCKIN | Lock In | 🔋 Passive Utility | 0.0026 | 38μ | 0.01x | 6.9k% |
| MYRO | Myro | 🔋 Passive Utility | 0.0060 | 83μ | 0.01x | 7.2k% |
| USDC | USD Coin | 🗿 Static | 0.9997 | 0.0071 | 0.0072x | 13.9k% |
| USDT | Tether USD | 🗿 Static | 0.9990 | 0.0068 | 0.0068x | 14.7k% |
| PYUSD | PayPal USD | 🗿 Static | 1.0000 | 0.0029 | 0.0029x | 34.3k% |
| EURC | Euro Coin | 🗿 Static | 1.17 | 0.0033 | 0.0028x | 35.2k% |
| USDS | Sky Dollar (Solana) | 🗿 Static | 1.00 | 0.0027 | 0.0027x | 37.1k% |
| TRYB | BiLira | 🗿 Static | 0.0232 | 23μ | 0.0010x | 102.8k% |
