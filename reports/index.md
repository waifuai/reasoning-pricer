# Solana Token Analysis Reports

## Overview

This repository contains comprehensive analysis reports for 99 Solana tokens, organized by risk classification and individual token details.

## 📊 Risk Class Summary

| Risk Class | Token Count | Description |
|------------|-------------|-------------|
| **Class A (Real Yield)** | 42 | Low-risk foundation assets |
| **Class B (Systemic)** | 35 | Medium-risk ecosystem tokens |
| **Class C (Venture Risk)** | 21 | Higher-risk growth opportunities |
| **Class D (Speculative)** | 1 | Maximum-risk speculative assets |

## 📈 Quick Stats

- **Total Tokens Analyzed:** 99
- **Data Source:** JSON files in `data/` directory
- **Risk Distribution:** 📊 Imbalanced
- **Average Insider Score:** 30.8/100
- **Average Tariff:** 41.8%

## 📁 Report Structure

### By Risk Class
- [Class A (Real Yield)](./summaries/class-a-real-yield.md) - 42 tokens
- [Class B (Systemic)](./summaries/class-b-systemic.md) - 35 tokens
- [Class C (Venture Risk)](./summaries/class-c-venture-risk.md) - 21 tokens
- [Class D (Speculative)](./summaries/class-d-speculative.md) - 1 tokens

### Individual Token Reports
- [📋 All Token Reports](./tokens/index.md)

## 🔍 How to Use These Reports

1. **Start with Risk Class Summaries** - Understand the landscape by reviewing each risk class
2. **Drill Down to Individual Tokens** - Click through to detailed token analysis
3. **Use Navigation Links** - Jump between related reports using the navigation sections
4. **Compare Exchange Rates** - Review tariff impacts and exchange multipliers

## 📊 Methodology

### Risk Classification
Tokens are classified into four risk classes based on:
- **Insider Score** (0-100): Degree of insider control
- **Economic Utility**: Real value accrual mechanisms  
- **Regulatory Risk**: Compliance and legal considerations
- **Operational Risk**: Technical and execution risks

### Exchange Rate Calculation
```
Exchange Rate = 1 + (Tariff Override / 100)
```

### Example: 5% tariff = 1.05x exchange rate

---

## 🔗 Quick Links

- [📊 Class A (Real Yield)](./summaries/class-a-real-yield.md)
- [📊 Class B (Systemic)](./summaries/class-b-systemic.md)  
- [📊 Class C (Venture Risk)](./summaries/class-c-venture-risk.md)
- [📊 Class D (Speculative)](./summaries/class-d-speculative.md)
- [📋 All Tokens](./tokens/index.md)

*For questions about methodology or data sources, see the main project documentation.*
