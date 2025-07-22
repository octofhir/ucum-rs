<script lang="ts">
  import { validate, analyze_unit, validate_property, units_comparable } from '@octofhir/ucum-wasm';

  // State for validation tab
  let validationInput = $state('');
  let validationResult = $state('');
  let validationError = $state('');
  let unitAnalysis: any = $state(null);
  let propertyValidation = $state('');
  let comparisonUnit = $state('');
  let comparisonResult = $state('');

  // Example units for validation tab
  const validationExamples = [
    'mg/dL', 'km/h', '°C', 'mm[Hg]', 'L/min', 'kg/m2', 'mol/L', 'Pa.s', 'J/mol', 'Cel'
  ];

  // Validate a UCUM expression
  function handleValidate() {
    try {
      validationError = '';
      const isValid = validate(validationInput);
      validationResult = isValid ? 'Valid UCUM expression' : 'Invalid UCUM expression';

      // If valid, also get detailed analysis
      if (isValid) {
        try {
          unitAnalysis = analyze_unit(validationInput);
        } catch (analysisError: any) {
          console.warn('Analysis failed:', analysisError.message);
          unitAnalysis = null;
        }
      } else {
        unitAnalysis = null;
      }
    } catch (error: any) {
      validationError = error.message || 'Unknown error';
      validationResult = '';
      unitAnalysis = null;
    }
  }

  // Validate unit against a specific property
  function handlePropertyValidation() {
    if (!validationInput || !propertyValidation) return;

    try {
      const isValidForProperty = validate_property(validationInput, propertyValidation);
      validationResult = isValidForProperty
        ? `Valid for property: ${propertyValidation}`
        : `Invalid for property: ${propertyValidation}`;
      validationError = '';
    } catch (error: any) {
      validationError = error.message || 'Unknown error';
      validationResult = '';
    }
  }

  // Compare two units for compatibility
  function handleUnitComparison() {
    if (!validationInput || !comparisonUnit) return;

    try {
      const areComparable = units_comparable(validationInput, comparisonUnit);
      comparisonResult = areComparable
        ? `Units are comparable/convertible`
        : `Units are not comparable`;
      validationError = '';
    } catch (error: any) {
      validationError = error.message || 'Unknown error';
      comparisonResult = '';
    }
  }

  // Function to insert example units into input
  function insertValidationExample(unit: string) {
    validationInput = unit;
  }
</script>

