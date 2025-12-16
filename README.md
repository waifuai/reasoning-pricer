# Solana Token Analysis Engine

A Python-based analysis tool for Solana tokens that provides risk classification, exchange multiplier calculations, and comprehensive visual reporting.

## Overview

This project analyzes Solana tokens by loading JSON data files, calculating exchange multipliers based on tariff overrides, and organizing tokens by risk classes. It provides both detailed analysis tables and summary statistics to help understand token portfolios.

## Features

- **Risk Classification**: Tokens are categorized into four risk classes:
  - **Class A (Real Yield)**: Lowest risk, established tokens with proven utility
  - **Class B (Systemic)**: Medium-low risk, system-critical tokens
  - **Class C (Venture Risk)**: Medium-high risk, venture-backed or emerging tokens
  - **Class D (Speculative)**: Highest risk, speculative or unproven tokens

- **Exchange Multiplier Calculation**: Automatically calculates multipliers using the formula: `1 + (tariff_override / 100)`

- **Rich Visual Output**: Uses the Rich library to display formatted tables with color-coded risk classes

- **Summary Statistics**: Provides counts and average exchange multipliers by risk class

- **Data Validation**: Built with Pydantic models for robust data validation

## Project Structure

```
pricer/
├── src/
│   ├── main.py          # Main TokenEngine class and entry point
│   └── models.py        # Pydantic models for Token and TokenAnalysis
├── data/                # JSON files containing token data
│   ├── 1.json
│   ├── 2.json
│   └── ...
└── README.md           # This file
```

## Installation

1. **Clone or download the project**
2. **Install dependencies**:
   ```bash
   pip install rich pydantic
   ```

## Usage

### Running the Analysis

Execute the main analysis script:

```bash
python src/main.py
```

Or run the module directly:

```bash
python -m src.main
```

## Data Format

Token data should be provided in JSON files with the following structure:

```json
[
  {
    "symbol": "RAY",
    "name": "Raydium",
    "archetype": "Class B (Systemic)",
    "insider_score": 20,
    "tags": ["dex", "amm", "legacy", "middleware"],
    "tariff_override": 50,
    "reason": "The original Toll Booth. High utility, but tokenomics have historically been highly inflationary. Pure rent-seeking."
  }
]
```

### Field Descriptions

- **symbol**: Token ticker symbol (required)
- **name**: Full token name (required)
- **archetype**: Risk classification (required)
- **insider_score**: Insider sentiment score (0-100, required)
- **tags**: List of descriptive tags (required)
- **tariff_override**: Percentage value for exchange multiplier calculation (required)
- **reason**: Analyst note or justification (required)

## Risk Classes

### Class A (Real Yield) - Green
Tokens that provide real utility and sustainable value. These are typically established protocols with proven track records.

### Class B (Systemic) - Blue
Tokens that are system-critical to the Solana ecosystem. Important infrastructure tokens that may have high utility but also carry systemic risks.

### Class C (Venture Risk) - Orange-Red
Tokens associated with venture-backed projects or emerging protocols. Higher risk but potentially higher reward.

### Class D (Speculative) - Red
Highly speculative tokens with limited track records or those primarily driven by speculation rather than utility.

## Output

The analysis produces:

1. **Summary Statistics**:
   - Total tokens analyzed
   - Token count by risk class
   - Average exchange multipliers by risk class

2. **Detailed Table**:
   - Ticker symbols
   - Risk classifications (color-coded)
   - Insider scores
   - Tariff percentages
   - Exchange rates (multipliers)
   - Analyst notes

## Technical Details

### Dependencies
- **Rich**: For beautiful terminal output and formatting
- **Pydantic**: For data validation and model definition
- **Pathlib**: For cross-platform file path handling

### Key Classes

- **TokenEngine**: Main analysis engine that loads data, creates analysis, and displays results
- **Token**: Pydantic model representing individual tokens
- **TokenAnalysis**: Container for analysis results with sorting and filtering methods
- **RiskClass**: Enum defining the four risk classifications

### Exchange Multiplier Calculation

The exchange multiplier is calculated as:
```
exchange_multiplier = 1 + (tariff_override / 100)
```

For example, a tariff override of 50% results in an exchange multiplier of 1.50x.

## Example Output

```
Solana Token Analysis Engine
==================================================

ANALYSIS SUMMARY
============================================================
Total Tokens Analyzed: 45
Class A (Real Yield): 12 tokens
Class B (Systemic): 18 tokens
Class C (Venture Risk): 10 tokens
Class D (Speculative): 5 tokens

----------------------------------------
AVERAGE EXCHANGE MULTIPLIERS
----------------------------------------
Class A (Real Yield): 1.25x
Class B (Systemic): 1.45x
Class C (Venture Risk): 1.67x
Class D (Speculative): 2.12x
============================================================

[Rich table displaying detailed token analysis]
```

## Customization

### Modifying Risk Classes
Edit the `RiskClass` enum in `src/models.py` to add, remove, or modify risk classifications.

### Adding New Fields
Extend the `Token` model in `src/models.py` to include additional token attributes.

### Customizing Output
Modify the `create_rich_table()` and `display_summary()` methods in `src/main.py` to customize the output format.
