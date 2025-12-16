"""
Index generator for creating index files.
"""

from pathlib import Path
from typing import List, Dict
import sys
import os

# Add src to path to import models
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from models import Token, TokenAnalysis
from utils import get_risk_class_filename


class IndexGenerator:
    """Generate index files for navigation."""
    
    def __init__(self, output_dir: str = "reports"):
        """Initialize the index generator."""
        self.output_dir = Path(output_dir)
        self.tokens_dir = self.output_dir / "tokens"
        self.summaries_dir = self.output_dir / "summaries"
    
    def generate_main_index(self, analysis: TokenAnalysis) -> str:
        """Generate main index file."""
        # Calculate overall statistics
        total_tokens = analysis.total_tokens
        class_counts = {
            "Class A (Real Yield)": len(analysis.class_a_tokens),
            "Class B (Systemic)": len(analysis.class_b_tokens),
            "Class C (Venture Risk)": len(analysis.class_c_tokens),
            "Class D (Speculative)": len(analysis.class_d_tokens)
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
- **Average Insider Score:** {sum(t.insider_score for t in analysis.tokens) / len(analysis.tokens):.1f}/100
- **Average Tariff:** {sum(t.tariff_override for t in analysis.tokens) / len(analysis.tokens):.1f}%

## 📁 Report Structure

### By Risk Class
{chr(10).join(f"- [{risk_class}](./summaries/{get_risk_class_filename(risk_class)}) - {count} tokens" for risk_class, count in class_counts.items() if count > 0)}

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
    
    def generate_tokens_index(self, analysis: TokenAnalysis) -> str:
        """Generate index of all token reports."""
        # Group tokens by risk class for organization
        tokens_by_class = {
            "Class A (Real Yield)": analysis.class_a_tokens,
            "Class B (Systemic)": analysis.class_b_tokens,
            "Class C (Venture Risk)": analysis.class_c_tokens,
            "Class D (Speculative)": analysis.class_d_tokens
        }
        
        content = f"""# All Token Reports

Complete list of all {analysis.total_tokens} token analysis reports.

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
    
    def generate_all_indices(self, analysis: TokenAnalysis) -> Dict[str, str]:
        """Generate all index files."""
        print("\n📋 Generating index files...")
        
        main_index = self.generate_main_index(analysis)
        tokens_index = self.generate_tokens_index(analysis)
        
        print(f"  ✓ Main index: {main_index}")
        print(f"  ✓ Tokens index: {tokens_index}")
        
        return {
            "main": main_index,
            "tokens": tokens_index
        }