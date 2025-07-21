/**
 * TypeScript definitions for octofhir-ucum-wasm
 */

/**
 * Unit information returned by get_unit_info
 */
export interface UnitInfo {
  /** UCUM code for the unit */
  code: string;
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
): EvaluationResult;

/**
 * List all available units
 * @param filter - Optional filter string
 * @returns Array of unit information
 */
export function list_units(filter?: string): UnitInfo[];
