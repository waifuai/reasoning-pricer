#!/usr/bin/env python3
"""
Main entry point for Solana Token Analysis Engine
Loads JSON data, calculates exchange multipliers, sorts by risk class, and displays results.
"""

import json
import glob
from pathlib import Path
from typing import List
from rich.console import Console
from rich.table import Table
from rich.text import Text
from rich import box

from .models import Token, TokenAnalysis, RiskClass

console = Console()

class TokenEngine:
    """Main engine for token analysis and visualization."""
    
    def __init__(self, data_dir: str = "data"):
        """Initialize the token engine with data directory."""
        self.data_dir = Path(data_dir)
        self.console = Console()
    
    def load_json_files(self) -> List[Token]:
        """Load all JSON files from the data directory and return tokens."""
        tokens = []
        json_files = list(self.data_dir.glob("*.json"))
        
        if not json_files:
            raise FileNotFoundError(f"No JSON files found in {self.data_dir}")
        
        console.print(f"Loading data from {len(json_files)} files...", style="cyan")
        
        for json_file in json_files:
            try:
                with open(json_file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                
                for token_data in data:
                    token = Token(**token_data)
                    tokens.append(token)
                
                console.print(f"  [OK] Loaded {len(data)} tokens from {json_file.name}", style="green")
                
            except Exception as e:
                console.print(f"  [ERROR] Failed to load {json_file.name}: {e}", style="red")
        
        console.print(f"Total tokens loaded: {len(tokens)}", style="cyan")
        return tokens
    
    def create_analysis(self, tokens: List[Token]) -> TokenAnalysis:
        """Create token analysis object from list of tokens."""
        return TokenAnalysis(tokens=tokens, total_tokens=len(tokens))
    
    def get_risk_class_color(self, risk_class: str) -> str:
        """Get color for risk class display."""
        color_mapping = {
            "Class A (Real Yield)": "green",
            "Class B (Systemic)": "blue", 
            "Class C (Venture Risk)": "orange_red1",
            "Class D (Speculative)": "red"
        }
        return color_mapping.get(risk_class, "white")
    
    def create_rich_table(self, analysis: TokenAnalysis) -> Table:
        """Create a rich table with token analysis results."""
        table = Table(
            title="Solana Token Risk Analysis",
            box=box.ROUNDED,
            show_header=True,
            header_style="bold cyan"
        )
        
        # Define columns
        table.add_column("Ticker", style="bold cyan", width=12)
        table.add_column("Risk Class", style="bold", width=20)
        table.add_column("Insider Score", justify="center", width=12)
        table.add_column("Tariff %", justify="center", width=10)
        table.add_column("Exchange Rate", justify="center", width=12)
        table.add_column("Analyst Note", width=40)
        
        # Add sorted tokens
        sorted_tokens = analysis.sorted_tokens
        
        for token in sorted_tokens:
            # Format ticker in bold cyan
            ticker_text = Text(token.symbol, style="bold cyan")
            
            # Format risk class with color
            risk_color = self.get_risk_class_color(token.archetype)
            risk_text = Text(token.archetype, style=risk_color)
            
            # Format insider score
            insider_score = str(token.insider_score)
            
            # Format tariff percentage
            tariff = f"{token.tariff_override}%"
            
            # Format exchange rate
            exchange_rate = token.exchange_rate
            
            # Format analyst note (truncate if too long)
            note = token.analyst_note
            if len(note) > 35:
                note = note[:32] + "..."
            
            table.add_row(
                ticker_text,
                risk_text,
                insider_score,
                tariff,
                exchange_rate,
                note
            )
        
        return table
    
    def display_summary(self, analysis: TokenAnalysis):
        """Display analysis summary."""
        console.print("\n" + "="*60, style="bold")
        console.print("ANALYSIS SUMMARY", style="bold cyan")
        console.print("="*60, style="bold")
        
        console.print(f"Total Tokens Analyzed: {analysis.total_tokens}", style="cyan")
        
        class_counts = {
            "Class A (Real Yield)": len(analysis.class_a_tokens),
            "Class B (Systemic)": len(analysis.class_b_tokens),
            "Class C (Venture Risk)": len(analysis.class_c_tokens),
            "Class D (Speculative)": len(analysis.class_d_tokens)
        }
        
        for risk_class, count in class_counts.items():
            color = self.get_risk_class_color(risk_class)
            console.print(f"{risk_class}: {count} tokens", style=color)
        
        # Calculate average exchange multiplier by class
        console.print("\n" + "-"*40, style="dim")
        console.print("AVERAGE EXCHANGE MULTIPLIERS", style="bold")
        console.print("-"*40, style="dim")
        
        for risk_class, tokens in [
            ("Class A (Real Yield)", analysis.class_a_tokens),
            ("Class B (Systemic)", analysis.class_b_tokens),
            ("Class C (Venture Risk)", analysis.class_c_tokens),
            ("Class D (Speculative)", analysis.class_d_tokens)
        ]:
            if tokens:
                avg_multiplier = sum(t.exchange_multiplier for t in tokens) / len(tokens)
                color = self.get_risk_class_color(risk_class)
                console.print(f"{risk_class}: {avg_multiplier:.2f}x", style=color)
        
        console.print("="*60, style="bold")
    
    def run_analysis(self):
        """Run the complete token analysis."""
        try:
            console.print("Solana Token Analysis Engine", style="bold cyan")
            console.print("="*50, style="bold")
            
            # Load data
            tokens = self.load_json_files()
            
            # Create analysis
            analysis = self.create_analysis(tokens)
            
            # Display summary
            self.display_summary(analysis)
            
            # Create and display table
            table = self.create_rich_table(analysis)
            console.print("\n")
            console.print(table)
            
            console.print("\nAnalysis complete!", style="bold green")
            
        except Exception as e:
            console.print(f"ERROR: Error during analysis: {e}", style="bold red")
            raise

def main():
    """Main entry point."""
    engine = TokenEngine()
    engine.run_analysis()

if __name__ == "__main__":
    main()