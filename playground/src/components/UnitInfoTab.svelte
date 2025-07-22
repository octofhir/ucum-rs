<script lang="ts">
  import { get_unit_info, search_units_text, search_units_fuzzy, get_unit_forms } from '@octofhir/ucum-wasm';

  // State for unit info tab
  let unitInfoInput = $state('');
  let unitInfo: any = $state(null);
  let unitInfoError = $state('');
  let searchQuery = $state('');
  let searchResults: any[] = $state([]);
  let unitForms: any[] = $state([]);

  // Example units for unit info tab
  const unitInfoExamples = [
    'mg', 'g', 'kg', 'L', 'mL', 'Pa', 'kPa', 'm', 'cm', 'mm', 's', 'min', 'h', 'mol', 'K', '°C'
  ];

  // Get information about a unit
  function handleGetUnitInfo() {
    try {
      unitInfoError = '';
      unitInfo = get_unit_info(unitInfoInput);

      // Also get unit forms if available
      try {
        unitForms = get_unit_forms(unitInfoInput);
      } catch (formsError: any) {
        console.warn('Could not get unit forms:', formsError.message);
        unitForms = [];
      }
    } catch (error: any) {
      unitInfoError = error.message || 'Unknown error';
      unitInfo = null;
      unitForms = [];
    }
  }

  // Search for units
  function handleSearch() {
    if (!searchQuery.trim()) {
      searchResults = [];
      return;
    }

    try {
      unitInfoError = '';
      // Try fuzzy search first
      try {
        const fuzzyResults = search_units_fuzzy(searchQuery, 70); // 70% threshold
        // Convert fuzzy search results to the expected format
        searchResults = fuzzyResults.results.map((match: any) => match.unit);
      } catch (fuzzyError: any) {
        // Fallback to text search
        const textResults = search_units_text(searchQuery);
        // Convert text search results to the expected format
        searchResults = textResults.units;
      }
    } catch (error: any) {
      unitInfoError = error.message || 'Unknown error';
      searchResults = [];
    }
  }

  // Select a unit from search results
  function selectUnit(code: string) {
    unitInfoInput = code;
    handleGetUnitInfo();
    // Hide search results when user clicks on a unit
    searchResults = [];
    searchQuery = '';
  }

  // Function to insert example units into input
  function insertUnitInfoExample(unit: string) {
    unitInfoInput = unit;
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

  .search-results {
    margin-top: var(--space-md);
    padding: var(--space-sm);
    background: var(--color-surface-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    max-height: 300px;
    overflow-y: auto;
  }

  .search-results h4 {
    margin: 0 0 var(--space-sm) 0;
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    color: var(--color-text-secondary);
  }
</style>

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

  <!-- Unit Search Section -->
  <div class="form-group" style="margin-top: 2rem;">
    <h3>Search Units</h3>
    <p class="tab-description">
      Search for units by name, symbol, or description. Uses fuzzy matching to find similar units.
    </p>
    <label for="search-input">Search for units:</label>
    <input
      id="search-input"
      type="text"
      bind:value={searchQuery}
      placeholder="e.g., meter, gram, pressure"
      oninput={handleSearch}
    />

    {#if searchResults.length > 0}
      <div class="search-results">
        <h4>Search Results:</h4>
        <div>
          {#each searchResults.slice(0, 10) as result}
            <button
              class="example-btn"
              onclick={() => selectUnit(result.code)}
              style="margin: 0.125rem; display: block; width: auto; text-align: left; padding: 0.5rem 0.75rem;"
            >
              <strong>{result.code}</strong> - {result.display_name || result.code}
              {#if result.property}
                <span style="color: var(--color-text-secondary); font-size: 0.9em;">
                  ({result.property})
                </span>
              {/if}
            </button>
          {/each}
        </div>
      </div>
    {/if}
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
      <p><strong>Display Name:</strong> {unitInfo.display_name || unitInfo.code}</p>
      <p><strong>Class:</strong> {unitInfo.property || 'Unknown'}</p>
      <p>Factor: {unitInfo.factor}</p>
      <p>Offset: {unitInfo.offset}</p>
      <p>Special: {unitInfo.is_special ? 'Yes' : 'No'}</p>
      <p>Arbitrary: {unitInfo.is_arbitrary ? 'Yes' : 'No'}</p>
      <p>Dimensions: {unitInfo.dimensions.join(', ')}</p>
    </div>
  {/if}

  {#if unitForms.length > 0}
    <div class="result">
      <h3>Unit Forms</h3>
      <p class="tab-description">
        Different representations and related units for <code>{unitInfoInput}</code>:
      </p>
      <div class="example-buttons">
        {#each unitForms as form}
          <button
            class="example-btn"
            onclick={() => selectUnit(form.code)}
            style="margin: 0.125rem;"
          >
            {form.code}
            {#if form.display_name && form.display_name !== form.code}
              <span style="font-size: 0.8em; opacity: 0.8;">
                ({form.display_name})
              </span>
            {/if}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  {#if unitInfoError}
    <div class="error">
      <p>Error: {unitInfoError}</p>
    </div>
  {/if}
</div>
