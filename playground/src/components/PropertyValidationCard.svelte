<script lang="ts">
  import { validate_property } from '@octofhir/ucum-wasm';

  // Local state for property validation
  let validationInput = $state('');
  let propertyValidation = $state('');
  let validationResult = $state('');
  let validationError = $state('');

  // Reactive state for button disabled status
  let isPropertyValidationDisabled = $state(true);

  // Update disabled state reactively using $effect
  $effect(() => {
    isPropertyValidationDisabled = !validationInput || !propertyValidation;
  });

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
  <h3>Property Validation</h3>
  <p class="tab-description">
    Validate if a unit belongs to a specific property (dimension). Common properties: <code>length</code>, <code>mass</code>, <code>time</code>, <code>temperature</code>, <code>volume</code>.
  </p>

  <div class="form-group">
    <label for="property-unit-input">Enter a UCUM expression:</label>
    <input
      id="property-unit-input"
      type="text"
      bind:value={validationInput}
      placeholder="e.g., mg/dL, km/h"
    />
  </div>

  <div class="form-group">
    <label for="property-input">Property to validate against:</label>
    <input
      id="property-input"
      type="text"
      bind:value={propertyValidation}
      placeholder="e.g., length, mass, volume"
    />
  </div>

  <button onclick={handlePropertyValidation} disabled={isPropertyValidationDisabled}>
    Validate Property
  </button>

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
