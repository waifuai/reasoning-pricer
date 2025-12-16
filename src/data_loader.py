"""
Data loader module for loading token data from JSON files.
"""

import json
import os
from pathlib import Path
from typing import List
import sys

# Add src to path to import models
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from models import Token


class DataLoader:
    """Load token data from JSON files."""
    
    def __init__(self, data_dir: str = "data"):
        """Initialize the data loader."""
        self.data_dir = Path(data_dir)
    
    def load_tokens(self) -> List[Token]:
        """Load all tokens from JSON files."""
        print(f"Loading tokens from {self.data_dir}...")
        
        tokens = []
        json_files = list(self.data_dir.glob("*.json"))
        if not json_files:
            raise FileNotFoundError(f"No JSON files found in {self.data_dir}")
        
        for json_file in json_files:
            try:
                with open(json_file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                
                for token_data in data:
                    token = Token(**token_data)
                    tokens.append(token)
                
                print(f"  ✓ Loaded {len(data)} tokens from {json_file.name}")
                
            except Exception as e:
                print(f"  ✗ Failed to load {json_file.name}: {e}")
        
        print(f"Total tokens loaded: {len(tokens)}")
        return tokens