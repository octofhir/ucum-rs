# Interactive CLI Features with Dialoguer

## Overview

The UCUM CLI has been enhanced with interactive features using the [dialoguer](https://docs.rs/dialoguer/latest/dialoguer/) library to significantly improve usability, especially for fuzzy search and other interactive operations.

## New Interactive Command

### `octofhir-ucum interactive`

Launch an interactive session with multiple tools for exploring and working with UCUM units.

```bash
# Start interactive mode
octofhir-ucum interactive

# Start with a specific property filter
octofhir-ucum interactive --property length
```

## Interactive Features

### 🔍 Fuzzy Unit Search

The interactive fuzzy search allows you to:
- Type partial unit names and see real-time matching
- Navigate through results with arrow keys
- Select units with Enter
- Get detailed information about selected units

**Benefits over static search:**
- **Real-time filtering**: See results as you type
- **Better matching**: Fuzzy matching finds units even with typos
- **Intuitive navigation**: Arrow keys and Enter for selection
- **Immediate feedback**: Instant unit details and analysis

### 📋 Property-Based Browsing

Interactive property browser provides:
- Categorized unit exploration by physical properties
- Quick access to related units
- Organized display of unit families

**Example properties:**
- length, mass, time, temperature
- force, energy, power, pressure
- electric current, voltage, resistance
- And many more...

### 🧮 Interactive Unit Conversion

Step-by-step conversion wizard:
- Guided input for source and target units
- Real-time validation of unit expressions
- Immediate conversion results
- Error handling with helpful messages

### ❓ Validation Help with Suggestions

Smart validation assistant:
- Interactive unit expression validation
- Fuzzy suggestions for invalid units
- Detailed error explanations
- Learning-friendly feedback

## Usage Examples

### Basic Interactive Session

```bash
$ octofhir-ucum interactive

🔍 Interactive UCUM Unit Search
================================

Choose an action:
❯ 🔍 Search units by fuzzy matching
  📋 Browse units by property  
  🧮 Interactive unit conversion
  ❓ Get help with unit validation
  🚪 Exit
```

### Fuzzy Search in Action

```bash
# User types "meter" and sees:
❯ m - meter [length]
  [BAU] - bioequivalent allergen unit [amount of allergen...]
  m[Hg] - meter of mercury column [pressure]
  km - kilometer [length]
  cm - centimeter [length]
  mm - millimeter [length]
```

### Interactive Conversion

```bash
🧮 Interactive Unit Conversion
Enter source unit: km/h
Enter target unit: m/s  
Enter value to convert: 100
✅ 100 km/h = 27.777777777777775 m/s
```

### Validation with Suggestions

```bash
❓ Unit Validation Help
Enter a unit expression to validate: metter
❌ 'metter' is invalid: unknown unit: metter

💡 Did you mean one of these?
   m - meter (similarity: 83%)
   [BAU] - bioequivalent allergen unit (similarity: 50%)
   km - kilometer (similarity: 66%)
```

## Technical Implementation

### Dialoguer Components Used

1. **FuzzySelect**: Real-time fuzzy matching for unit search
2. **Select**: Menu navigation for actions and properties
3. **Input**: Text input for conversions and validation
4. **Confirm**: Yes/no prompts for user actions
5. **ColorfulTheme**: Enhanced visual appearance

### Key Benefits

#### Improved Usability
- **Discoverability**: Users can explore units without knowing exact codes
- **Error Prevention**: Fuzzy matching reduces typos and invalid inputs
- **Guided Workflows**: Step-by-step processes for complex operations
- **Immediate Feedback**: Real-time validation and suggestions

#### Enhanced User Experience
- **Visual Appeal**: Colorful, well-formatted output
- **Intuitive Navigation**: Arrow keys and standard terminal interactions
- **Progressive Disclosure**: Information revealed as needed
- **Error Recovery**: Helpful suggestions when things go wrong

#### Accessibility
- **Keyboard Navigation**: Full keyboard support
- **Clear Prompts**: Descriptive prompts and instructions
- **Consistent Interface**: Uniform interaction patterns
- **Graceful Degradation**: Falls back to standard CLI if needed

## Backward Compatibility

All existing CLI commands remain unchanged:
- Static commands work exactly as before
- No breaking changes to existing workflows
- Interactive features are additive enhancements
- Can be used alongside traditional commands

## Integration with Existing Features

The interactive mode leverages all existing UCUM-RS capabilities:
- **ADR-001 API**: Uses enhanced validation, analysis, and search functions
- **Fuzzy Search**: Integrates with existing fuzzy matching algorithms
- **Unit Registry**: Accesses the complete UCUM unit database
- **Conversion Engine**: Uses the same high-precision conversion logic

## Future Enhancements

Potential additions for future versions:
- **Multi-select operations**: Batch operations on multiple units
- **History tracking**: Remember recent searches and conversions
- **Bookmarks**: Save frequently used units
- **Custom themes**: User-configurable color schemes
- **Configuration files**: Persistent user preferences

## Examples in Practice

### Scientific Workflow
```bash
# Researcher exploring pressure units
octofhir-ucum interactive --property pressure
# Fuzzy search for "pascal" variants
# Interactive conversion between different pressure scales
```

### Educational Use
```bash
# Student learning about units
octofhir-ucum interactive
# Explore different unit properties
# Validate homework unit expressions
# Get suggestions for corrections
```

### Development Integration
```bash
# Developer testing unit expressions
octofhir-ucum interactive
# Quick validation of API inputs
# Explore unit relationships
# Test conversion logic
```

## Conclusion

The integration of dialoguer significantly enhances the UCUM CLI's usability by providing:
- **Interactive discovery** of units and their relationships
- **Fuzzy search capabilities** that reduce errors and improve efficiency
- **Guided workflows** that make complex operations accessible
- **Real-time feedback** that accelerates learning and usage

These improvements make the UCUM CLI more approachable for both beginners and power users, while maintaining full backward compatibility with existing workflows.
