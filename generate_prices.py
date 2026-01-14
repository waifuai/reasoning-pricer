import json
import glob
import os
import re

def parse_tariffs():
    tariffs = {}
    tariff_file = 'reports/tariffs.md'
    if not os.path.exists(tariff_file):
        print(f"Error: {tariff_file} not found")
        return tariffs

    with open(tariff_file, 'r', encoding='utf-8') as f:
        content = f.read()

    # Match rows in the tariff tables: | Symbol | Name | ... | Multiplier | Effective Tariff | ... |
    # Example: | RENDER | ... | 144.84x | 0% |
    # Example: | PYUSD | ... | 0.00x | 35263% |
    pattern = re.compile(r'\|\s*([A-Za-z0-9.\-]+)\s*\|\s*[^|]+\s*\|\s*[^|]+\s*\|\s*([\d,.]+)x\s*\|\s*([\d,.]+)%\s*\|')
    matches = pattern.findall(content)
    for symbol_raw, mult_str, tariff_str in matches:
        symbol = symbol_raw.strip()
        mult = float(mult_str.replace(',', ''))
        # Remove commas from tariff (e.g. "35,263")
        tariff = float(tariff_str.replace(',', ''))
        
        # If multiplier is low and tariff is high, recover precision
        # Formula: Tariff = (100 / Multiplier) - 10 => Multiplier = 100 / (Tariff + 10)
        if mult < 1.0 and tariff > 0:
            precise_mult = 100.0 / (tariff + 10.0)
            tariffs[symbol] = precise_mult
        else:
            tariffs[symbol] = mult
    
    return tariffs

def get_token_data():
    tokens = []
    files = glob.glob('data/*.json')
    for fpath in files:
        with open(fpath, 'r', encoding='utf-8') as f:
            data = json.load(f)
            for token in data:
                tokens.append(token)
    return tokens

def calculate_predicted_multiplier(token, current_multiplier):
    """
    Project multiplier to 2027 based on AI category.
    2026 Phase: Creative Renaissance
    2027 Phase: Agentic (Great Aligner)
    
    Growth Factors (2026 -> 2027):
    - Static Assets: 5.0x -> 2.0x (0.4x)
    - AI-Native Assets: 25.0x -> 50.0x (2.0x)
    - Protocol Utility: 18.0x -> 25.0x (1.39x)
    - AI-Enabled: (interpolated)
    """
    ai_category = token.get('ai_category', 'Static')
    
    if ai_category == 'Static':
        return current_multiplier * (2.0 / 5.0)
    elif ai_category == 'AINative':
        return current_multiplier * (50.0 / 25.0)
    elif ai_category == 'AIEvolving':
        return current_multiplier * (50.0 / 25.0) # Evolving follows Native growth rate
    elif ai_category == 'ProtocolUtility' or ai_category == 'PassiveUtility':
        return current_multiplier * (25.0 / 18.0)
    elif ai_category == 'AIEnabled':
        # 2026 AI-Enabled is roughly 22.2x (interpolated in reasoning_pricer.rs)
        # 2027 AI-Enabled is 50.0x
        return current_multiplier * (50.0 / 22.2)
    
    return current_multiplier

def format_sci(val, prefix="$"):
    if val == 0:
        return f"{prefix}0.0e0"
    s = f"{val:.1e}"
    # Clean up: 1.4e+02 -> 1.4e2, 1.0e-03 -> 1.0e-3
    s = s.replace("e+0", "e").replace("e-0", "e-").replace("e+", "e")
    return f"{prefix}{s}"

def calculate_tariff(multiplier):
    if multiplier <= 0:
        return 1000000.0 # Extremely high tariff for zero multiplier
    t = (100.0 / multiplier) - 10.0
    return max(0.0, t)

def generate_report():
    tariffs = parse_tariffs()
    tokens = get_token_data()
    
    # Sort tokens by multiplier descending
    tokens_with_prices = []
    for token in tokens:
        symbol = token['symbol']
        current_price = token.get('price')
        market_cap = token.get('market_cap', 0)
        
        if symbol in tariffs and current_price is not None:
            mult = tariffs[symbol]
            tariff = calculate_tariff(mult)
            
            # Exchange Price is current price adjusted for tariff friction
            exchange_price = current_price / (1.0 + tariff / 100.0)
            
            # Format AI Category with spaces: AIEnabled -> AI Enabled, PassiveUtility -> Passive Utility
            raw_cat = token.get('ai_category', 'Static')
            
            # Map emojis from tariffs.md
            emojis = {
                'AIEnabled': '🧠',
                'AINative': '🤖',
                'AIEvolving': '🧬',
                'PassiveUtility': '🔋',
                'Static': '🗿'
            }
            emoji = emojis.get(raw_cat, '')
            
            formatted_cat = re.sub(r'(?<=[a-z])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])', ' ', raw_cat)
            full_cat = f"{emoji} {formatted_cat}".strip()
            
            tokens_with_prices.append({
                'symbol': symbol,
                'name': token['name'],
                'current_price': current_price,
                'exchange_price': exchange_price,
                'multiplier': mult,
                'tariff': tariff,
                'market_cap': market_cap,
                'ai_category': full_cat
            })

    # Sort by Multiplier (descending)
    tokens_with_prices.sort(key=lambda x: x['multiplier'], reverse=True)

    report_content = """# Token Price Report (2026)

## Overview

This report provides the **Current Price**, the **Exchange Price** (Immediate Purchasing Power after tariff friction), the **Multiplier** (the real valuation factor), and the **Tariff Rate** (the risk-based friction).

Calculations are based on the AI-Acceleration Pricing model defined in `tariffs.md`.

**Pricing Formula:** `Exchange Price = Current / (1 + Tariff/100)`

All numerical values are shown in **scientific notation** ($X.YeZ$) for precision across all scales.

## Price Table

| Symbol | Name | AI Category | Current Price ($) | Exchange Price ($) | Multiplier | Tariff Rate | Market Cap ($) |
|--------|------|-------------|-------------------|--------------------|------------|-------------|----------------|
"""

    for t in tokens_with_prices:
        cur = format_sci(t['current_price'], prefix="")
        ex = format_sci(t['exchange_price'], prefix="")
        mcap = format_sci(t['market_cap'], prefix="")
        # Scale tariff by 1/100 as per user example (4073% -> 4.1e1)
        tariff_sci = format_sci(t['tariff'] / 100.0, prefix="")
        mult_str = format_sci(t['multiplier'], prefix="")
        
        report_content += f"| {t['symbol']} | {t['name']} | {t['ai_category']} | {cur} | {ex} | {mult_str} | {tariff_sci} | {mcap} |\n"


    with open('reports/token_prices.md', 'w', encoding='utf-8') as f:
        f.write(report_content)
    
    print(f"Successfully generated reports/token_prices.md with {len(tokens_with_prices)} tokens.")

if __name__ == "__main__":
    generate_report()
