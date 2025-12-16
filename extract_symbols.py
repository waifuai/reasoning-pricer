#!/usr/bin/env python3
"""
Extract ticker symbols from JSON files in the data/ directory
and create a comma-separated list in symbols.txt
"""

import json
import os
import glob

def extract_symbols():
    """Extract all symbols from JSON files in data/ directory"""
    symbols = set()  # Use set to avoid duplicates
    
    # Find all JSON files in data/ directory
    json_files = glob.glob("data/*.json")
    print(f"Found {len(json_files)} JSON files")
    
    for json_file in json_files:
        print(f"Processing {json_file}")
        try:
            with open(json_file, 'r', encoding='utf-8') as f:
                data = json.load(f)
                
                # Handle both single objects and arrays
                if isinstance(data, list):
                    items = data
                elif isinstance(data, dict):
                    items = [data]
                else:
                    print(f"Warning: Unexpected data structure in {json_file}")
                    continue
                
                # Extract symbols
                for item in items:
                    if isinstance(item, dict) and 'symbol' in item:
                        symbol = item['symbol'].strip()
                        if symbol:  # Only add non-empty symbols
                            symbols.add(symbol)
                            
        except Exception as e:
            print(f"Error processing {json_file}: {e}")
            continue
    
    # Sort symbols for consistent output
    sorted_symbols = sorted(symbols)
    
    # Write to symbols.txt as comma-separated values
    with open('symbols.txt', 'w', encoding='utf-8') as f:
        f.write(', '.join(sorted_symbols))
    
    print(f"Extracted {len(sorted_symbols)} unique symbols to symbols.txt")
    print(f"Symbols: {', '.join(sorted_symbols)}")
    
    return sorted_symbols

if __name__ == "__main__":
    extract_symbols()
