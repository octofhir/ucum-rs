import { useCallback, useEffect, useState } from 'react';

// Import the WASM module
let ucumWasm: typeof import('@octofhir/ucum-wasm') | null = null;
let wasmInitialized = false;

// WASM call queue to prevent concurrent access
interface WasmCall {
  fn: () => any;
  resolve: (value: any) => void;
  reject: (error: any) => void;
}

const wasmCallQueue: WasmCall[] = [];
let isProcessingQueue = false;

const processWasmQueue = async () => {
  if (isProcessingQueue || wasmCallQueue.length === 0) return;

  isProcessingQueue = true;

  while (wasmCallQueue.length > 0) {
    const call = wasmCallQueue.shift();
    if (!call) continue;

    try {
      const result = await call.fn();
      call.resolve(result);
    } catch (error) {
      call.reject(error);
    }

    // Small delay to prevent mutex conflicts
    await new Promise((resolve) => setTimeout(resolve, 1));
  }

  isProcessingQueue = false;
};

const queueWasmCall = <T>(fn: () => T): Promise<T> => {
  return new Promise((resolve, reject) => {
    wasmCallQueue.push({ fn, resolve, reject });
    processWasmQueue();
  });
};

const initializeUcum = async () => {
  if (ucumWasm && wasmInitialized) return ucumWasm;

  try {
    console.log('Loading UCUM WASM module...');
    ucumWasm = await import('@octofhir/ucum-wasm');
    
    if (!ucumWasm) {
      throw new Error('Failed to load WASM module');
    }

    // Initialize WASM using the default export function
    if (typeof ucumWasm.default === 'function') {
      console.log('Initializing WASM...');
      await ucumWasm.default();
      console.log('WASM initialized');
    }

    // Call the start function
    if (typeof ucumWasm.start === 'function') {
      console.log('Starting UCUM...');
      ucumWasm.start();
      console.log('UCUM started');
    }

    // Test a simple function
    if (typeof ucumWasm.validate === 'function') {
      console.log('Testing WASM with simple validation...');
      const testResult = ucumWasm.validate('kg');
      console.log('Test result for "kg":', testResult);
    }

    wasmInitialized = true;
    return ucumWasm;
  } catch (error) {
    console.error('Failed to load UCUM WASM module:', error);
    throw error;
  }
};