<style>
  .tab-description {
    font-size: var(--text-base);
    line-height: 1.6;
    color: var(--color-text-secondary);
    margin: 0 0 var(--space-lg) 0;
    padding: var(--space-md);
    background: rgba(94, 106, 210, 0.05);
    border: 1px solid rgba(94, 106, 210, 0.1);
    border-radius: var(--radius-md);
  }

  .tab-description code {
    background: rgba(94, 106, 210, 0.15);
    color: var(--color-primary);
    padding: 0.2em 0.4em;
    border-radius: var(--radius-sm);
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    font-size: 0.9em;
    font-weight: 500;
  }

  .example-units {
    margin: var(--space-xl) 0;
    padding: var(--space-lg);
    background: var(--color-primary-light);
    border: 1px solid var(--color-primary-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
  }

  .example-label {
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    color: var(--color-text-secondary);
    margin: 0 0 var(--space-md) 0;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    display: flex;
    align-items: center;
  }

  .example-label::before {
    content: '✨';
    margin-right: var(--space-sm);
    font-size: var(--text-base);
  }

  .example-buttons {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-sm);
  }

  .example-btn {
    background: var(--color-surface-elevated);
    border: 1px solid var(--color-border);
    color: var(--color-text-primary);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    cursor: pointer;
    transition: all var(--transition-fast);
    margin-top: 0;
    min-width: auto;
    box-shadow: var(--shadow-sm);
    height: 32px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .example-btn:hover {
    background: var(--color-primary);
    border-color: var(--color-primary);
    color: var(--color-text-on-primary);
    transform: translateY(-1px);
    box-shadow: var(--shadow-md);
  }

  .example-btn:active {
    transform: translateY(0);
    box-shadow: var(--shadow-sm);
  }

  .error {
    margin-top: var(--space-xl);
    padding: var(--space-lg) var(--space-xl);
    background: var(--color-error-bg);
    border: 1px solid var(--color-error-border);
    border-left: 3px solid var(--color-error);
    border-radius: var(--radius-lg);
    color: var(--color-error);
    position: relative;
    overflow: hidden;
    box-shadow: var(--shadow-md);
    animation: fadeIn var(--transition-normal);
  }

  .error::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 3px;
    height: 100%;
    background: var(--color-error);
    opacity: 0.8;
  }

  .error p {
    margin: 0;
    padding-left: var(--space-md);
    font-weight: var(--font-medium);
    display: flex;
    align-items: center;
    line-height: 1.6;
  }

  .error p::before {
    content: '⚠';
    display: inline-block;
    font-size: var(--text-lg);
    margin-right: var(--space-md);
    opacity: 0.9;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>

<div class="card">
  <h2>Validate UCUM Expression</h2>
  <p class="tab-description">
    Check if a unit expression follows UCUM syntax rules. Try expressions like <code>mg/dL</code>, <code>km/h</code>, or <code>°C</code>.
  </p>

  <div class="example-units">
    <p class="example-label">Quick examples:</p>
    <div class="example-buttons">
      {#each validationExamples as example}
        <button
          type="button"
          class="example-btn"
          onclick={() => insertValidationExample(example)}
        >
          {example}
        </button>
      {/each}
    </div>
  </div>

  <div class="form-group">
    <label for="validation-input">Enter a UCUM expression:</label>
    <input
      id="validation-input"
      type="text"
      bind:value={validationInput}
      placeholder="e.g., mg/dL"
    />
  </div>
  <button onclick={handleValidate}>Validate</button>

  {#if validationResult}
    <div class="result">
      <p>{validationResult}</p>
    </div>
  {/if}

  {#if unitAnalysis}
    <div class="result">
      <h3>Unit Analysis</h3>
      <p><strong>Canonical Form:</strong> {unitAnalysis.canonical || 'N/A'}</p>
      <p><strong>Property:</strong> {unitAnalysis.property || 'N/A'}</p>
      <p><strong>Dimension:</strong> {JSON.stringify(unitAnalysis.dimension)}</p>
      <p><strong>Factor:</strong> {unitAnalysis.factor}</p>
      {#if unitAnalysis.offset !== 0}
        <p><strong>Offset:</strong> {unitAnalysis.offset}</p>
      {/if}
    </div>
  {/if}

  <!-- Property Validation Section -->
  <div class="form-group" style="margin-top: 2rem;">
    <h3>Property Validation</h3>
    <p class="tab-description">
      Validate if a unit belongs to a specific property (dimension). Common properties: <code>length</code>, <code>mass</code>, <code>time</code>, <code>temperature</code>, <code>volume</code>.
    </p>
    <label for="property-input">Property to validate against:</label>
    <input
      id="property-input"
      type="text"
      bind:value={propertyValidation}
      placeholder="e.g., length, mass, volume"
    />
    <button onclick={handlePropertyValidation} disabled={!validationInput || !propertyValidation}>
      Validate Property
    </button>
  </div>

  <!-- Unit Comparison Section -->
  <div class="form-group" style="margin-top: 2rem;">
    <h3>Unit Compatibility</h3>
    <p class="tab-description">
      Check if two units are comparable (can be converted between each other).
    </p>
    <label for="comparison-input">Compare with unit:</label>
    <input
      id="comparison-input"
      type="text"
      bind:value={comparisonUnit}
      placeholder="e.g., cm, kg, L"
    />
    <button onclick={handleUnitComparison} disabled={!validationInput || !comparisonUnit}>
      Check Compatibility
    </button>

    {#if comparisonResult}
      <div class="result">
        <p>{comparisonResult}</p>
      </div>
    {/if}
  </div>

  {#if validationError}
    <div class="error">
      <p>Error: {validationError}</p>
    </div>
  {/if}
</div>
