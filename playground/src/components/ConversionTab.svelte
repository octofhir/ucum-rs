<script lang="ts">
  import { convert } from '@octofhir/ucum-wasm';

  // State for conversion tab
  let conversionValue = $state(1);
  let fromUnit = $state('mg');
  let toUnit = $state('g');
  let conversionResult = $state('');
  let conversionError = $state('');

  // Example units for conversion tab
  const conversionFromExamples = [
    'mg', 'g', 'kg', 'lb', 'oz', 'mL', 'L', 'fl_oz', 'kPa', 'mm[Hg]', 'psi', 'm', 'ft', 'in', '°C', '°F'
  ];

  const conversionToExamples = [
    'g', 'kg', 'lb', 'oz', 'mg', 'L', 'mL', 'fl_oz', 'mm[Hg]', 'kPa', 'psi', 'ft', 'm', 'in', '°F', '°C'
  ];

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

  // Functions to insert example units into inputs
  function insertFromUnitExample(unit: string) {
    fromUnit = unit;
  }

  function insertToUnitExample(unit: string) {
    toUnit = unit;
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

  .example-col {
    display: flex;
    flex-direction: column;
  }

  @media (min-width: 640px) {
    .example-row {
      grid-template-columns: 1fr 1fr;
    }
  }

  .row {
    display: grid;
    gap: var(--space-2xl);
    grid-template-columns: 1fr;
  }

  @media (min-width: 640px) {
    .row {
      grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    }
  }

  @media (min-width: 1024px) {
    .row {
      gap: calc(var(--space-2xl) + var(--space-md));
    }
  }

  .col {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
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
