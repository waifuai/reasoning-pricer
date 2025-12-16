#!/usr/bin/env python3
"""
Main entry point for the Solana Token Analysis Report Generator.
"""

import sys
import os
from report_generator import ReportGenerator


def main():
    """Main entry point."""
    try:
        generator = ReportGenerator()
        results = generator.generate_all_reports()
        
        print(f"\n🎉 Success! Generated {len(results['tokens'])} token reports")
        print("Browse the reports in the 'reports/' directory on GitHub!")
        
        # Print summary statistics
        stats = generator.get_summary_statistics()
        print(f"\n📊 Summary Statistics:")
        for key, value in stats.items():
            print(f"  {key.replace('_', ' ').title()}: {value}")
        
    except Exception as e:
        print(f"❌ Error: {e}")
        raise


if __name__ == "__main__":
    main()