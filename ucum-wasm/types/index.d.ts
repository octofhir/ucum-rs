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
  /** Property or class of the unit */
  property: string;
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
 * Result of unit search operations
 */
export interface SearchResult {
  /** Array of matching units */
  units: UnitInfo[];
}

/**
 * Initialize the WASM module. Must be called before using other functions.
 */
export function start(): void;

/**
 * Validate a UCUM expression
 * @param expression - UCUM expression to validate
 * @returns true if the expression is valid, false otherwise
 * @throws Error if validation fails
 */
export function validate(expression: string): boolean;

/**
 * Analyze a UCUM unit expression and get detailed information
 * @param expression - UCUM expression to analyze
 * @returns Analysis result with detailed information
 * @throws Error if the expression is invalid
 */
export function analyze(expression: string): UnitAnalysis;

/**
 * Get the canonical (base unit) representation of a UCUM expression
 * @param expression - UCUM expression
 * @returns Canonical unit information
 * @throws Error if the expression is invalid
 */
export function get_canonical(expression: string): CanonicalUnit;

/**
 * Check if two units are comparable (have the same dimensions)
 * @param unit1 - First unit expression
 * @param unit2 - Second unit expression
 * @returns true if the units are comparable
 * @throws Error if either expression is invalid
 */
export function comparable(unit1: string, unit2: string): boolean;

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
 * Search for units by name or code
 * @param query - Search query
 * @returns Search results
 * @throws Error if search fails
 */
export function search(query: string): SearchResult;

/**
 * Get detailed information about a unit
 * @param code - UCUM code for the unit
 * @returns Information about the unit
 * @throws Error if the unit is not found
 */
export function get_unit_info(code: string): UnitInfo;