export const useUcum = () => {
  const [isLoaded, setIsLoaded] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    initializeUcum()
      .then(() => {
        setIsLoaded(true);
        setError(null);
      })
      .catch((err) => {
        setError(err.message || 'Failed to initialize UCUM');
        setIsLoaded(false);
      });
  }, []);

  const validateExpression = useCallback(
    async (expression: string): Promise<{ valid: boolean; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        const result = await queueWasmCall(() => ucumWasm?.validate(expression));
        return { valid: Boolean(result) };
      } catch (error: any) {
        // Parse error from WASM
        if (error && typeof error === 'object' && error.message) {
          return { valid: false, error: error.message };
        }
        return { valid: false, error: String(error) };
      }
    },
    []
  );

  const analyzeUnit = useCallback(
    async (expression: string): Promise<{ info?: any; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        const result = await queueWasmCall(() => ucumWasm?.analyze_unit(expression));
        return { info: result };
      } catch (error: any) {
        if (error && typeof error === 'object' && error.message) {
          return { error: error.message };
        }
        return { error: String(error) };
      }
    },
    []
  );

  const getUnitInfo = useCallback(
    async (unitCode: string): Promise<{ info?: any; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        const result = await queueWasmCall(() => ucumWasm?.get_unit_info(unitCode));
        return { info: result };
      } catch (error: any) {
        if (error && typeof error === 'object' && error.message) {
          return { error: error.message };
        }
        return { error: String(error) };
      }
    },
    []
  );

  const getCanonicalUnit = useCallback(
    async (expression: string): Promise<{ canonical?: any; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        const result = await queueWasmCall(() => ucumWasm?.get_canonical(expression));
        return { canonical: result };
      } catch (error: any) {
        if (error && typeof error === 'object' && error.message) {
          return { error: error.message };
        }
        return { error: String(error) };
      }
    },
    []
  );

  const searchUnits = useCallback(
    async (query: string): Promise<{ units?: any[]; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        const result = await queueWasmCall(() => ucumWasm?.search_units_text(query));
        if (result && result.units) {
          return { units: result.units };
        }
        return { units: [] };
      } catch (error: any) {
        if (error && typeof error === 'object' && error.message) {
          return { error: error.message };
        }
        return { error: String(error) };
      }
    },
    []
  );

  const convertValue = useCallback(
    async (
      value: number,
      fromUnit: string,
      toUnit: string
    ): Promise<{ result?: number; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        const result = await queueWasmCall(() => ucumWasm?.convert(value, fromUnit, toUnit));
        return { result };
      } catch (error: any) {
        if (error && typeof error === 'object' && error.message) {
          return { error: error.message };
        }
        return { error: String(error) };
      }
    },
    []
  );

  const multiplyUnits = useCallback(
    async (
      unit1: string,
      unit2: string
    ): Promise<{ result?: any; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        const result = await queueWasmCall(() => ucumWasm?.multiply_units(unit1, unit2));
        return { result };
      } catch (error: any) {
        if (error && typeof error === 'object' && error.message) {
          return { error: error.message };
        }
        return { error: String(error) };
      }
    },
    []
  );

  const divideUnits = useCallback(
    async (
      unit1: string,
      unit2: string
    ): Promise<{ result?: any; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        const result = await queueWasmCall(() => ucumWasm?.divide_units(unit1, unit2));
        return { result };
      } catch (error: any) {
        if (error && typeof error === 'object' && error.message) {
          return { error: error.message };
        }
        return { error: String(error) };
      }
    },
    []
  );

  const checkCompatibility = useCallback(
    async (
      unit1: string,
      unit2: string
    ): Promise<{ compatible: boolean; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        const result = await queueWasmCall(() => ucumWasm?.units_comparable(unit1, unit2));
        return { compatible: Boolean(result) };
      } catch (error: any) {
        if (error && typeof error === 'object' && error.message) {
          return { compatible: false, error: error.message };
        }
        return { compatible: false, error: String(error) };
      }
    },
    []
  );

  const listUnits = useCallback(
    async (filter?: string): Promise<{ units?: any[]; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        const result = await queueWasmCall(() => ucumWasm?.list_units(filter || null));
        return { units: Array.isArray(result) ? result : [] };
      } catch (error: any) {
        if (error && typeof error === 'object' && error.message) {
          return { error: error.message };
        }
        return { error: String(error) };
      }
    },
    []
  );

  const evaluateExpression = useCallback(
    async (expression: string): Promise<{ result?: any; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        const result = await queueWasmCall(() => ucumWasm?.evaluate_expression(expression));
        return { result };
      } catch (error: any) {
        if (error && typeof error === 'object' && error.message) {
          return { error: error.message };
        }
        return { error: String(error) };
      }
    },
    []
  );

  // Cached versions that use the same underlying functions
  const validateExpressionCached = validateExpression;
  const convertValueCached = convertValue;
  const multiplyUnitsCached = multiplyUnits;
  const divideUnitsCached = divideUnits;
  const checkCompatibilityCached = checkCompatibility;
  const searchUnitsCached = searchUnits;
  const analyzeUnitCached = analyzeUnit;
  const getUnitInfoCached = getUnitInfo;
  const getCanonicalUnitCached = getCanonicalUnit;

  return {
    isLoaded,
    error,
    // Core functions
    validateExpression,
    analyzeUnit,
    getUnitInfo,
    getCanonicalUnit,
    convertValue,
    multiplyUnits,
    divideUnits,
    checkCompatibility,
    searchUnits,
    listUnits,
    evaluateExpression,
    // Cached versions (same as originals for now)
    validateExpressionCached,
    convertValueCached,
    multiplyUnitsCached,
    divideUnitsCached,
    checkCompatibilityCached,
    searchUnitsCached,
    analyzeUnitCached,
    getUnitInfoCached,
    getCanonicalUnitCached,
  };
};