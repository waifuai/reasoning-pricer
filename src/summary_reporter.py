"""
Summary report generator for risk class summaries.
"""

from pathlib import Path
from typing import List
import sys
import os

# Add src to path to import models
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from models import Token
from utils import (
    get_risk_class_filename, 
    get_risk_class_description, 
    generate_tag_analysis,
    calculate_average_insider_score,
    calculate_average_tariff,
    calculate_average_multiplier,
    format_percentage,
    format_multiplier
)


class SummaryReporter:
    """Generate risk class summary reports."""
    
    def __init__(self, summaries_dir: str = "reports/summaries"):
        """Initialize the summary reporter."""
        self.summaries_dir = Path(summaries_dir)
        # Ensure output directory exists
        self.summaries_dir.mkdir(parents=True, exist_ok=True)
    
    def generate_risk_class_summary(self, risk_class: str, tokens: List[Token]) -> str:
        """Generate summary report for a specific risk class."""
        filename = get_risk_class_filename(risk_class)
        filepath = self.summaries_dir / filename
        
        # Calculate statistics
        avg_insider_score = calculate_average_insider_score(tokens)
        avg_tariff = calculate_average_tariff(tokens)
        avg_multiplier = calculate_average_multiplier(tokens)
        
        # Create table rows
        table_rows = []
        for token in sorted(tokens, key=lambda t: t.insider_score):
            table_rows.append(f"| [{token.symbol}](../tokens/{token.symbol.lower()}.md) | {token.name} | {token.insider_score} | {format_percentage(token.tariff_override)} | {format_multiplier(token.exchange_multiplier)} |")
        
        content = f"""# {risk_class} - Summary

## Overview

This report summarizes all tokens classified as **{risk_class}**.

### Statistics

| Metric | Value |
|--------|-------|
| **Total Tokens** | {len(tokens)} |
| **Average Insider Score** | {avg_insider_score:.1f}/100 |
| **Average Tariff** | {format_percentage(avg_tariff)} |
| **Average Exchange Multiplier** | {format_multiplier(avg_multiplier)} |

## Token Details

| Symbol | Name | Insider Score | Tariff | Exchange Rate |
|--------|------|---------------|--------|---------------|
{chr(10).join(table_rows)}

## Risk Characteristics

{"### Class A (Real Yield)" if risk_class == "Class A (Real Yield)" else ""}
{"### Class B (Systemic)" if risk_class == "Class B (Systemic)" else ""}
{"### Class C (Venture Risk)" if risk_class == "Class C (Venture Risk)" else ""}
{"### Class D (Speculative)" if risk_class == "Class D (Speculative)" else ""}

{get_risk_class_description(risk_class)}

## Tag Analysis

{generate_tag_analysis(tokens)}

---

## Navigation

- [🏠 Main Index](../index.md)
- [📊 All Risk Classes](./)
- [🔗 Individual Token Reports](../tokens/)
"""

        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        
        return filename
    
    def generate_multiple_summaries(self, risk_class_data: List[tuple]) -> List[str]:
        """Generate summaries for multiple risk classes."""
        print("\n📊 Generating risk class summaries...")
        
        summary_files = []
        for risk_class, tokens in risk_class_data:
            if tokens:
                filename = self.generate_risk_class_summary(risk_class, tokens)
                summary_files.append(filename)
                print(f"  ✓ {risk_class} - {len(tokens)} tokens")
        
        return summary_files