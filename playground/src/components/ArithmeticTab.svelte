<script lang="ts">
  import { arithmetic } from '@octofhir/ucum-wasm';

  // State for arithmetic tab
  let leftUnit = $state('mg');
  let operation = $state('mul');
  let rightUnit = $state('mL');
  let arithmeticValue = $state(1);
  let arithmeticResult: any = $state(null);
  let arithmeticError = $state('');

  // Example units for arithmetic tab
  const arithmeticLeftExamples = [
    'mg', 'g', 'kg', 'mL', 'L', 'm', 'cm', 's', 'min', 'Pa', 'J', 'mol'
  ];

  const arithmeticRightExamples = [
    'mL', 'L', 'g', 'kg', 's', 'min', 'm', 'cm', 'mol', 'K', 'Pa', 'J'
  ];

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

  // Functions to insert example units into inputs
  function insertLeftUnitExample(unit: string) {
    leftUnit = unit;
  }

  function insertRightUnitExample(unit: string) {
    rightUnit = unit;
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
