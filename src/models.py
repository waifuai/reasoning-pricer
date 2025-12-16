#!/usr/bin/env python3
"""
Pydantic models for Solana Token Analysis
Defines the data structure for token information and risk classifications.
"""

from typing import List, Optional
from pydantic import BaseModel
from enum import Enum

class RiskClass(str, Enum):
    """Risk class enumeration with proper ordering."""
    CLASS_A = "Class A (Real Yield)"
    CLASS_B = "Class B (Systemic)"
    CLASS_C = "Class C (Venture Risk)"
    CLASS_D = "Class D (Speculative)"
    
    @classmethod
    def get_sort_order(cls) -> dict:
        """Returns a mapping for sorting by risk class."""
        return {
            cls.CLASS_A: 1,
            cls.CLASS_B: 2,
            cls.CLASS_C: 3,
            cls.CLASS_D: 4
        }

class Token(BaseModel):
    """Pydantic model representing a Solana token."""
    symbol: str
    name: str
    archetype: RiskClass
    insider_score: int
    tags: List[str]
    tariff_override: int
    reason: str
    
    @property
    def exchange_multiplier(self) -> float:
        """Calculate the exchange multiplier: 1 + (tariff_override / 100)."""
        return 1.0 + (self.tariff_override / 100.0)
    
    @property
    def exchange_rate(self) -> str:
        """Format exchange rate as string with 'x' suffix."""
        return f"{self.exchange_multiplier:.2f}x"
    
    @property
    def analyst_note(self) -> str:
        """Extract analyst note from reason field (first sentence)."""
        return self.reason.split('.')[0] + '.'
    
    class Config:
        """Pydantic configuration."""
        use_enum_values = True
        json_encoders = {
            RiskClass: lambda v: v.value
        }

class TokenAnalysis(BaseModel):
    """Container for token analysis results."""
    tokens: List[Token]
    total_tokens: int
    
    @property
    def sorted_tokens(self) -> List[Token]:
        """Return tokens sorted by risk class (A to D)."""
        sort_order = RiskClass.get_sort_order()
        return sorted(self.tokens, key=lambda t: sort_order.get(RiskClass(t.archetype), 999))
    
    @property
    def class_a_tokens(self) -> List[Token]:
        """Return only Class A (Real Yield) tokens."""
        return [t for t in self.tokens if t.archetype == RiskClass.CLASS_A]
    
    @property
    def class_b_tokens(self) -> List[Token]:
        """Return only Class B (Systemic) tokens."""
        return [t for t in self.tokens if t.archetype == RiskClass.CLASS_B]
    
    @property
    def class_c_tokens(self) -> List[Token]:
        """Return only Class C (Venture Risk) tokens."""
        return [t for t in self.tokens if t.archetype == RiskClass.CLASS_C]
    
    @property
    def class_d_tokens(self) -> List[Token]:
        """Return only Class D (Speculative) tokens."""
        return [t for t in self.tokens if t.archetype == RiskClass.CLASS_D]