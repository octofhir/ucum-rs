# UCUM-RS Playground

An interactive web-based playground for exploring the UCUM-RS library's capabilities.

## Features

- **Validation**: Validate UCUM expressions
- **Unit Information**: Get detailed information about units
- **Conversion**: Convert values between compatible units
- **Arithmetic**: Perform arithmetic operations on units
- **Interactive UI**: User-friendly interface built with Svelte 5

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
# or with npm
npm install
```

### Running the Playground

```bash
# Start the development server
pnpm dev
# or with npm
npm run dev
```

The playground will be available at http://localhost:6000.

## Usage

### Validating UCUM Expressions

Enter a UCUM expression in the validation input field and click "Validate" to check if it's a valid UCUM expression.

Examples of valid expressions:
- `kg`
- `mg/dL`
- `kg.m/s2`
- `mm[Hg]`

### Getting Unit Information

Enter a unit code in the unit information input field and click "Get Info" to see detailed information about the unit, including:

- Code
- Display name
- Factor
- Dimensions
- Property
- Whether it's a special or arbitrary unit

### Converting Between Units

To convert a value between units:

1. Enter the value to convert
2. Enter the source unit
3. Enter the target unit
4. Click "Convert"

The result will show the converted value and indicate whether the conversion was successful.

### Performing Arithmetic Operations

To perform arithmetic operations on units:

1. Enter the left unit
2. Select the operation (multiply, divide)
3. Enter the right unit
4. Enter a value (optional)
5. Click "Calculate"

The result will show the resulting unit expression and its properties.

## Development

### Project Structure

- `src/` - Source code
  - `App.svelte` - Main application component
  - `lib/` - Utility functions and components
  - `main.ts` - Entry point
  - `app.css` - Global styles
- `public/` - Static assets
- `vite.config.ts` - Vite configuration

### Building for Production

```bash
# Build the WebAssembly package first
cd ../ucum-wasm
wasm-pack build --target web

# Build the playground
cd ../playground
pnpm build
# or with npm
npm run build
```

The production build will be available in the `dist/` directory.

### Deployment

The playground can be deployed to any static hosting service, such as GitHub Pages, Netlify, or Vercel.

```bash
# Example: Deploy to GitHub Pages
pnpm build
# Then push the dist/ directory to the gh-pages branch
```

## Contributing

Contributions to the playground are welcome! Please see the main repository's contribution guidelines for more information.

## License

MIT OR Apache-2.0
