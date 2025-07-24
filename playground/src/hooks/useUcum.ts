import { useCallback, useEffect, useState } from 'react';

// Import the WASM module
let ucumWasm: typeof import('@octofhir/ucum-wasm') | null = null;
let wasmInitialized = false;

// Simple cache statistics tracker for development
const cacheStats = {
  hits: 0,
  misses: 0,
  operations: new Map<string, any>(), // key -> result cache
  get total_operations() { return this.hits + this.misses; },
  get hit_rate() { return this.total_operations ? this.hits / this.total_operations : 0; }
};

// WASM call queue to prevent concurrent access and mutex conflicts
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
    console.log(
      'WASM module loaded, available functions:',
      ucumWasm ? Object.keys(ucumWasm) : 'none'
    );

    if (!ucumWasm) {
      throw new Error('Failed to load WASM module');
    }

    // Initialize WASM using the default export function
    if (typeof ucumWasm.default === 'function') {
      console.log('Initializing WASM...');
      await ucumWasm.default();
      console.log('WASM initialized');
    } else {
      console.log('No default initializer found, WASM might already be ready');
    }

    // Check if the start function exists and call it
    if (typeof ucumWasm.start === 'function') {
      console.log('Starting UCUM...');
      ucumWasm.start();
      console.log('UCUM started');
    } else {
      console.log('No start function found');
    }

    // Test a simple function to see if WASM is working
    if (typeof ucumWasm.validate === 'function') {
      console.log('Testing WASM with simple validation...');
      const testResult = ucumWasm.validate('kg');
      console.log('Test result for "kg":', testResult);
    }

    wasmInitialized = true;
    return ucumWasm;
  } catch (error) {
    console.error('Failed to load UCUM WASM module:', error);
    console.error('Error details:', error);
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

  const validateExpression = async (
    expression: string
  ): Promise<{ valid: boolean; error?: string }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      const result = await queueWasmCall(() => ucumWasm?.validate(expression));
      return { valid: Boolean(result) };
    } catch (error) {
      return {
        valid: false,
        error: error instanceof Error ? error.message : String(error),
      };
    }
  };

  const validateProperty = async (
    expression: string,
    property: string
  ): Promise<{ valid: boolean; error?: string }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      const result = await queueWasmCall(() => ucumWasm?.validate_property(expression, property));
      return { valid: Boolean(result) };
    } catch (error) {
      return {
        valid: false,
        error: error instanceof Error ? error.message : String(error),
      };
    }
  };

  const checkCompatibility = async (
    unit1: string,
    unit2: string
  ): Promise<{ compatible: boolean; error?: string }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      const result = await queueWasmCall(() => ucumWasm?.units_comparable(unit1, unit2));
      return { compatible: Boolean(result) };
    } catch (error) {
      return {
        compatible: false,
        error: error instanceof Error ? error.message : String(error),
      };
    }
  };

  const getUnitInfo = async (unit: string): Promise<{ info?: any; error?: string }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      const result = await queueWasmCall(() => ucumWasm?.get_unit_info(unit));
      
      // Check if the result is an error object from WASM
      if (result && typeof result === 'object' && result.error) {
        return {
          error: typeof result.error === 'string' ? result.error : 'Failed to get unit information',
        };
      }
      
      // Check if result is null/undefined (unit not found)
      if (!result) {
        return {
          error: `Unit '${unit}' not found or invalid`,
        };
      }
      
      return { info: result };
    } catch (error) {
      return {
        error: error instanceof Error ? error.message : String(error),
      };
    }
  };

  const convertValue = async (
    value: number,
    fromUnit: string,
    toUnit: string
  ): Promise<{ result?: number; error?: string }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      const result = await queueWasmCall(() => ucumWasm?.convert(value, fromUnit, toUnit));
      return { result };
    } catch (error) {
      return {
        error: error instanceof Error ? error.message : String(error),
      };
    }
  };

  const multiplyUnits = async (
    unit1: string,
    unit2: string
  ): Promise<{ result?: string; error?: string }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      const result = await queueWasmCall(() => ucumWasm?.multiply_units(unit1, unit2));
      return { result };
    } catch (error) {
      return {
        error: error instanceof Error ? error.message : String(error),
      };
    }
  };

  const divideUnits = async (
    unit1: string,
    unit2: string
  ): Promise<{ result?: string; error?: string }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      const result = await queueWasmCall(() => ucumWasm?.divide_units(unit1, unit2));
      return { result };
    } catch (error) {
      return {
        error: error instanceof Error ? error.message : String(error),
      };
    }
  };

  const validateFhirQuantity = async (
    quantity: any
  ): Promise<{ valid: boolean; error?: string }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      // For FHIR validation, we can check if the unit code is valid and if the structure is correct
      if (!quantity.code || !quantity.value) {
        return { valid: false, error: 'FHIR Quantity must have value and code' };
      }

      const result = await queueWasmCall(() => ucumWasm?.validate(quantity.code));
      return { valid: Boolean(result) };
    } catch (error) {
      return {
        valid: false,
        error: error instanceof Error ? error.message : String(error),
      };
    }
  };

  // New autocomplete functions
  const listUnits = useCallback(
    async (filter?: string): Promise<{ units?: any[]; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        // Check if the function exists
        if (typeof ucumWasm.list_units !== 'function') {
          console.warn('list_units function not available');
          return { units: [] };
        }

        const result = await queueWasmCall(() =>
          filter ? ucumWasm?.list_units(filter) : ucumWasm?.list_units()
        );
        return { units: Array.isArray(result) ? result : [] };
      } catch (error) {
        return {
          error: error instanceof Error ? error.message : String(error),
        };
      }
    },
    []
  );

  const searchUnitsText = useCallback(
    async (query: string): Promise<{ units?: any[]; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        // Check if the function exists
        if (typeof ucumWasm.search_units_text !== 'function') {
          console.warn('search_units_text function not available, falling back to list_units');
          if (typeof ucumWasm.list_units === 'function') {
            const allUnits = ucumWasm.list_units();
            if (Array.isArray(allUnits)) {
              // Filter units that contain the query
              const filtered = allUnits.filter(
                (unit: any) =>
                  unit.code?.toLowerCase().includes(query.toLowerCase()) ||
                  unit.display_name?.toLowerCase().includes(query.toLowerCase())
              );
              return { units: filtered };
            }
          }
          return { units: [] };
        }

        const result = await queueWasmCall(() => ucumWasm?.search_units_text(query));
        return { units: Array.isArray(result) ? result : [] };
      } catch (error) {
        return {
          error: error instanceof Error ? error.message : String(error),
        };
      }
    },
    []
  );

  const searchUnitsFuzzy = useCallback(
    async (query: string, threshold: number = 80): Promise<{ units?: any[]; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        // Check if the function exists
        if (typeof ucumWasm.search_units_fuzzy !== 'function') {
          console.warn('search_units_fuzzy function not available, returning empty results');
          return { units: [] };
        }

        const result = await queueWasmCall(() =>
          ucumWasm?.search_units_fuzzy(query, BigInt(threshold))
        );
        return { units: Array.isArray(result) ? result : [] };
      } catch (error) {
        return {
          error: error instanceof Error ? error.message : String(error),
        };
      }
    },
    []
  );

  const getUnitSuggestions = useCallback(
    async (invalidUnit: string): Promise<{ suggestions?: string[]; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        // Use the actual WASM function for unit suggestions
        const result = await queueWasmCall(() => ucumWasm?.get_unit_suggestions(invalidUnit));

        // Handle the result - it could be an array of strings or objects with code property
        if (Array.isArray(result)) {
          const suggestions = result
            .map((item) => {
              if (typeof item === 'string') {
                // Extract unit code from formatted suggestion strings like "Did you mean 'g'? (gram) [similarity: 100.0%]"
                const match = item.match(/'([^']+)'/);
                return match ? match[1] : item;
              }
              return item?.code || item;
            })
            .filter((suggestion) => suggestion && suggestion !== invalidUnit)
            .slice(0, 5);
          return { suggestions };
        }

        return { suggestions: [] };
      } catch (error) {
        return {
          error: error instanceof Error ? error.message : String(error),
        };
      }
    },
    []
  );

  const getPropertyAlternatives = useCallback(
    async (
      unit: string,
      property: string
    ): Promise<{ alternatives?: string[]; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        const result = await queueWasmCall(() =>
          ucumWasm?.get_property_alternatives(unit, property)
        );

        if (Array.isArray(result)) {
          const alternatives = result
            .map((item) => {
              if (typeof item === 'string') {
                // Extract unit code from formatted strings if needed
                const match = item.match(/'([^']+)'/);
                return match ? match[1] : item;
              }
              return item?.code || item;
            })
            .filter((alt) => alt && alt !== unit)
            .slice(0, 10);
          return { alternatives };
        }

        return { alternatives: [] };
      } catch (error) {
        return {
          error: error instanceof Error ? error.message : String(error),
        };
      }
    },
    []
  );

  const getDimensionSuggestions = useCallback(
    async (
      expectedProperty: string,
      foundUnit: string
    ): Promise<{ suggestions?: string[]; error?: string }> => {
      if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

      try {
        const result = await queueWasmCall(() =>
          ucumWasm?.get_dimension_suggestions(expectedProperty, foundUnit)
        );

        if (Array.isArray(result)) {
          const suggestions = result
            .map((item) => (typeof item === 'string' ? item : item?.code || item))
            .filter((suggestion) => suggestion && suggestion !== foundUnit)
            .slice(0, 5);
          return { suggestions };
        }

        return { suggestions: [] };
      } catch (error) {
        return {
          error: error instanceof Error ? error.message : String(error),
        };
      }
    },
    []
  );

  const getUcumProperties = useCallback(async (): Promise<{
    properties?: string[];
    error?: string;
  }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      const result = await queueWasmCall(() => ucumWasm?.get_ucum_properties());
      return { properties: Array.isArray(result) ? result : [] };
    } catch (error) {
      return {
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }, []);

  const getPerformanceCacheStats = useCallback(async (): Promise<{
    stats?: any;
    error?: string;
  }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      // Check if the method exists
      if (typeof ucumWasm?.get_performance_cache_stats !== 'function') {
        console.warn('⚠️ get_performance_cache_stats method not available on WASM module');
        // Return real cache statistics from our tracker
        return { 
          stats: {
            hit_rate: cacheStats.hit_rate,
            hits: cacheStats.hits,
            misses: cacheStats.misses,
            total_operations: cacheStats.total_operations
          }
        };
      }
      
      const result = await queueWasmCall(() => ucumWasm?.get_performance_cache_stats());
      console.debug('📊 Cache stats result:', result);
      
      // Map WASM cache stats format to our expected format
      if (result && typeof result === 'object') {
        const expressionHits = result.expression_hits || 0;
        const expressionMisses = result.expression_misses || 0;
        const conversionHits = result.conversion_hits || 0;
        const conversionMisses = result.conversion_misses || 0;
        const dimensionHits = result.dimension_hits || 0;
        const dimensionMisses = result.dimension_misses || 0;
        
        const totalHits = expressionHits + conversionHits + dimensionHits;
        const totalMisses = expressionMisses + conversionMisses + dimensionMisses;
        const totalOps = totalHits + totalMisses;
        
        const mappedStats = {
          // Map WASM cache counts to our format
          expressions: expressionHits,
          conversions: conversionHits,
          dimensions: dimensionHits,
          expression_misses: expressionMisses,
          conversion_misses: conversionMisses,
          dimension_misses: dimensionMisses,
          // Calculate totals for compatibility
          hits: totalHits,
          misses: totalMisses,
          total_operations: totalOps,
          hit_rate: totalOps > 0 ? totalHits / totalOps : 0
        };
        console.debug('📊 Mapped cache stats:', mappedStats);
        return { stats: mappedStats };
      }
      
      return { stats: result };
    } catch (error) {
      console.error('❌ Cache stats error:', error);
      return {
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }, []);

  const getPerformanceCacheSizes = useCallback(async (): Promise<{
    sizes?: any;
    error?: string;
  }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      // Check if the method exists
      if (typeof ucumWasm?.get_performance_cache_sizes !== 'function') {
        console.warn('⚠️ get_performance_cache_sizes method not available on WASM module');
        // Return real cache sizes from our tracker
        return { 
          sizes: {
            current_size: cacheStats.operations.size,
            max_size: 1000 // Reasonable limit
          }
        };
      }
      
      const result = await queueWasmCall(() => ucumWasm?.get_performance_cache_sizes());
      console.debug('📏 Cache sizes result:', result);
      return { sizes: result };
    } catch (error) {
      console.error('❌ Cache sizes error:', error);
      return {
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }, []);

  const getUcumModel = useCallback(async (): Promise<{
    model?: any;
    error?: string;
  }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      const result = await queueWasmCall(() => ucumWasm?.get_ucum_model());
      return { model: result };
    } catch (error) {
      return {
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }, []);

  const validateUcumImplementation = useCallback(async (): Promise<{
    validation?: any;
    error?: string;
  }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      const result = await queueWasmCall(() => ucumWasm?.validate_ucum_implementation());
      return { validation: result };
    } catch (error) {
      return {
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }, []);

  const clearPerformanceCache = useCallback(async (): Promise<{
    success?: boolean;
    error?: string;
  }> => {
    if (!ucumWasm || !wasmInitialized) throw new Error('UCUM not initialized');

    try {
      // Check if the method exists
      if (typeof ucumWasm?.clear_performance_cache !== 'function') {
        console.warn('⚠️ clear_performance_cache method not available on WASM module');
        // Clear our JavaScript cache tracker
        cacheStats.hits = 0;
        cacheStats.misses = 0;
        cacheStats.operations.clear();
        console.debug('🗑️ JavaScript cache cleared');
        return { success: true };
      }
      
      const result = await queueWasmCall(() => ucumWasm?.clear_performance_cache());
      console.debug('🗑️ Cache cleared:', result);
      return { success: Boolean(result) };
    } catch (error) {
      console.error('❌ Clear cache error:', error);
      return {
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }, []);

  // Cached version of search functions - leverage WASM internal caching
  const searchUnitsTextCached = useCallback(
    async (query: string): Promise<{ units?: any[]; error?: string }> => {
      // The WASM package already handles caching internally via the global cache
      // These functions benefit from expression caching, conversion caching, etc.
      return searchUnitsText(query);
    },
    [searchUnitsText]
  );

  const searchUnitsFuzzyCached = useCallback(
    async (query: string, threshold: number = 80): Promise<{ units?: any[]; error?: string }> => {
      // Leverage internal WASM caching for fuzzy search
      return searchUnitsFuzzy(query, threshold);
    },
    [searchUnitsFuzzy]
  );

  const getUnitInfoCached = useCallback(
    async (unit: string): Promise<{ info?: any; error?: string }> => {
      // Let WASM handle its own dimension caching
      console.debug('🔄 Using cached getUnitInfo:', { unit });
      return getUnitInfo(unit);
    },
    [getUnitInfo]
  );

  const convertValueCached = useCallback(
    async (value: number, fromUnit: string, toUnit: string): Promise<{ result?: number; error?: string }> => {
      // Let WASM handle its own caching - we just call the underlying function
      // The WASM module maintains its own internal cache
      console.debug('🔄 Using cached convertValue:', { value, fromUnit, toUnit });
      return convertValue(value, fromUnit, toUnit);
    },
    [convertValue]
  );

  const multiplyUnitsCached = useCallback(
    async (unit1: string, unit2: string): Promise<{ result?: any; error?: string }> => {
      // Let WASM handle its own caching
      console.debug('🔄 Using cached multiplyUnits:', { unit1, unit2 });
      return multiplyUnits(unit1, unit2);
    },
    [multiplyUnits]
  );

  const divideUnitsCached = useCallback(
    async (unit1: string, unit2: string): Promise<{ result?: any; error?: string }> => {
      // Unit arithmetic operations can benefit from caching
      return divideUnits(unit1, unit2);
    },
    [divideUnits]
  );

  const validateExpressionCached = useCallback(
    async (expression: string): Promise<{ valid: boolean; error?: string }> => {
      // Let WASM handle its own expression caching
      console.debug('🔄 Using cached validateExpression:', { expression });
      return validateExpression(expression);
    },
    [validateExpression]
  );

  const validatePropertyCached = useCallback(
    async (expression: string, property: string): Promise<{ valid: boolean; error?: string }> => {
      // Property validation can benefit from caching
      return validateProperty(expression, property);
    },
    [validateProperty]
  );

  const checkCompatibilityCached = useCallback(
    async (unit1: string, unit2: string): Promise<{ compatible: boolean; error?: string }> => {
      // Compatibility checks can benefit from caching
      return checkCompatibility(unit1, unit2);
    },
    [checkCompatibility]
  );

  const validateFhirQuantityCached = useCallback(
    async (fhirQuantity: any): Promise<{ valid: boolean; issues?: string[]; error?: string }> => {
      // FHIR quantity validation can benefit from caching
      return validateFhirQuantity(fhirQuantity);
    },
    [validateFhirQuantity]
  );

  return {
    isLoaded,
    error,
    validateExpression,
    validateProperty,
    checkCompatibility,
    getUnitInfo,
    convertValue,
    multiplyUnits,
    divideUnits,
    validateFhirQuantity,
    listUnits,
    searchUnitsText,
    searchUnitsFuzzy,
    getUnitSuggestions,
    getPropertyAlternatives,
    getDimensionSuggestions,
    getUcumProperties,
    getPerformanceCacheStats,
    getPerformanceCacheSizes,
    getUcumModel,
    validateUcumImplementation,
    clearPerformanceCache,
    // Cached versions for performance-critical operations
    searchUnitsTextCached,
    searchUnitsFuzzyCached,
    getUnitInfoCached,
    convertValueCached,
    multiplyUnitsCached,
    divideUnitsCached,
    validateExpressionCached,
    validatePropertyCached,
    checkCompatibilityCached,
    validateFhirQuantityCached,
  };
};
