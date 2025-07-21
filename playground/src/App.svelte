<script lang="ts">
  import { onMount } from 'svelte';
  import init, {
    start,
    validate,
    get_unit_info,
    convert,
    arithmetic
  } from '@octofhir/ucum-wasm';

  // Theme management
  let isDarkMode = $state(true);

  // Initialize WASM module and theme
  onMount(async () => {
    await init();
    start();

    // Load theme preference from localStorage
    const savedTheme = localStorage.getItem('theme');
    if (savedTheme) {
      isDarkMode = savedTheme === 'dark';
    } else {
      // Default to system preference
      isDarkMode = !window.matchMedia('(prefers-color-scheme: light)').matches;
    }
    updateTheme();
  });

  // Update theme
  function updateTheme() {
    document.documentElement.setAttribute('data-theme', isDarkMode ? 'dark' : 'light');
    localStorage.setItem('theme', isDarkMode ? 'dark' : 'light');
  }

  // Toggle theme
  function toggleTheme() {
    isDarkMode = !isDarkMode;
    updateTheme();
  }

  // State for validation tab
  let validationInput = $state('');
  let validationResult = $state('');
  let validationError = $state('');

  // State for unit info tab
  let unitInfoInput = $state('');
  let unitInfo: any = $state(null);
  let unitInfoError = $state('');

  // State for conversion tab
  let conversionValue = $state(1);
  let fromUnit = $state('mg');
  let toUnit = $state('g');
  let conversionResult = $state('');
  let conversionError = $state('');

  // State for arithmetic tab
  let leftUnit = $state('mg');
  let operation = $state('mul');
  let rightUnit = $state('mL');
  let arithmeticValue = $state(1);
  let arithmeticResult: any = $state(null);
  let arithmeticError = $state('');

  // Active tab
  let activeTab = $state('validation');

  // Validate a UCUM expression
  function handleValidate() {
    try {
      validationError = '';
      const isValid = validate(validationInput);
      validationResult = isValid ? 'Valid UCUM expression' : 'Invalid UCUM expression';
    } catch (error: any) {
      validationError = error.message || 'Unknown error';
      validationResult = '';
    }
  }

  // Get information about a unit
  function handleGetUnitInfo() {
    try {
      unitInfoError = '';
      unitInfo = get_unit_info(unitInfoInput);
    } catch (error: any) {
      unitInfoError = error.message || 'Unknown error';
      unitInfo = null;
    }
  }

  // Convert a value from one unit to another
  function handleConvert() {
    try {
      conversionError = '';
      const result = convert(conversionValue, fromUnit, toUnit);
      conversionResult = `${conversionValue} ${fromUnit} = ${result} ${toUnit}`;
    } catch (error: any) {
      conversionError = error.message || 'Unknown error';
      conversionResult = '';
    }
  }

  // Perform arithmetic operations on units
  function handleArithmetic() {
    try {
      arithmeticError = '';
      arithmeticResult = arithmetic(leftUnit, operation as "mul" | "div", rightUnit, arithmeticValue);
    } catch (error: any) {
      arithmeticError = error.message || 'Unknown error';
      arithmeticResult = null;
    }
  }

  // Example units for different tabs
  const validationExamples = [
    'mg/dL', 'km/h', '°C', 'mm[Hg]', 'L/min', 'kg/m2', 'mol/L', 'Pa.s', 'J/mol', 'Cel'
  ];

  const unitInfoExamples = [
    'mg', 'g', 'kg', 'L', 'mL', 'Pa', 'kPa', 'm', 'cm', 'mm', 's', 'min', 'h', 'mol', 'K', '°C'
  ];

  const conversionFromExamples = [
    'mg', 'g', 'kg', 'lb', 'oz', 'mL', 'L', 'fl_oz', 'kPa', 'mm[Hg]', 'psi', 'm', 'ft', 'in', '°C', '°F'
  ];

  const conversionToExamples = [
    'g', 'kg', 'lb', 'oz', 'mg', 'L', 'mL', 'fl_oz', 'mm[Hg]', 'kPa', 'psi', 'ft', 'm', 'in', '°F', '°C'
  ];

  const arithmeticLeftExamples = [
    'mg', 'g', 'kg', 'mL', 'L', 'm', 'cm', 's', 'min', 'Pa', 'J', 'mol'
  ];

  const arithmeticRightExamples = [
    'mL', 'L', 'g', 'kg', 's', 'min', 'm', 'cm', 'mol', 'K', 'Pa', 'J'
  ];

  // Functions to insert example units into inputs
  function insertValidationExample(unit: string) {
    validationInput = unit;
  }

  function insertUnitInfoExample(unit: string) {
    unitInfoInput = unit;
  }

  function insertFromUnitExample(unit: string) {
    fromUnit = unit;
  }

  function insertToUnitExample(unit: string) {
    toUnit = unit;
  }

  function insertLeftUnitExample(unit: string) {
    leftUnit = unit;
  }

  function insertRightUnitExample(unit: string) {
    rightUnit = unit;
  }
