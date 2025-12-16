# Solana Token Analysis Report Generator - Refactored Structure

This directory contains the refactored report generation system, split into modular components for better maintainability and reusability.

## File Structure

### Core Modules

- **`main.py`** - Entry point that runs the report generation process
- **`report_generator.py`** - Main orchestrator that coordinates all components
- **`models.py`** - Data models (Token, TokenAnalysis, RiskClass) using Pydantic
- **`utils.py`** - Utility functions, constants, and helper methods

### Report Generators

- **`data_loader.py`** - Handles loading token data from JSON files
- **`token_reporter.py`** - Generates individual token analysis reports
- **`summary_reporter.py`** - Creates risk class summary reports
- **`index_generator.py`** - Generates navigation index files

## Usage

Run the report generator from the `src` directory:

```bash
cd src
python main.py
```

## Key Improvements

1. **Modularity**: Each component has a single responsibility
2. **Maintainability**: Easier to modify individual components without affecting others
3. **Testability**: Each module can be tested independently
4. **Reusability**: Components can be imported and used in other projects
5. **Error Handling**: Better error isolation and reporting

## Dependencies

- Python 3.7+
- Pydantic for data validation
- Standard library modules (pathlib, json, typing)

## Output

The generator creates markdown reports in the `../reports/` directory:
- Individual token reports (`tokens/` subdirectory)
- Risk class summaries (`summaries/` subdirectory) 
- Navigation index files