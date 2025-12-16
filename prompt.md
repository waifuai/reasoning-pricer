# LLM Prompt: Generate Solana Token Data Files

## Task Overview
Generate additional JSON data files containing Solana token information that are NOT present in the excluded symbols list. Create realistic, well-structured token data that follows the established format and provides diverse token classifications.

## Excluded Symbols (DO NOT USE)
The following symbols are already processed and must be excluded from new data:
1INCH, AART, ABR, ACS, ACT, AI16Z, AKT, ALEPH, ANALOS, ATLAS, AURY, BERN, BILLY, BLOCK, BOME, BONK, BOZO, BRZ, CAVE, CHAT, CLOUD, COPE, CROWN, CRP, CRWNY, CWIF, CYS, DBR, DEAN, DFL, DIO, DRIFT, DUAL, DUST, EURC, FCON, FIDA, FLUXB, FORGE, FRKT, GARI, GENE, GH0ST, GMT, GOAT, GOFX, GRAPE, GRASS, GST, GUAC, HADES, HAWK, HBB, HNT, HONEY, HXRO, INF, IO, IOT, ISC, IVN, JLP, JTO, JUP, JitoSOL, KIN, KMNO, KOII, LARIX, LFG, LFNTY, LIKE, LIQ, LMR, LOAF, LOCKIN, MAIL, MANEKI, MAPS, MBS, ME, MEAN, MEDIA, MET, MEW, MICHI, MNDE, MNGO, MOBILE, MPLX, MUMU, MYRO, NATIX, NEON, NOS, ONDE, OPEN, ORCA, OUSG, OXY, PAR, PAXG, PHOENIX, PICA, PIP, POLIS, PONKE, POPCAT, PORT, PRCL, PRISM, PYTH, PYUSD, RAIN, RAY, REAL, RENDER, RIN, ROPE, SAMO, SAROS, SBR, SC, SHDW, SHRAP, SILLY, SLERF, SLIM, SLND, SNS, SNY, SOL, SOLC, SOLR, SOLS, SONAR, SRM, STEP, STRM, SUNNY, SYN, TAPS, TNSR, TRYB, TULIP, UPC, UPT, USDC, USDH, USDS, USDT, USDY, UXD, W, WBTC, WEN, WIF, WIFI, WLKN, XAUT, YAKU, YAW, ZBCN, ZEREBRO, ZERO, ZETA, ZEUS, ZIG, bSOL, cbBTC, daosol, hSOL, jSOL, mSOL, tBTC, wAVAX, wBNB, wCRV, wETH, wLDO, wLINK, wMATIC, wNEAR, wUNI, zBTC

## Output Requirements

### File Structure
- Create multiple JSON files named sequentially (e.g., data/11.json, data/12.json, etc.)
- Each file should contain 8-12 token entries
- Follow the exact JSON structure shown in the example

### Token Data Format
Each token must include these exact fields:
```json
{
  "symbol": "TOKEN_SYMBOL",
  "name": "Token Full Name",
  "archetype": "Class X (Description)",
  "insider_score": 0-100,
  "tags": ["tag1", "tag2", "tag3", "tag4"],
  "tariff_override": 0-15,
  "reason": "Detailed explanation of token's characteristics and scoring rationale"
}
```

### Archetype Categories
Use these archetype classifications:
- Class A (Real Yield) - Productive assets with genuine utility
- Class B (Systemic) - Infrastructure tokens with network effects
- Class C (Speculative) - High-risk, high-reward tokens
- Class D (Memetic) - Community-driven tokens
- Class E (Experimental) - Novel concepts and innovations

### Insider Score Guidelines
- 0-10: Minimal insider control, highly decentralized
- 11-25: Low insider influence
- 26-50: Moderate insider presence
- 51-75: Significant insider control
- 76-90: Heavy insider dominance
- 91-100: Maximum insider control, centralized

### Tag Categories
Use relevant tags from these categories:
**Asset Types:** rwa, defi, gaming, infrastructure, meme, ai, data, privacy, identity, payments, lending, derivatives, insurance
**Technical:** bridged, synthetic, derivatives, staking, governance, liquidity, amm, lending-borrowing, cross-chain
**Risk Profile:** hard-money, custodial, decentralized, volatile, stable, blue-chip, experimental
**Business Model:** fee-generating, yield-bearing, reserve-backed, algorithmic, incentive-driven
**Regulatory:** compliant, institutional, retail, offshore, licensed

### Tariff Override Guidelines
- 0-2: Premium tokens with minimal friction
- 3-5: Standard tokens with moderate friction
- 6-8: Higher friction for additional risk
- 9-12: Significant friction for elevated risk
- 13-15: Maximum friction for highest risk tokens

## Content Guidelines

### Token Diversity
Generate tokens across different categories:
- DeFi protocols and governance tokens
- Gaming and metaverse assets
- AI and data infrastructure
- Real-world asset tokenizations
- Privacy and identity solutions
- Cross-chain bridges and infrastructure
- Memetic and community tokens
- Experimental financial instruments

### Quality Standards
- Ensure all token symbols are unique and not in the excluded list
- Provide realistic token names that match their symbols
- Create believable tag combinations
- Write comprehensive reasoning for each scoring decision
- Maintain consistency within token categories

### Examples of Good Token Entries

#### Real Yield Token (Class A)
```json
{
  "symbol": "FLEX",
  "name": "Flexible Yield Protocol",
  "archetype": "Class A (Real Yield)",
  "insider_score": 8,
  "tags": ["defi", "yield-bearing", "fee-generating", "liquidity"],
  "tariff_override": 2,
  "reason": "Automated yield optimization protocol with transparent fee structure. Low insider control with community governance."
}
```

#### Systemic Token (Class B)
```json
{
  "symbol": "ORBIT",
  "name": "Solana Oracle Network",
  "archetype": "Class B (Systemic)",
  "insider_score": 35,
  "tags": ["infrastructure", "data", "governance", "oracle"],
  "tariff_override": 5,
  "reason": "Critical oracle infrastructure for Solana ecosystem. Moderate insider control balanced with decentralized validator network."
}
```

## Output Format
Return your response as properly formatted JSON arrays, with each file clearly labeled. Ensure all generated symbols are checked against the excluded list to guarantee uniqueness.

## Quality Assurance
Before finalizing, verify that:
1. No excluded symbols are used
2. All required fields are present
3. Insider scores align with reasoning
4. Tags accurately describe the token
5. Tariff overrides match risk assessments
6. JSON syntax is valid
7. Token diversity is maintained across files