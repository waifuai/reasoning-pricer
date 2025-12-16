#!/usr/bin/env python3
"""
Script to check for duplicate tokens across all JSON files in data/*.json
"""

import json
import os
from pathlib import Path
from collections import defaultdict


def load_tokens_from_json(file_path):
    """Load tokens from a JSON file and return list of token symbols."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        symbols = []
        for token in data:
            if isinstance(token, dict) and 'symbol' in token:
                symbols.append(token['symbol'])
        
        return symbols
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return []


def find_duplicate_tokens(data_dir):
    """Find duplicate tokens across all JSON files in the data directory."""
    data_path = Path(data_dir)
    
    # Find all JSON files
    json_files = sorted(data_path.glob('*.json'))
    
    if not json_files:
        print(f"No JSON files found in {data_dir}")
        return
    
    print(f"Found {len(json_files)} JSON files to analyze")
    
    # Dictionary to track which file contains each token
    token_locations = defaultdict(list)
    all_symbols = []
    
    # Process each JSON file
    for json_file in json_files:
        symbols = load_tokens_from_json(json_file)
        file_symbols = [(symbol, str(json_file)) for symbol in symbols]
        
        for symbol in file_symbols:
            token_locations[symbol[0]].append(symbol[1])
            all_symbols.append(symbol[0])
    
    # Find duplicates
    duplicates = {symbol: locations for symbol, locations in token_locations.items() 
                  if len(locations) > 1}
    
    print(f"\nTotal unique tokens: {len(token_locations)}")
    print(f"Total token entries: {len(all_symbols)}")
    print(f"Duplicate tokens found: {len(duplicates)}")
    
    if duplicates:
        print("\n" + "="*60)
        print("DUPLICATE TOKENS FOUND:")
        print("="*60)
        
        for symbol, locations in sorted(duplicates.items()):
            print(f"\nToken: {symbol}")
            print(f"  Appears in {len(locations)} files:")
            for location in locations:
                print(f"    - {location}")
    else:
        print("\n[SUCCESS] No duplicate tokens found!")
    
    return duplicates


def main():
    """Main function to run the duplicate token checker."""
    data_dir = "data"
    
    print("Checking for duplicate tokens in data/*.json")
    print("-" * 50)
    
    if not os.path.exists(data_dir):
        print(f"Error: Directory '{data_dir}' not found!")
        return
    
    duplicates = find_duplicate_tokens(data_dir)
    
    print("\n" + "="*60)
    print("SUMMARY")
    print("="*60)
    
    if duplicates:
        print(f"[ERROR] Found {len(duplicates)} duplicate tokens")
        print("See details above for which files contain each duplicate")
    else:
        print("[SUCCESS] All tokens are unique across the dataset!")


if __name__ == "__main__":
    main()
