/**
 * TypeScript definitions for @octofhir/ucum-wasm
 */

/**
 * Unit information returned by get_unit_info
 */
export interface UnitInfo {
  /** UCUM code for the unit */
  code: string;
  /** Display name for the unit */
  display_name: string;
  /** Conversion factor to canonical unit */
  factor: number;
  /** Offset for units with linear offset (e.g., °C) */
  offset: number;
  /** Whether this is a special unit (non-linear conversion) */
  is_special: boolean;
  /** Whether this is an arbitrary unit (e.g., [IU]) */
  is_arbitrary: boolean;
  /** Dimensional vector [M, L, T, I, Θ, N, J] */
  dimensions: number[];
  /** Property or class of the unit */
  property: string;
}

/**
 * Result of evaluating a UCUM expression
 */
export interface EvaluationResult {
  /** Conversion factor to canonical unit */
  factor: number;
  /** Offset for units with linear offset */
  offset: number;
  /** Dimensional vector [M, L, T, I, Θ, N, J] */
  dimensions: number[];
}

/**
 * Result of analyzing a UCUM unit expression
 */
export interface UnitAnalysis {
  /** The analyzed expression */
  expression: string;
  /** Conversion factor to canonical unit */
  factor: number;
  /** Offset for units with linear offset */
  offset: number;
  /** Dimensional vector [M, L, T, I, Θ, N, J] */
  dimensions: number[];
  /** Whether the unit is dimensionless */
  is_dimensionless: boolean;
  /** Whether the unit has an offset */
  has_offset: boolean;
}

/**
 * Canonical unit representation
 */
export interface CanonicalUnit {
  /** Canonical unit expression */
  unit: string;
  /** Conversion factor to canonical unit */
  factor: number;
  /** Offset for units with linear offset */
  offset: number;
  /** Dimensional vector [M, L, T, I, Θ, N, J] */
  dimensions: number[];
}

/**
 * Result of unit arithmetic operations
 */
export interface UnitArithmeticResult {
  /** Resulting unit expression */
  expression: string;
  /** Conversion factor */
  factor: number;
  /** Dimensional vector [M, L, T, I, Θ, N, J] */
  dimensions: number[];
  /** Offset for units with linear offset */
  offset: number;
  /** Whether the result is dimensionless */
  is_dimensionless: boolean;
}

/**
 * Result of unit search operations
 */
export interface SearchResult {
  /** Array of matching units */
  units: UnitInfo[];
}

/**
 * Result of fuzzy unit search
 */
export interface FuzzySearchResult {
  /** Array of fuzzy matches */
  results: FuzzyMatch[];
}

/**
 * A single fuzzy search match
 */
export interface FuzzyMatch {
  /** The matched unit */
  unit: UnitInfo;
  /** Match score */
  score: number;
}

/**
 * FHIR Quantity representation
 */
export interface FhirQuantity {
  /** Numerical value */
  value: number;
  /** Human-readable unit */
  unit?: string;
  /** System URI (typically UCUM) */
  system?: string;
  /** Coded unit */
  code?: string;
  /** Comparison operator */
  comparator?: string;
}

/**
 * UCUM Quantity result
 */
export interface UcumQuantityResult {
  /** Numerical value */
  value: number;
  /** UCUM unit expression */
  unit: string;
}

/**
 * Initialize the WASM module
 */
export function start(): void;

/**
 * Validate a UCUM expression
 * @param expression - UCUM expression to validate
 * @returns true if the expression is valid
 * @throws Error if the expression is invalid
 */
export function validate(expression: string): boolean;

/**
 * Analyze a UCUM unit expression
 * @param expression - UCUM expression to analyze
 * @returns Analysis result with detailed information
 * @throws Error if the expression is invalid
 */
export function analyze_unit(expression: string): UnitAnalysis;

/**
 * Validate that a UCUM expression has a specific property
 * @param expression - UCUM expression to validate
 * @param property - Expected property (e.g., "length", "mass")
 * @returns true if the expression has the specified property
 * @throws Error if the expression is invalid
 */
export function validate_property(expression: string, property: string): boolean;

/**
 * Check if two units are comparable (same dimensions)
 * @param unit1 - First unit expression
 * @param unit2 - Second unit expression
 * @returns true if the units are comparable
 * @throws Error if either expression is invalid
 */
export function units_comparable(unit1: string, unit2: string): boolean;

/**
 * Get the canonical representation of a unit
 * @param expression - UCUM expression
 * @returns Canonical unit information
 * @throws Error if the expression is invalid
 */
export function get_canonical(expression: string): CanonicalUnit;