</script>

<main class="container">
  <div class="content-wrapper">
    <div class="header">
      <h1>UCUM Playground</h1>
      <button class="theme-toggle" onclick={toggleTheme} aria-label="Toggle theme">
        {#if isDarkMode}
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 3V4M12 20V21M4 12H3M6.31412 6.31412L5.5 5.5M17.6859 6.31412L18.5 5.5M6.31412 17.69L5.5 18.5M17.6859 17.69L18.5 18.5M21 12H20M16 12C16 14.2091 14.2091 16 12 16C9.79086 16 8 14.2091 8 12C8 9.79086 9.79086 8 12 8C14.2091 8 16 9.79086 16 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        {:else}
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        {/if}
      </button>
    </div>

    <div class="intro">
      <p class="intro-text">
        <strong>UCUM</strong> (Unified Code for Units of Measure) is a standard for representing units of measure in a consistent, unambiguous way.
        This interactive playground lets you explore UCUM's capabilities for validating unit expressions, getting unit information,
        converting between compatible units, and performing arithmetic operations.
      </p>
    </div>

    <div class="tabs">
    <button class:active={activeTab === 'validation'} onclick={() => activeTab = 'validation'}>
      Validation
    </button>
    <button class:active={activeTab === 'unitInfo'} onclick={() => activeTab = 'unitInfo'}>
      Unit Info
    </button>
    <button class:active={activeTab === 'conversion'} onclick={() => activeTab = 'conversion'}>
      Conversion
    </button>
    <button class:active={activeTab === 'arithmetic'} onclick={() => activeTab = 'arithmetic'}>
      Arithmetic
    </button>
  </div>

  <div class="tab-content">
    <!-- Validation Tab -->
    {#if activeTab === 'validation'}
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

        {#if validationError}
          <div class="error">
            <p>Error: {validationError}</p>
          </div>
        {/if}
      </div>
    {/if}

    <!-- Unit Info Tab -->
    {#if activeTab === 'unitInfo'}
      <div class="card">
        <h2>Unit Information</h2>
        <p class="tab-description">
          Get detailed information about a UCUM unit including its conversion factor, dimensions, and properties. Try units like <code>mg</code>, <code>L</code>, or <code>Pa</code>.
        </p>

        <div class="example-units">
          <p class="example-label">Quick examples:</p>
          <div class="example-buttons">
            {#each unitInfoExamples as example}
              <button
                type="button"
                class="example-btn"
                onclick={() => insertUnitInfoExample(example)}
              >
                {example}
              </button>
            {/each}
          </div>
        </div>

        <div class="form-group">
          <label for="unit-info-input">Enter a UCUM unit code:</label>
          <input
            id="unit-info-input"
            type="text"
            bind:value={unitInfoInput}
            placeholder="e.g., mg"
          />
        </div>
        <button onclick={handleGetUnitInfo}>Get Info</button>

        {#if unitInfo}
          <div class="result">
            <h3>Unit: {unitInfo.code}</h3>
            <p><strong>Class:</strong> {unitInfo.property || 'Unknown'}</p>
            <p>Factor: {unitInfo.factor}</p>
            <p>Offset: {unitInfo.offset}</p>
            <p>Special: {unitInfo.is_special ? 'Yes' : 'No'}</p>
            <p>Arbitrary: {unitInfo.is_arbitrary ? 'Yes' : 'No'}</p>
            <p>Dimensions: {unitInfo.dimensions.join(', ')}</p>
          </div>
        {/if}

        {#if unitInfoError}
          <div class="error">
            <p>Error: {unitInfoError}</p>
          </div>
        {/if}
      </div>
    {/if}

    <!-- Conversion Tab -->
    {#if activeTab === 'conversion'}
      <div class="card">
        <h2>Unit Conversion</h2>
        <p class="tab-description">
          Convert values between compatible units. For example, convert <code>100 kPa</code> to <code>mm[Hg]</code> or <code>5 ft</code> to <code>m</code>.
        </p>

        <div class="example-units">
          <div class="example-row">
            <div class="example-col">
              <p class="example-label">From unit examples:</p>
              <div class="example-buttons">
                {#each conversionFromExamples.slice(0, 8) as example}
                  <button
                    type="button"
                    class="example-btn"
                    onclick={() => insertFromUnitExample(example)}
                  >
                    {example}
                  </button>
                {/each}
              </div>
            </div>
            <div class="example-col">
              <p class="example-label">To unit examples:</p>
              <div class="example-buttons">
                {#each conversionToExamples.slice(0, 8) as example}
                  <button
                    type="button"
                    class="example-btn"
                    onclick={() => insertToUnitExample(example)}
                  >
                    {example}
                  </button>
                {/each}
              </div>
            </div>
          </div>
        </div>

        <div class="row">
          <div class="col">
            <div class="form-group">
              <label for="conversion-value">Value:</label>
              <input
                id="conversion-value"
                type="number"
                bind:value={conversionValue}
              />
            </div>
          </div>
          <div class="col">
            <div class="form-group">
              <label for="from-unit">From Unit:</label>
              <input
                id="from-unit"
                type="text"
                bind:value={fromUnit}
                placeholder="e.g., mg"
              />
            </div>
          </div>
          <div class="col">
            <div class="form-group">
              <label for="to-unit">To Unit:</label>
              <input
                id="to-unit"
                type="text"
                bind:value={toUnit}
                placeholder="e.g., g"
              />
            </div>
          </div>
        </div>
        <button onclick={handleConvert}>Convert</button>

        {#if conversionResult}
          <div class="result">
            <p>{conversionResult}</p>
          </div>
        {/if}

        {#if conversionError}
          <div class="error">
            <p>Error: {conversionError}</p>
          </div>
        {/if}
      </div>
    {/if}

    <!-- Arithmetic Tab -->
    {#if activeTab === 'arithmetic'}
      <div class="card">
        <h2>Unit Arithmetic</h2>
        <p class="tab-description">
          Perform arithmetic operations (multiplication or division) on units to create derived units. For example, multiply <code>mg</code> by <code>mL</code> or divide <code>m</code> by <code>s</code>.
        </p>

        <div class="example-units">
          <div class="example-row">
            <div class="example-col">
              <p class="example-label">Left unit examples:</p>
              <div class="example-buttons">
                {#each arithmeticLeftExamples.slice(0, 6) as example}
                  <button
                    type="button"
                    class="example-btn"
                    onclick={() => insertLeftUnitExample(example)}
                  >
                    {example}
                  </button>
                {/each}
              </div>
            </div>
            <div class="example-col">
              <p class="example-label">Right unit examples:</p>
              <div class="example-buttons">
                {#each arithmeticRightExamples.slice(0, 6) as example}
                  <button
                    type="button"
                    class="example-btn"
                    onclick={() => insertRightUnitExample(example)}
                  >
                    {example}
                  </button>
                {/each}
              </div>
            </div>
          </div>
        </div>

        <div class="row">
          <div class="col">
            <div class="form-group">
              <label for="arithmetic-value">Value:</label>
              <input
                id="arithmetic-value"
                type="number"
                bind:value={arithmeticValue}
              />
            </div>
          </div>
          <div class="col">
            <div class="form-group">
              <label for="left-unit">Left Unit:</label>
              <input
                id="left-unit"
                type="text"
                bind:value={leftUnit}
                placeholder="e.g., mg"
              />
            </div>
          </div>
          <div class="col">
            <div class="form-group">
              <label for="operation">Operation:</label>
              <select id="operation" bind:value={operation}>
                <option value="mul">Multiply</option>
                <option value="div">Divide</option>
              </select>
            </div>
          </div>
          <div class="col">
            <div class="form-group">
              <label for="right-unit">Right Unit:</label>
              <input
                id="right-unit"
                type="text"
                bind:value={rightUnit}
                placeholder="e.g., mL"
              />
            </div>
          </div>
        </div>
        <button onclick={handleArithmetic}>Calculate</button>

        {#if arithmeticResult}
          <div class="result">
            <h3>Result:</h3>
            <p>Factor: {arithmeticResult.factor}</p>
            <p>Offset: {arithmeticResult.offset}</p>
            <p>Dimensions: {arithmeticResult.dimensions.join(', ')}</p>
          </div>
        {/if}

        {#if arithmeticError}
          <div class="error">
            <p>Error: {arithmeticError}</p>
          </div>
        {/if}
      </div>
    {/if}
  </div>
  </div>
</main>

<style>
  .intro {
    max-width: 700px;
    margin-bottom: var(--space-xl);
    text-align: center;
  }

  .intro-text {
    font-size: var(--text-lg);
    line-height: 1.7;
    color: var(--color-text-secondary);
    margin: 0;
  }

  .intro-text strong {
    color: var(--color-primary);
    font-weight: 600;
  }

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
    margin: var(--space-lg) 0;
    padding: var(--space-md);
    background: rgba(94, 106, 210, 0.03);
    border: 1px solid rgba(94, 106, 210, 0.08);
    border-radius: var(--radius-md);
  }

  .example-label {
    font-size: var(--text-sm);
    font-weight: 600;
    color: var(--color-text-secondary);
    margin: 0 0 var(--space-sm) 0;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .example-buttons {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-xs);
  }

  .example-btn {
    background: var(--color-surface-elevated);
    border: 1px solid var(--color-border);
    color: var(--color-text-primary);
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-sm);
    font-size: var(--text-xs);
    font-weight: 500;
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    cursor: pointer;
    transition: all var(--transition-fast);
    margin-top: 0;
    min-width: auto;
  }

  .example-btn:hover {
    background: var(--color-primary);
    border-color: var(--color-primary);
    color: white;
    transform: none;
  }

  .example-btn:active {
    transform: scale(0.95);
  }

  .example-row {
    display: grid;
    gap: var(--space-lg);
    grid-template-columns: 1fr;
  }

  .example-col {
    display: flex;
    flex-direction: column;
  }

  @media (min-width: 640px) {
    .example-row {
      grid-template-columns: 1fr 1fr;
    }
  }

  .tabs {
    display: flex;
    gap: var(--space-sm);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: var(--space-sm);
    overflow-x: auto;
    width: 100%;
    max-width: 600px;
  }

  .tabs button {
    margin-top: 0;
    flex: 1;
    min-width: max-content;
    background: transparent;
    color: var(--color-text-secondary);
    border: none;
    padding: var(--space-sm) var(--space-lg);
    border-radius: var(--radius-md);
    cursor: pointer;
    font-weight: 500;
    font-size: var(--text-sm);
    transition: all var(--transition-fast);
    position: relative;
    white-space: nowrap;
  }

  .tabs button:hover {
    background: var(--color-primary-light);
    color: var(--color-text-primary);
  }

  .tabs button.active {
    background: var(--color-primary);
    color: white;
    box-shadow: none;
  }


  .tab-content {
    background: transparent;
    border-radius: var(--radius-xl);
  }

  .error {
    margin-top: var(--space-lg);
    padding: var(--space-lg);
    background: var(--color-error-bg);
    border: 1px solid var(--color-error);
    border-radius: var(--radius-lg);
    color: var(--color-error);
    position: relative;
    overflow: hidden;
  }

  .error::before {
    content: '⚠';
    position: absolute;
    top: var(--space-md);
    left: var(--space-md);
    font-size: var(--text-lg);
    opacity: 0.7;
  }

  .error p {
    margin: 0;
    padding-left: var(--space-xl);
    font-weight: 500;
  }

  /* Header styles */
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    max-width: 900px;
    margin-bottom: var(--space-lg);
  }

  .header h1 {
    margin: 0;
    flex: 1;
  }

  .theme-toggle {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    color: var(--color-text-primary);
    padding: var(--space-sm);
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all var(--transition-fast);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 44px;
    height: 44px;
    margin: 0;
    box-shadow: var(--shadow-sm);
  }

  .theme-toggle:hover {
    background: var(--color-surface-elevated);
    border-color: var(--color-primary);
    transform: translateY(-1px);
    box-shadow: var(--shadow-md);
  }

  .theme-toggle:active {
    transform: translateY(0);
  }

  .theme-toggle svg {
    transition: all var(--transition-fast);
  }

  @media (max-width: 640px) {
    .header {
      flex-direction: column;
      gap: var(--space-md);
      text-align: center;
    }

    .header h1 {
      order: 1;
    }

    .theme-toggle {
      order: 2;
      align-self: center;
    }
  }

  /* Mobile responsiveness for tabs */
  @media (max-width: 640px) {
    .tabs {
      flex-direction: column;
      gap: var(--space-sm);
    }

    .tabs button {
      flex: none;
      text-align: center;
    }

    .tabs button.active::after {
      display: none;
    }
  }
</style>
