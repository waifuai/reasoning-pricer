"""
Utility functions and constants for report generation.
"""

from typing import List, Dict
from pathlib import Path


# Risk class to filename mapping
RISK_CLASS_MAPPING = {
    "Class A (Real Yield)": "class-a-real-yield.md",
    "Class B (Systemic)": "class-b-systemic.md", 
    "Class C (Venture Risk)": "class-c-venture-risk.md",
    "Class D (Speculative)": "class-d-speculative.md"
}


def get_risk_class_filename(risk_class: str) -> str:
    """Get filename for risk class summary."""
    return RISK_CLASS_MAPPING.get(risk_class, "unknown.md")


def get_risk_class_description(risk_class: str) -> str:
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


def generate_tag_analysis(tokens: List) -> str:
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


def get_insider_score_level(score: int) -> str:
    """Get descriptive level for insider score."""
    if score < 30:
        return "Low"
    elif score < 70:
        return "Medium"
    else:
        return "High"


def calculate_average_insider_score(tokens: List) -> float:
    """Calculate average insider score for a list of tokens."""
    return sum(t.insider_score for t in tokens) / len(tokens) if tokens else 0


def calculate_average_tariff(tokens: List) -> float:
    """Calculate average tariff for a list of tokens."""
    return sum(t.tariff_override for t in tokens) / len(tokens) if tokens else 0


def calculate_average_multiplier(tokens: List) -> float:
    """Calculate average exchange multiplier for a list of tokens."""
    return sum(t.exchange_multiplier for t in tokens) / len(tokens) if tokens else 0


def format_percentage(value: float, decimals: int = 1) -> str:
    """Format a number as a percentage string."""
    return f"{value:.{decimals}f}%"


def format_multiplier(value: float, decimals: int = 2) -> str:
    """Format a number as a multiplier string."""
    return f"{value:.{decimals}f}x"


def ensure_directory_exists(directory: Path) -> None:
    """Ensure a directory exists, creating it if necessary."""
    directory.mkdir(exist_ok=True)