/**
 * Multiply two unit expressions
 * @param unit1 - First unit expression
 * @param unit2 - Second unit expression
 * @returns Result of multiplication
 * @throws Error if either expression is invalid
 */
export function multiply_units(unit1: string, unit2: string): UnitArithmeticResult;

/**
 * Divide two unit expressions
 * @param numerator - Numerator unit expression
 * @param denominator - Denominator unit expression
 * @returns Result of division
 * @throws Error if either expression is invalid
 */
export function divide_units(numerator: string, denominator: string): UnitArithmeticResult;

/**
 * Search units by text query
 * @param query - Text to search for
 * @returns Search results
 * @throws Error if search fails
 */
export function search_units_text(query: string): SearchResult;

/**
 * Search units by property
 * @param property - Property to search for (e.g., "length", "mass")
 * @returns Search results
 * @throws Error if search fails
 */
export function search_units_property(property: string): SearchResult;

/**
 * Get different forms of a unit
 * @param base_code - Base unit code
 * @returns Different forms of the unit
 * @throws Error if unit not found
 */
export function get_unit_forms(base_code: string): SearchResult;

/**
 * Fuzzy search for units
 * @param query - Search query
 * @param threshold - Minimum match score threshold
 * @returns Fuzzy search results
 * @throws Error if search fails
 */
export function search_units_fuzzy(query: string, threshold: number): FuzzySearchResult;

/**
 * Search units using regular expressions
 * @param pattern - Regular expression pattern
 * @param case_sensitive - Whether search is case sensitive
 * @returns Search results
 * @throws Error if pattern is invalid or search fails
 */
export function search_units_regex(pattern: string, case_sensitive: boolean): SearchResult;

/**
 * Get information about a unit
 * @param code - UCUM code for the unit
 * @returns Information about the unit
 * @throws Error if the unit is not found
 */
export function get_unit_info(code: string): UnitInfo;

/**
 * Convert a value from one unit to another
 * @param value - Value to convert
 * @param from_unit - Source unit (UCUM expression)
 * @param to_unit - Target unit (UCUM expression)
 * @returns Converted value
 * @throws Error if the units are incompatible or invalid
 */
export function convert(value: number, from_unit: string, to_unit: string): number;

/**
 * Evaluate a UCUM expression
 * @param expression - UCUM expression to evaluate
 * @returns Evaluation result
 * @throws Error if the expression is invalid
 */
export function evaluate_expression(expression: string): EvaluationResult;

/**
 * Perform arithmetic operations on units
 * @param left_unit - Left operand (UCUM expression)
 * @param operation - Operation to perform ("mul" or "div")
 * @param right_unit - Right operand (UCUM expression)
 * @param value - Value to apply the operation to
 * @returns Result of the operation
 * @throws Error if the units are invalid or the operation is unsupported
 */
export function arithmetic(
  left_unit: string,
  operation: "mul" | "div",
  right_unit: string,
  value: number
): UnitArithmeticResult;

/**
 * List all available units
 * @param filter - Optional filter string
 * @returns Array of unit information
 */
export function list_units(filter?: string): UnitInfo[];

/**
 * Create a FHIR Quantity with a UCUM code
 * @param value - Numerical value
 * @param ucum_code - UCUM unit code
 * @returns FHIR Quantity object
 * @throws Error if the UCUM code is invalid
 */
export function create_fhir_quantity(value: number, ucum_code: string): FhirQuantity;

/**
 * Convert a FHIR Quantity to a UCUM Quantity
 * @param js_quantity_val - FHIR Quantity object
 * @returns UCUM Quantity result
 * @throws Error if conversion fails
 */
export function fhir_to_ucum(js_quantity_val: FhirQuantity): UcumQuantityResult;

/**
 * Convert a UCUM Quantity to a FHIR Quantity
 * @param value - Numerical value
 * @param unit - UCUM unit expression
 * @returns FHIR Quantity object
 * @throws Error if the unit is invalid
 */
export function ucum_to_fhir(value: number, unit: string): FhirQuantity;

/**
 * Convert a FHIR Quantity from one unit to another
 * @param js_quantity_val - Source FHIR Quantity
 * @param target_unit - Target UCUM unit expression
 * @returns Converted FHIR Quantity
 * @throws Error if conversion fails
 */
export function convert_fhir_quantity(js_quantity_val: FhirQuantity, target_unit: string): FhirQuantity;

/**
 * Check if two FHIR Quantities are equivalent
 * @param a_val - First FHIR Quantity
 * @param b_val - Second FHIR Quantity
 * @returns true if the quantities are equivalent
 * @throws Error if comparison fails
 */
export function are_fhir_quantities_equivalent(a_val: FhirQuantity, b_val: FhirQuantity): boolean;
