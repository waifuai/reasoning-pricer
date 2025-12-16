"""
Main report generator that orchestrates all report generation components.
"""

from typing import List, Dict
import sys
import os

# Add src to path to import models
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from models import Token, TokenAnalysis
from data_loader import DataLoader
from token_reporter import TokenReporter
from summary_reporter import SummaryReporter
from index_generator import IndexGenerator


class ReportGenerator:
    """Main orchestrator for report generation."""
    
    def __init__(self, data_dir: str = "../data", output_dir: str = "../reports"):
        """Initialize the report generator."""
        self.data_dir = data_dir
        self.output_dir = output_dir
        
        # Initialize components
        self.data_loader = DataLoader(data_dir)
        self.token_reporter = TokenReporter(f"{output_dir}/tokens")
        self.summary_reporter = SummaryReporter(f"{output_dir}/summaries")
        self.index_generator = IndexGenerator(output_dir)
        
        # Data containers
        self.tokens = []
        self.analysis = None
    
    def load_data(self) -> List[Token]:
        """Load token data from JSON files."""
        print("🚀 Starting markdown report generation...")
        print("Loading data...")
        
        self.tokens = self.data_loader.load_tokens()
        self.analysis = TokenAnalysis(tokens=self.tokens, total_tokens=len(self.tokens))
        
        return self.tokens
    
    def generate_token_reports(self) -> List[str]:
        """Generate individual token reports."""
        return self.token_reporter.generate_multiple_reports(self.tokens)
    
    def generate_summary_reports(self) -> List[str]:
        """Generate risk class summary reports."""
        risk_class_data = [
            ("Class A (Real Yield)", self.analysis.class_a_tokens),
            ("Class B (Systemic)", self.analysis.class_b_tokens),
            ("Class C (Venture Risk)", self.analysis.class_c_tokens),
            ("Class D (Speculative)", self.analysis.class_d_tokens)
        ]
        
        return self.summary_reporter.generate_multiple_summaries(risk_class_data)
    
    def generate_index_files(self) -> Dict[str, str]:
        """Generate index files."""
        return self.index_generator.generate_all_indices(self.analysis)
    
    def generate_all_reports(self) -> Dict[str, List[str]]:
        """Generate all markdown reports."""
        # Load data
        self.load_data()
        
        # Generate individual token reports
        token_files = self.generate_token_reports()
        
        # Generate risk class summaries
        summary_files = self.generate_summary_reports()
        
        # Generate index files
        index_files = self.generate_index_files()
        
        print(f"\n✅ Report generation complete!")
        print(f"📁 Reports saved to: {self.output_dir}")
        print(f"📊 {len(self.tokens)} token reports generated")
        print(f"📈 {len(summary_files)} risk class summaries generated")
        
        return {
            "tokens": token_files,
            "summaries": summary_files,
            "indices": index_files
        }
    
    def get_summary_statistics(self) -> Dict[str, str]:
        """Get summary statistics of generated reports."""
        if not self.tokens:
            return {"error": "No data loaded"}
        
        total_tokens = len(self.tokens)
        class_counts = {
            "Class A (Real Yield)": len(self.analysis.class_a_tokens),
            "Class B (Systemic)": len(self.analysis.class_b_tokens),
            "Class C (Venture Risk)": len(self.analysis.class_c_tokens),
            "Class D (Speculative)": len(self.analysis.class_d_tokens)
        }
        
        return {
            "total_tokens": str(total_tokens),
            "class_a_count": str(class_counts["Class A (Real Yield)"]),
            "class_b_count": str(class_counts["Class B (Systemic)"]),
            "class_c_count": str(class_counts["Class C (Venture Risk)"]),
            "class_d_count": str(class_counts["Class D (Speculative)"]),
            "average_insider_score": f"{sum(t.insider_score for t in self.tokens) / len(self.tokens):.1f}",
            "average_tariff": f"{sum(t.tariff_override for t in self.tokens) / len(self.tokens):.1f}%"
        }