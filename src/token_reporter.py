"""
Token report generator for individual token analysis reports.
"""

from pathlib import Path
from typing import List
import sys
import os

# Add src to path to import models
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from models import Token
from utils import get_risk_class_filename, get_insider_score_level


class TokenReporter:
    """Generate individual token reports."""
    
    def __init__(self, tokens_dir: str = "reports/tokens"):
        """Initialize the token reporter."""
        self.tokens_dir = Path(tokens_dir)
        # Ensure output directory exists
        self.tokens_dir.mkdir(parents=True, exist_ok=True)
    
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
**{token.insider_score}/100** - {get_insider_score_level(token.insider_score)} insider control

### Exchange Rate Impact
**{token.exchange_rate}** (Tariff: {token.tariff_override}%)

## Tags
{chr(10).join(f"- {tag}" for tag in token.tags)}

## Analyst Notes

{token.reason}

---

## Navigation

- [📊 Risk Class Summary](../summaries/{get_risk_class_filename(token.archetype)})
- [🏠 Main Index](../index.md)
- [📈 All Tokens](../tokens/index.md)
"""

        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        
        return filename
    
    def generate_multiple_reports(self, tokens: List[Token]) -> List[str]:
        """Generate reports for multiple tokens."""
        print("📄 Generating individual token reports...")
        
        token_files = []
        for token in tokens:
            filename = self.generate_token_report(token)
            token_files.append(filename)
            print(f"  ✓ {token.symbol} - {token.name}")
        
        return token_files