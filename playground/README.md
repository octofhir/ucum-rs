# UCUM-RS Playground

An interactive web-based playground for exploring the UCUM-RS library's capabilities.

## Features

The playground provides a simplified, focused interface with three main tools:

- **Validation**: Validate UCUM expressions and get instant feedback
- **Unit Operations**: Convert values between units and perform arithmetic operations
- **Unit Explorer**: Look up detailed information about any UCUM unit

## Getting Started

### Prerequisites

- Node.js 20+ 
- pnpm (recommended) or npm

### Installation

```bash
# Clone the repository if you haven't already
git clone https://github.com/octofhir/ucum-rs.git
cd ucum-rs

# Navigate to the playground directory
cd playground

# Install dependencies
pnpm install
```

### Running the Playground

```bash
# Start the development server (use npm due to pnpm script execution issues)
npm run dev
```

The playground will be available at http://localhost:6000.

## Usage Guide

### 1. Validation Tab

Validate UCUM expressions to ensure they conform to the specification.

**Examples of valid expressions:**
- `kg` (kilogram)
- `mg/dL` (milligrams per deciliter)
- `kg.m/s2` (kilogram meter per second squared)
- `mm[Hg]` (millimeters of mercury)

### 2. Unit Operations Tab

This tab combines conversion and arithmetic functionality:

#### Conversion
Convert values between compatible units:
1. Enter the value to convert
2. Select or type the source unit
3. Select or type the target unit
4. Click "Convert"

**Quick examples:** Mass (kg → g), Temperature (°C → °F), Pressure (mmHg → kPa)

#### Arithmetic
Perform multiplication and division on units:
1. Select the operation (× or ÷)
2. Enter the first unit
3. Enter the second unit
4. Click "Calculate"

**Common formulas:** Force × Distance = Energy, Distance ÷ Time = Speed

### 3. Unit Explorer Tab

Search and explore detailed information about any unit:
- Unit code and display name
- Conversion factor to base units
- Dimensional analysis
- Property classification
- Special unit indicators

## Key Improvements in v0.3.0

- **Simplified UI**: Removed complex widgets and focused on core functionality
- **Combined Operations**: Merged conversion and arithmetic into a single, intuitive tab
- **Cleaner Sidebar**: Streamlined status display with essential information only
- **Better Examples**: More practical, real-world unit conversion examples
- **Responsive Design**: Improved mobile and tablet experience

## Development

### Building for Production

```bash
# Build the WebAssembly package first
cd ../ucum-wasm
wasm-pack build --target web

# Build the playground
cd ../playground
pnpm build
```

The production build will be available in the `dist/` directory.

### Technology Stack

- React with TypeScript
- Mantine UI v7
- WebAssembly (UCUM-RS compiled to WASM)
- Vite for fast development and building

## License

Apache-2.0