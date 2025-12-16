#!/usr/bin/env python3
"""
Markdown Report Generator for Solana Token Analysis
Generates comprehensive markdown reports that can be browsed on GitHub.
"""

import json
import os
from pathlib import Path
from datetime import datetime
from typing import List, Dict
import sys

# Add src to path to import models
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'src'))

from models import Token, TokenAnalysis, RiskClass

class MarkdownReportGenerator:
    """Generate markdown reports from token analysis data."""
    
    def __init__(self, data_dir: str = "data", output_dir: str = "reports"):
        """Initialize the report generator."""
        self.data_dir = Path(data_dir)
        self.output_dir = Path(output_dir)
        self.tokens_dir = self.output_dir / "tokens"
        self.summaries_dir = self.output_dir / "summaries"
        
        # Ensure output directories exist
        self.tokens_dir.mkdir(exist_ok=True)
        self.summaries_dir.mkdir(exist_ok=True)
        
        self.tokens = []
        self.analysis = None
    
    def load_tokens(self) -> List[Token]:
        """Load all tokens from JSON files."""
        print(f"Loading tokens from {self.data_dir}...")
        
        json_files = list(self.data_dir.glob("*.json"))
        if not json_files:
            raise FileNotFoundError(f"No JSON files found in {self.data_dir}")
        
        for json_file in json_files:
            try:
                with open(json_file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                
                for token_data in data:
                    token = Token(**token_data)
                    self.tokens.append(token)
                
                print(f"  ✓ Loaded {len(data)} tokens from {json_file.name}")
                
            except Exception as e:
                print(f"  ✗ Failed to load {json_file.name}: {e}")
        
        print(f"Total tokens loaded: {len(self.tokens)}")
        return self.tokens
    
    def create_analysis(self) -> TokenAnalysis:
        """Create analysis object from loaded tokens."""
        self.analysis = TokenAnalysis(tokens=self.tokens, total_tokens=len(self.tokens))
        return self.analysis
    
    def generate_token_report(self, token: Token) -> str:
        """Generate individual token report."""
        filename = f"{token.symbol.lower()}.md"
        filepath = self.tokens_dir / filename
        
        content = f"""# {token.symbol} - {token.name}

## Overview

| Field | Value |
|-------|-------|
| **Symbol** | `{token.symbol}` |
| **Name** | {token.name} |
| **Risk Class** | {token.archetype} |
| **Insider Score** | {token.insider_score}/100 |
| **Tariff Override** | {token.tariff_override}% |
| **Exchange Multiplier** | {token.exchange_rate} |

## Risk Analysis

### Archetype
{token.archetype}

### Insider Control Score
**{token.insider_score}/100** - {"Low" if token.insider_score < 30 else "Medium" if token.insider_score < 70 else "High"} insider control

### Exchange Rate Impact
**{token.exchange_rate}** (Tariff: {token.tariff_override}%)

## Tags
{chr(10).join(f"- {tag}" for tag in token.tags)}

## Analyst Notes

{token.reason}

---

## Navigation

- [📊 Risk Class Summary](../summaries/{self.get_risk_class_filename(token.archetype)})
- [🏠 Main Index](../index.md)
- [📈 All Tokens](../tokens/index.md)
"""

        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        
        return filename
    
    def get_risk_class_filename(self, risk_class: str) -> str:
        """Get filename for risk class summary."""
        class_mapping = {
            "Class A (Real Yield)": "class-a-real-yield.md",
            "Class B (Systemic)": "class-b-systemic.md", 
            "Class C (Venture Risk)": "class-c-venture-risk.md",
            "Class D (Speculative)": "class-d-speculative.md"
        }
        return class_mapping.get(risk_class, "unknown.md")
    
    def generate_risk_class_summary(self, risk_class: str, tokens: List[Token]) -> str:
        """Generate summary report for a specific risk class."""
        filename = self.get_risk_class_filename(risk_class)
        filepath = self.summaries_dir / filename
        
        # Calculate statistics
        avg_insider_score = sum(t.insider_score for t in tokens) / len(tokens) if tokens else 0
        avg_tariff = sum(t.tariff_override for t in tokens) / len(tokens) if tokens else 0
        avg_multiplier = sum(t.exchange_multiplier for t in tokens) / len(tokens) if tokens else 0
        
        # Create table rows
        table_rows = []
        for token in sorted(tokens, key=lambda t: t.insider_score):
            table_rows.append(f"| [{token.symbol}](../tokens/{token.symbol.lower()}.md) | {token.name} | {token.insider_score} | {token.tariff_override}% | {token.exchange_rate} |")
        
        content = f"""# {risk_class} - Summary

## Overview

This report summarizes all tokens classified as **{risk_class}**.

### Statistics

| Metric | Value |
|--------|-------|
| **Total Tokens** | {len(tokens)} |
| **Average Insider Score** | {avg_insider_score:.1f}/100 |
| **Average Tariff** | {avg_tariff:.1f}% |
| **Average Exchange Multiplier** | {avg_multiplier:.2f}x |

## Token Details

| Symbol | Name | Insider Score | Tariff | Exchange Rate |
|--------|------|---------------|--------|---------------|
{chr(10).join(table_rows)}

## Risk Characteristics

{"### Class A (Real Yield)" if risk_class == "Class A (Real Yield)" else ""}
{"### Class B (Systemic)" if risk_class == "Class B (Systemic)" else ""}
{"### Class C (Venture Risk)" if risk_class == "Class C (Venture Risk)" else ""}
{"### Class D (Speculative)" if risk_class == "Class D (Speculative)" else ""}

{self.get_risk_class_description(risk_class)}

## Tag Analysis

{self.generate_tag_analysis(tokens)}

---

## Navigation

- [🏠 Main Index](../index.md)
- [📊 All Risk Classes](./)
- [🔗 Individual Token Reports](../tokens/)
"""

        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        
        return filename
    
    def get_risk_class_description(self, risk_class: str) -> str:
        """Get description for risk class."""
        descriptions = {
            "Class A (Real Yield)": """
**Characteristics:**
- Low insider control (typically <30 score)
- Real economic utility and value accrual
- Minimal regulatory/operational risks
- Foundation assets for portfolio construction

**Investment Thesis:** Core holdings with asymmetric risk/reward profiles.""",
            
            "Class B (Systemic)": """
**Characteristics:**
- Medium insider control (30-70 score)
- Important for ecosystem functionality
- Moderate regulatory or operational risks
- Systemic importance creates resilience

**Investment Thesis:** Strategic positions with ecosystem dependencies.""",
            
            "Class C (Venture Risk)": """
**Characteristics:**
- Higher insider control (70-90 score)
- Early-stage or unproven technology
- Significant execution and adoption risks
- High potential upside with substantial risk

**Investment Thesis:** Growth opportunities requiring careful risk management.""",
            
            "Class D (Speculative)": """
**Characteristics:**
- Very high insider control (90+ score)
- Experimental or highly speculative
- Maximum regulatory and execution risks
- High volatility and uncertainty

**Investment Thesis:** Maximum risk/reward for sophisticated investors only."""
        }
        return descriptions.get(risk_class, "No description available.")
    
    def generate_tag_analysis(self, tokens: List[Token]) -> str:
        """Generate tag frequency analysis."""
        tag_counts = {}
        for token in tokens:
            for tag in token.tags:
                tag_counts[tag] = tag_counts.get(tag, 0) + 1
        
        # Sort by frequency
        sorted_tags = sorted(tag_counts.items(), key=lambda x: x[1], reverse=True)
        
        rows = [f"| {tag} | {count} |" for tag, count in sorted_tags]
        
        return f"""| Tag | Frequency |
|------|-----------|
{chr(10).join(rows)}"""
    
    def generate_main_index(self) -> str:
        """Generate main index file."""
        # Calculate overall statistics
        total_tokens = len(self.tokens)
        class_counts = {
            "Class A (Real Yield)": len(self.analysis.class_a_tokens),
            "Class B (Systemic)": len(self.analysis.class_b_tokens),
            "Class C (Venture Risk)": len(self.analysis.class_c_tokens),
            "Class D (Speculative)": len(self.analysis.class_d_tokens)
        }
        
        content = f"""# Solana Token Analysis Reports

## Overview

This repository contains comprehensive analysis reports for {total_tokens} Solana tokens, organized by risk classification and individual token details.

## 📊 Risk Class Summary

| Risk Class | Token Count | Description |
|------------|-------------|-------------|
| [**Class A (Real Yield)**](./summaries/class-a-real-yield.md) | {class_counts["Class A (Real Yield)"]} | Low-risk foundation assets |
| [**Class B (Systemic)**](./summaries/class-b-systemic.md) | {class_counts["Class B (Systemic)"]} | Medium-risk ecosystem tokens |
| [**Class C (Venture Risk)**](./summaries/class-c-venture-risk.md) | {class_counts["Class C (Venture Risk)"]} | Higher-risk growth opportunities |
| [**Class D (Speculative)**](./summaries/class-d-speculative.md) | {class_counts["Class D (Speculative)"]} | Maximum-risk speculative assets |

## 📈 Quick Stats

- **Total Tokens Analyzed:** {total_tokens}
- **Data Source:** JSON files in `data/` directory
- **Risk Distribution:** {"⚖️ Balanced" if max(class_counts.values()) - min(class_counts.values()) < 10 else "📊 Imbalanced"}
- **Average Insider Score:** {sum(t.insider_score for t in self.tokens) / len(self.tokens):.1f}/100
- **Average Tariff:** {sum(t.tariff_override for t in self.tokens) / len(self.tokens):.1f}%

## 📁 Report Structure

### By Risk Class
{chr(10).join(f"- [{risk_class}](./summaries/{self.get_risk_class_filename(risk_class)}) - {count} tokens" for risk_class, count in class_counts.items() if count > 0)}

### Individual Token Reports
- [📋 All Token Reports](./tokens/index.md)

## 🔍 How to Use These Reports

1. **Start with Risk Class Summaries** - Understand the landscape by reviewing each risk class
2. **Drill Down to Individual Tokens** - Click through to detailed token analysis
3. **Use Navigation Links** - Jump between related reports using the navigation sections
4. **Compare Exchange Rates** - Review tariff impacts and exchange multipliers

## 📊 Methodology

### Risk Classification
Tokens are classified into four risk classes based on:
- **Insider Score** (0-100): Degree of insider control
- **Economic Utility**: Real value accrual mechanisms  
- **Regulatory Risk**: Compliance and legal considerations
- **Operational Risk**: Technical and execution risks

### Exchange Rate Calculation
```
Exchange Rate = 1 + (Tariff Override / 100)
```

### Example: 5% tariff = 1.05x exchange rate

---

## 🔗 Quick Links

- [📊 Class A (Real Yield)](./summaries/class-a-real-yield.md)
- [📊 Class B (Systemic)](./summaries/class-b-systemic.md)  
- [📊 Class C (Venture Risk)](./summaries/class-c-venture-risk.md)
- [📊 Class D (Speculative)](./summaries/class-d-speculative.md)
- [📋 All Tokens](./tokens/index.md)

*For questions about methodology or data sources, see the main project documentation.*
"""

        index_path = self.output_dir / "index.md"
        with open(index_path, 'w', encoding='utf-8') as f:
            f.write(content)
        
        return "index.md"
    
    def generate_tokens_index(self) -> str:
        """Generate index of all token reports."""
        # Group tokens by risk class for organization
        tokens_by_class = {
            "Class A (Real Yield)": self.analysis.class_a_tokens,
            "Class B (Systemic)": self.analysis.class_b_tokens,
            "Class C (Venture Risk)": self.analysis.class_c_tokens,
            "Class D (Speculative)": self.analysis.class_d_tokens
        }
        
        content = f"""# All Token Reports

Complete list of all {len(self.tokens)} token analysis reports.

{chr(10).join(self._generate_class_section(risk_class, tokens) for risk_class, tokens in tokens_by_class.items() if tokens)}

---

## Navigation

- [🏠 Main Index](../index.md)
- [📊 Risk Class Summaries](../summaries/)

"""

        index_path = self.tokens_dir / "index.md"
        with open(index_path, 'w', encoding='utf-8') as f:
            f.write(content)
        
        return "index.md"
    
    def _generate_class_section(self, risk_class: str, tokens: List[Token]) -> str:
        """Generate section for a risk class in tokens index."""
        if not tokens:
            return ""
        
        # Sort tokens by symbol
        sorted_tokens = sorted(tokens, key=lambda t: t.symbol)
        
        token_links = []
        for token in sorted_tokens:
            token_links.append(f"- [{token.symbol}](./{token.symbol.lower()}.md) - {token.name}")
        
        return f"""## {risk_class} ({len(tokens)} tokens)

{chr(10).join(token_links)}

"""
    
    def generate_all_reports(self):
        """Generate all markdown reports."""
        print("🚀 Starting markdown report generation...")
        
        # Load data
        self.load_tokens()
        self.create_analysis()
        
        print("\n📄 Generating individual token reports...")
        
        # Generate individual token reports
        token_files = []
        for token in self.tokens:
            filename = self.generate_token_report(token)
            token_files.append(filename)
            print(f"  ✓ {token.symbol} - {token.name}")
        
        print(f"\n📊 Generating risk class summaries...")
        
        # Generate risk class summaries
        summary_files = []
        for risk_class, tokens in [
            ("Class A (Real Yield)", self.analysis.class_a_tokens),
            ("Class B (Systemic)", self.analysis.class_b_tokens),
            ("Class C (Venture Risk)", self.analysis.class_c_tokens),
            ("Class D (Speculative)", self.analysis.class_d_tokens)
        ]:
            if tokens:
                filename = self.generate_risk_class_summary(risk_class, tokens)
                summary_files.append(filename)
                print(f"  ✓ {risk_class} - {len(tokens)} tokens")
        
        print("\n📋 Generating index files...")
        
        # Generate index files
        main_index = self.generate_main_index()
        tokens_index = self.generate_tokens_index()
        
        print(f"  ✓ Main index: {main_index}")
        print(f"  ✓ Tokens index: {tokens_index}")
        
        print(f"\n✅ Report generation complete!")
        print(f"📁 Reports saved to: {self.output_dir}")
        print(f"📊 {len(self.tokens)} token reports generated")
        print(f"📈 {len(summary_files)} risk class summaries generated")
        
        return {
            "tokens": token_files,
            "summaries": summary_files,
            "indices": [main_index, tokens_index]
        }

def main():
    """Main entry point."""
    try:
        generator = MarkdownReportGenerator()
        results = generator.generate_all_reports()
        
        print(f"\n🎉 Success! Generated {len(results['tokens'])} token reports")
        print("Browse the reports in the 'reports/' directory on GitHub!")
        
    except Exception as e:
        print(f"❌ Error: {e}")
        raise

if __name__ == "__main__":
    main()