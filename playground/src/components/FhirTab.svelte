<script lang="ts">
  import {
    create_fhir_quantity,
    convert_fhir_quantity,
    are_fhir_quantities_equivalent
  } from '@octofhir/ucum-wasm';
  import { onMount } from 'svelte';

  // State for FHIR tab
  let fhirValue = $state(1);
  let fhirUnit = $state('mg');
  let fhirTargetUnit = $state('g');
  let fhirQuantity: any = $state(null);
  let fhirConvertedQuantity: any = $state(null);
  let fhirError = $state('');
  let fhirOperation = $state('create'); // 'create', 'convert', 'equivalent'
  let fhirSecondValue = $state(1000);
  let fhirSecondUnit = $state('mcg');
  let fhirEquivalent: boolean | null = $state(null);

  // Example units for FHIR tab
  const fhirUnitExamples = [
    'mg', 'g', 'kg', 'mL', 'L', 'm', 'cm', 's', 'min', 'h', 'mmol/L', 'mm[Hg]', '°C', 'Cel'
  ];

  // Handle operation changes - trigger appropriate handler when operation changes
  $effect(() => {
    // Only react to operation changes, not input values
    if (fhirOperation === 'create') {
      // Clear previous results when switching to create mode
      fhirConvertedQuantity = null;
      fhirEquivalent = null;
      fhirError = '';
    } else if (fhirOperation === 'convert') {
      // Clear previous results when switching to convert mode
      fhirEquivalent = null;
      fhirError = '';
    } else if (fhirOperation === 'equivalent') {
      // Clear previous results when switching to equivalent mode
      fhirConvertedQuantity = null;
      fhirError = '';
    }
  });

  // Handle FHIR operations
  function handleCreateFhirQuantity() {
    try {
      fhirError = '';
      fhirQuantity = create_fhir_quantity(fhirValue, fhirUnit);
      fhirConvertedQuantity = null;
      fhirEquivalent = null;
    } catch (error: any) {
      fhirError = error.message || 'Unknown error';
      fhirQuantity = null;
    }
  }

  function handleConvertFhirQuantity() {
    try {
      // Always create a new FHIR quantity with current values
      fhirError = '';
      fhirQuantity = create_fhir_quantity(fhirValue, fhirUnit);

      // Then perform the conversion with the fresh quantity
      fhirConvertedQuantity = convert_fhir_quantity(fhirQuantity, fhirTargetUnit);
      fhirEquivalent = null;
    } catch (error: any) {
      fhirError = error.message || 'Unknown error';
      fhirConvertedQuantity = null;
    }
  }

  function handleCheckEquivalent() {
    try {
      fhirError = '';

      // Create the first FHIR quantity if it doesn't exist
      if (!fhirQuantity) {
        handleCreateFhirQuantity();
      }

      // Create the second FHIR quantity
      const secondQuantity = create_fhir_quantity(fhirSecondValue, fhirSecondUnit);

      // Check if they are equivalent
      if (fhirQuantity) {
        fhirEquivalent = are_fhir_quantities_equivalent(fhirQuantity, secondQuantity);
        fhirConvertedQuantity = null;
      }
    } catch (error: any) {
      fhirError = error.message || 'Unknown error';
      fhirEquivalent = null;
    }
  }

  // Functions to insert example units into inputs
  function insertFhirUnitExample(unit: string) {
    fhirUnit = unit;
  }

  function insertFhirTargetUnitExample(unit: string) {
    fhirTargetUnit = unit;
  }

  function insertFhirSecondUnitExample(unit: string) {
    fhirSecondUnit = unit;
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

  .example-row {
    display: grid;
    gap: var(--space-lg);
    grid-template-columns: 1fr;
  }

  @media (min-width: 640px) {
    .example-row {
      grid-template-columns: 1fr 1fr;
    }
  }

  .input-row {
    display: flex;
    flex-direction: column;
    gap: var(--space-2xl);
    width: 100%;
    max-width: 100%;
  }

  @media (min-width: 640px) {
    .input-row {
      flex-direction: row;
      align-items: flex-end;
      flex-wrap: wrap;
    }
  }

  @media (min-width: 1024px) {
    .input-row {
      gap: calc(var(--space-2xl) + var(--space-md));
      flex-wrap: nowrap;
    }
  }

  .input-field {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .input-field input {
    width: 100%;
    max-width: 100%;
  }

  .input-group {
    width: 100%;
    max-width: 100%;
    overflow: hidden;
  }

  .operation-selector {
    margin: var(--space-lg) 0;
    padding: var(--space-md);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    border: 1px solid var(--color-border);
  }

  .radio-button-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  @media (min-width: 640px) {
    .radio-button-group {
      flex-direction: row;
      justify-content: space-between;
    }
  }

  .radio-button-container {
    position: relative;
    display: flex;
    align-items: center;
    padding: var(--space-md) var(--space-lg);
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all var(--transition-fast);
    background: var(--color-surface-elevated);
    border: 1px solid var(--color-border);
    flex: 1;
  }

  .radio-button-container:hover {
    border-color: var(--color-primary);
    background: var(--color-primary-light);
  }

  .radio-button-container.active {
    background: var(--color-primary-light);
    border-color: var(--color-primary);
    box-shadow: 0 0 0 1px var(--color-primary);
  }

  .radio-button-container input[type="radio"] {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .radio-button-custom {
    position: relative;
    display: inline-block;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    border: 2px solid var(--color-border);
    margin-right: var(--space-sm);
    transition: all var(--transition-fast);
  }

  .radio-button-container:hover .radio-button-custom {
    border-color: var(--color-primary);
  }

  .radio-button-container.active .radio-button-custom {
    border-color: var(--color-primary);
  }

  .radio-button-container.active .radio-button-custom::after {
    content: '';
    position: absolute;
    top: 3px;
    left: 3px;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--color-primary);
  }

  .radio-button-label {
    font-weight: 500;
    color: var(--color-text-primary);
  }

  .success {
    color: var(--color-success) !important;
    font-weight: var(--font-medium);
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
  <h2>FHIR Quantity Operations</h2>
  <p class="tab-description">
    This tab demonstrates integration with FHIR (Fast Healthcare Interoperability Resources) Quantity data type.
    You can create FHIR Quantities with UCUM codes, convert between units, and check if quantities are equivalent.
    FHIR Quantities include a value, unit, system URI (<code>http://unitsofmeasure.org</code> for UCUM), and code.
  </p>

  <div class="operation-selector">
    <div class="radio-button-group">
      <label class="radio-button-container" class:active={fhirOperation === 'create'}>
        <input type="radio" bind:group={fhirOperation} value="create" />
        <span class="radio-button-custom"></span>
        <span class="radio-button-label">Create FHIR Quantity</span>
      </label>
      <label class="radio-button-container" class:active={fhirOperation === 'convert'}>
        <input type="radio" bind:group={fhirOperation} value="convert" />
        <span class="radio-button-custom"></span>
        <span class="radio-button-label">Convert FHIR Quantity</span>
      </label>
      <label class="radio-button-container" class:active={fhirOperation === 'equivalent'}>
        <input type="radio" bind:group={fhirOperation} value="equivalent" />
        <span class="radio-button-custom"></span>
        <span class="radio-button-label">Check Equivalence</span>
      </label>
    </div>
  </div>

  <!-- Create FHIR Quantity -->
  {#if fhirOperation === 'create'}
    <div class="input-group">
      <div class="input-row">
        <div class="input-field">
          <label for="fhirValue">Value:</label>
          <input
            type="number"
            id="fhirValue"
            bind:value={fhirValue}
            step="any"
          />
        </div>
        <div class="input-field">
          <label for="fhirUnit">UCUM Unit:</label>
          <input
            type="text"
            id="fhirUnit"
            bind:value={fhirUnit}
            placeholder="e.g., mg"
          />
        </div>
      </div>
    </div>

    <div class="example-units">
      <p class="example-label">Example Units:</p>
      <div class="example-buttons">
        {#each fhirUnitExamples as unit}
          <button
            type="button"
            class="example-btn"
            onclick={() => insertFhirUnitExample(unit)}
          >
            {unit}
          </button>
        {/each}
      </div>
    </div>

    <button onclick={handleCreateFhirQuantity}>Create FHIR Quantity</button>

    {#if fhirQuantity}
      <div class="result">
        <h3>FHIR Quantity:</h3>
        <p>Value: {fhirQuantity.value}</p>
        <p>Unit: {fhirQuantity.unit || 'N/A'}</p>
        <p>System: {fhirQuantity.system || 'N/A'}</p>
        <p>Code: {fhirQuantity.code || 'N/A'}</p>
      </div>
    {/if}
  {/if}

  <!-- Convert FHIR Quantity -->
  {#if fhirOperation === 'convert'}
    <div class="input-group">
      <div class="input-row">
        <div class="input-field">
          <label for="fhirValue">Value:</label>
          <input
            type="number"
            id="fhirValue"
            bind:value={fhirValue}
            step="any"
          />
        </div>
        <div class="input-field">
          <label for="fhirUnit">From Unit:</label>
          <input
            type="text"
            id="fhirUnit"
            bind:value={fhirUnit}
            placeholder="e.g., mg"
          />
        </div>
        <div class="input-field">
          <label for="fhirTargetUnit">To Unit:</label>
          <input
            type="text"
            id="fhirTargetUnit"
            bind:value={fhirTargetUnit}
            placeholder="e.g., g"
          />
        </div>
      </div>
    </div>

    <div class="example-row">
      <div class="example-units">
        <p class="example-label">From Units:</p>
        <div class="example-buttons">
          {#each fhirUnitExamples as unit}
            <button
              type="button"
              class="example-btn"
              onclick={() => insertFhirUnitExample(unit)}
            >
              {unit}
            </button>
          {/each}
        </div>
      </div>

      <div class="example-units">
        <p class="example-label">To Units:</p>
        <div class="example-buttons">
          {#each fhirUnitExamples as unit}
            <button
              type="button"
              class="example-btn"
              onclick={() => insertFhirTargetUnitExample(unit)}
            >
              {unit}
            </button>
          {/each}
        </div>
      </div>
    </div>

    <button onclick={handleConvertFhirQuantity}>Convert</button>

    {#if fhirQuantity}
      <div class="result">
        <h3>Original FHIR Quantity:</h3>
        <p>Value: {fhirQuantity.value}</p>
        <p>Unit: {fhirQuantity.unit || 'N/A'}</p>
        <p>System: {fhirQuantity.system || 'N/A'}</p>
        <p>Code: {fhirQuantity.code || 'N/A'}</p>
      </div>
    {/if}

    {#if fhirConvertedQuantity}
      <div class="result">
        <h3>Converted FHIR Quantity:</h3>
        <p>Value: {fhirConvertedQuantity.value}</p>
        <p>Unit: {fhirConvertedQuantity.unit || 'N/A'}</p>
        <p>System: {fhirConvertedQuantity.system || 'N/A'}</p>
        <p>Code: {fhirConvertedQuantity.code || 'N/A'}</p>
      </div>
    {/if}
  {/if}

  <!-- Check Equivalence -->
  {#if fhirOperation === 'equivalent'}
    <div class="input-group">
      <h3>First Quantity</h3>
      <div class="input-row">
        <div class="input-field">
          <label for="fhirValue">Value:</label>
          <input
            type="number"
            id="fhirValue"
            bind:value={fhirValue}
            step="any"
          />
        </div>
        <div class="input-field">
          <label for="fhirUnit">Unit:</label>
          <input
            type="text"
            id="fhirUnit"
            bind:value={fhirUnit}
            placeholder="e.g., g"
          />
        </div>
      </div>

      <h3>Second Quantity</h3>
      <div class="input-row">
        <div class="input-field">
          <label for="fhirSecondValue">Value:</label>
          <input
            type="number"
            id="fhirSecondValue"
            bind:value={fhirSecondValue}
            step="any"
          />
        </div>
        <div class="input-field">
          <label for="fhirSecondUnit">Unit:</label>
          <input
            type="text"
            id="fhirSecondUnit"
            bind:value={fhirSecondUnit}
            placeholder="e.g., mg"
          />
        </div>
      </div>
    </div>

    <div class="example-row">
      <div class="example-units">
        <p class="example-label">First Unit:</p>
        <div class="example-buttons">
          {#each fhirUnitExamples as unit}
            <button
              type="button"
              class="example-btn"
              onclick={() => insertFhirUnitExample(unit)}
            >
              {unit}
            </button>
          {/each}
        </div>
      </div>

      <div class="example-units">
        <p class="example-label">Second Unit:</p>
        <div class="example-buttons">
          {#each fhirUnitExamples as unit}
            <button
              type="button"
              class="example-btn"
              onclick={() => insertFhirSecondUnitExample(unit)}
            >
              {unit}
            </button>
          {/each}
        </div>
      </div>
    </div>

    <button onclick={handleCheckEquivalent}>Check Equivalence</button>

    {#if fhirEquivalent !== null}
      <div class="result">
        <h3>Equivalence Result:</h3>
        {#if fhirEquivalent}
          <p class="success">The quantities are equivalent!</p>
        {:else}
          <p class="error">The quantities are not equivalent.</p>
        {/if}
      </div>
    {/if}
  {/if}

  {#if fhirError}
    <div class="error">
      <p>Error: {fhirError}</p>
    </div>
  {/if}
</div>
