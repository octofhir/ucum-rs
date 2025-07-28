# UCUM CLI

Command-line interface for UCUM (Unified Code for Units of Measure) operations.

## Installation

```sh
cargo install octofhir-ucum-cli
```

## Usage

### Validate UCUM expressions
```sh
octofhir-ucum validate "mg/dL"
# ✅ Valid UCUM expression: mg/dL
```

### Analyze unit expressions
```sh
octofhir-ucum analyze "mg/dL"
# 📊 Unit Analysis: mg/dL
#    Factor: 0.000010
#    Dimension: [1, -3, 0, 0, 0, 0, 0]
#    Offset: 0.000000
#    Dimensionless: false
#    Has offset: false
```

### Get canonical forms
```sh
octofhir-ucum canonical "mg/dL"
# 📐 Canonical form of 'mg/dL':
#    Unit: kg.m-3
#    Factor: 0.000010
#    Dimension: [1, -3, 0, 0, 0, 0, 0]
```

### Convert between units
```sh
octofhir-ucum convert 100 "mg" "g"
# 100 mg = 0.1 g
```

### Check unit compatibility
```sh
octofhir-ucum comparable "mg" "g"
# ✅ 'mg' and 'g' are comparable (can be converted)

octofhir-ucum comparable "mg" "s"
# ❌ 'mg' and 's' are NOT comparable (different dimensions)
```

### Search for units
```sh
octofhir-ucum search "gram" --limit 5
# 🔍 Search results for 'gram':
# ================================
# 1. g - gram [mass]
# 2. kg - kilogram [mass]
# 3. mg - milligram [mass]
# 4. ug - microgram [mass]
# 5. ng - nanogram [mass]
```

## Commands

| Command | Description |
|---------|-------------|
| `validate <expression>` | Validate a UCUM expression |
| `analyze <expression>` | Analyze unit properties (factor, dimension, etc.) |
| `canonical <expression>` | Get canonical (base SI) form |
| `convert <value> <from> <to>` | Convert value between units |  
| `comparable <unit1> <unit2>` | Check if units are comparable |
| `search <query>` | Search for units by name/code |

## License

Apache-2.0