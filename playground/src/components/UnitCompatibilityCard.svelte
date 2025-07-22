<script lang="ts">
  import { units_comparable } from '@octofhir/ucum-wasm';

  // Local state for unit compatibility
  let validationInput = $state('');
  let comparisonUnit = $state('');
  let comparisonResult = $state('');
  let validationError = $state('');

  // Reactive state for button disabled status
  let isComparisonDisabled = $state(true);

  // Update disabled state reactively using $effect
  $effect(() => {
    isComparisonDisabled = !validationInput || !comparisonUnit;
  });

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
  <h3>Unit Compatibility</h3>
  <p class="tab-description">
    Check if two units are comparable (can be converted between each other).
  </p>

  <div class="form-group">
    <label for="compatibility-unit1-input">First unit:</label>
    <input
      id="compatibility-unit1-input"
      type="text"
      bind:value={validationInput}
      placeholder="e.g., m, kg, L"
    />
  </div>

  <div class="form-group">
    <label for="compatibility-unit2-input">Compare with unit:</label>
    <input
      id="compatibility-unit2-input"
      type="text"
      bind:value={comparisonUnit}
      placeholder="e.g., cm, kg, L"
    />
  </div>

  <button onclick={handleUnitComparison} disabled={isComparisonDisabled}>
    Check Compatibility
  </button>

  {#if comparisonResult}
    <div class="result">
      <p>{comparisonResult}</p>
    </div>
  {/if}

  {#if validationError}
    <div class="error">
      <p>Error: {validationError}</p>
    </div>
  {/if}
</div>
