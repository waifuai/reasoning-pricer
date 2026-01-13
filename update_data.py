import json
import glob
import urllib.request
import time
import os

print("Starting update_data.py...")

# 1. Read all local tokens
token_files = {} # Symbol -> [file_paths]
all_symbols = set()

# Get all json files
files = glob.glob('data/*.json')
print(f"Found {len(files)} JSON files in data/")

for fpath in files:
    try:
        with open(fpath, 'r', encoding='utf-8') as f:
            data = json.load(f)
            for token in data:
                symbol = token['symbol']
                all_symbols.add(symbol)
    except Exception as e:
        print(f"Error reading {fpath}: {e}")

print(f"Found {len(all_symbols)} unique tokens.")

# 2. Map symbols to Coingecko IDs
print("Fetching coin list from Coingecko...")
symbol_to_id = {}
try:
    url = "https://api.coingecko.com/api/v3/coins/list"
    req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})
    with urllib.request.urlopen(req) as response:
        all_coins = json.load(response)
        print(f"Fetched {len(all_coins)} coins from Coingecko.")
except Exception as e:
    print(f"Failed to fetch coin list: {e}")
    # Proceed? No, can't map.
    exit(1)

# Manual overrides for accurate mapping (focus on Solana ecosystem)
overrides = {
    "SOL": "solana",
    "USDC": "usd-coin",
    "USDT": "tether",
    "BTC": "bitcoin",
    "ETH": "ethereum",
    "JUP": "jupiter-exchange-solana",
    "BONK": "bonk",
    "WIF": "dogwifcoin",
    "PYTH": "pyth-network",
    "JTO": "jito-governance-token",
    "RENDER": "render-token",
    "HNT": "helium",
    "RAY": "raydium",
    "ORCA": "orca",
    "MSOL": "msol",
    "BSOL": "blazestake-staked-sol",
    "JITOSOL": "jito-staked-sol",
    "W": "wormhole",
    "TNSR": "tensor",
    "KMNO": "kamino",
    "PRCL": "parcl",
    "CLOUD": "sanctum-cloud",
    "ZEUS": "zeus-network",
    "DRIFT": "drift-protocol",
    "IO": "io-net",
    "ZETA": "zeta", # ambiguous
    "POPCAT": "popcat",
    "BOME": "book-of-meme",
    "MEW": "cat-in-a-dogs-world",
    "SLERF": "slerf",
    "WEN": "wen",
    "MYRO": "myro",
    "PONKE": "ponke",
    "SILLY": "silly-dragon",
    "MANEKI": "maneki",
    "GARI": "gari-network",
    "GMT": "stepn",
    "ATLAS": "star-atlas",
    "POLIS": "star-atlas-dao",
    "AURY": "aurory",
    "SHRAP": "shrapnel",
    "ACS": "access-protocol",
    "GENE": "genopets",
    "CROWN": "photofinish-live",
    "WBTC": "wrapped-bitcoin",
    "TBTC": "tbtc",
    "USDY": "ondo-us-dollar-yield",
    "OUSG": "ondo-short-term-us-government-bond",
    "PAXG": "pax-gold",
    "XAUT": "tether-gold",
    "RLB": "rollbit-coin",
    "SAMO": "samoyedcoin",
    "BERN": "bonk-earn",
    "COPE": "cope",
    "ROPE": "rope-token",
    "FIDA": "bonfida",
    "SRM": "serum",
    "MAPS": "maps",
    "OXY": "oxygen",
    "KIN": "kin",
    "HONEY": "hivemapper",
    "MOBILE": "helium-mobile",
    "IOT": "helium-iot",
    "GUAC": "guacamole",
    "CHAT": "solchat",
    "LMR": "lumerin",
    "NOS": "nosana",
    "SHDW": "shadow-token",
    "AKT": "akash-network",
    "ALEPH": "aleph-im",
    "ZIG": "zignaly",
    "LCX": "lcx",
    "MPLX": "metaplex",
    "HADES": "hades",
    "DUST": "dust-protocol",
    "FORGE": "forge",
    "RAIN": "rain-coin",
    "STEP": "step-finance",
    "MNGO": "mango-markets",
    "SLND": "solend",
    "TULIP": "solfarm",
    "MNDE": "marinade",
    "LFNTY": "lifinity",
    "ABR": "allbridge",
    "GRASS": "grass",
    "UPC": "upc",
    "ACT": "act-i-the-ai-prophecy",
    "GOAT": "goatseus-maximus",
    "AI16Z": "ai16z",
    "ZEREBRO": "zerebro",
    "GRAPE": "grape-network",
    "ANALOS": "analos",
}

# Build map: Symbol -> ID
for coin in all_coins:
    s = coin['symbol'].upper()
    i = coin['id']
    
    if s in overrides:
        continue # Use override ID later

    if s in all_symbols:
        # If we saw this symbol in our files
        if s not in symbol_to_id:
            symbol_to_id[s] = i
        else:
            # Duplicate symbol in Coingecko.
            # Heuristic: Prefer IDs that contain the symbol name or are shorter?
            # Or just skip/keep first.
            # Usually the first one in the list is the oldest/main one? Unclear.
            pass

# Apply overrides
for s, i in overrides.items():
    symbol_to_id[s] = i

ids_to_fetch = list(set(symbol_to_id.values()) & set([c['id'] for c in all_coins])) # Valid IDs only?
# Actually overrides might specify IDs that exist.
ids_to_fetch = []
for s in all_symbols:
    if s in symbol_to_id:
        ids_to_fetch.append(symbol_to_id[s])

ids_to_fetch = list(set(ids_to_fetch))
print(f"Mapped {len(ids_to_fetch)} symbols to Coingecko IDs.")

# 3. Fetch Market Data in batches
print("Fetching market data...")
chunk_size = 200 # max 250
market_data = {} # ID -> {mcap, fdv}

for i in range(0, len(ids_to_fetch), chunk_size):
    chunk = ids_to_fetch[i:i+chunk_size]
    ids_str = ",".join(chunk)
    print(f"Fetching batch {i}...")
    url = f"https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids={ids_str}&order=market_cap_desc&sparkline=false"
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})
        with urllib.request.urlopen(req) as response:
            data = json.load(response)
            for item in data:
                cid = item['id']
                market_data[cid] = {
                    "market_cap": item.get('market_cap'),
                    "fdv": item.get('fully_diluted_valuation'),
                    "launch_date": item.get('atl_date') # Rough proxy if needed, but keeping logic
                }
        time.sleep(1.5)
    except Exception as e:
        print(f"Error fetching batch {i}: {e}")

# 4. Update files
print("Updating files...")
updated_count = 0
for fpath in files:
    with open(fpath, 'r', encoding='utf-8') as f:
        tokens = json.load(f)
    
    file_updated = False
    for token in tokens:
        s = token['symbol']
        cid = symbol_to_id.get(s)
        
        if cid and cid in market_data:
            data = market_data[cid]
            if data['market_cap'] is not None:
                token['market_cap'] = data['market_cap']
                file_updated = True
            if data['fdv'] is not None:
                token['fdv'] = data['fdv']
                file_updated = True
            # Skipping launch_date for now as it's not reliable from markets endpoint
            
    if file_updated:
        with open(fpath, 'w', encoding='utf-8') as f:
            json.dump(tokens, f, indent=2)
            updated_count += 1
            print(f"Updated {fpath}")

print(f"Done. Updated {updated_count} files.")
