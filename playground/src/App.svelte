<script lang="ts">
  import { onMount } from 'svelte';
  import init, { start } from '@octofhir/ucum-wasm';

  // Import tab components
  import ValidationTab from './components/ValidationTab.svelte';
  import UnitInfoTab from './components/UnitInfoTab.svelte';
  import ConversionTab from './components/ConversionTab.svelte';
  import ArithmeticTab from './components/ArithmeticTab.svelte';
  import FhirTab from './components/FhirTab.svelte';

  // Theme management
  let isDarkMode = $state(true);

  // WASM initialization state
  let wasmReady = $state(false);

  // Initialize WASM module and theme
  onMount(async () => {
    try {
      await init();
      start();
      wasmReady = true;
      console.log('✅ WASM module initialized successfully');
    } catch (error) {
      console.error('❌ Failed to initialize WASM module:', error);
    }

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

  // Active tab
  let activeTab = $state('validation');
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
      <button class:active={activeTab === 'fhir'} onclick={() => activeTab = 'fhir'}>
        FHIR
      </button>
    </div>

    <div class="tab-content">
      {#if !wasmReady}
        <div class="loading">
          <div class="loading-spinner"></div>
          <p>Initializing UCUM WebAssembly module...</p>
        </div>
      {:else}
        <!-- Validation Tab -->
        {#if activeTab === 'validation'}
          <ValidationTab />
        {/if}

        <!-- Unit Info Tab -->
        {#if activeTab === 'unitInfo'}
          <UnitInfoTab />
        {/if}

        <!-- Conversion Tab -->
        {#if activeTab === 'conversion'}
          <ConversionTab />
        {/if}

        <!-- Arithmetic Tab -->
        {#if activeTab === 'arithmetic'}
          <ArithmeticTab />
        {/if}

        <!-- FHIR Tab -->
        {#if activeTab === 'fhir'}
          <FhirTab />
        {/if}
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

  .tabs {
    display: flex;
    gap: var(--space-xs);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-xs);
    overflow-x: auto;
    width: 100%;
    max-width: 700px;
    margin-bottom: var(--space-xl);
    box-shadow: var(--shadow-sm);
  }

  .tabs button {
    margin: 0;
    flex: 1;
    min-width: max-content;
    background: transparent;
    color: var(--color-text-secondary);
    border: none;
    padding: var(--space-sm) var(--space-lg);
    border-radius: var(--radius-md);
    cursor: pointer;
    font-weight: var(--font-medium);
    font-size: var(--text-sm);
    transition: all var(--transition-fast);
    position: relative;
    white-space: nowrap;
    height: 36px;
    min-height: 36px;
    box-shadow: none;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .tabs button:hover {
    background: var(--color-primary-light);
    color: var(--color-text-primary);
    transform: none;
  }

  .tabs button.active {
    background: var(--color-primary);
    color: var(--color-text-on-primary);
    box-shadow: var(--shadow-sm);
    font-weight: var(--font-semibold);
  }

  .tabs button.active::after {
    content: '';
    position: absolute;
    bottom: -4px;
    left: 50%;
    transform: translateX(-50%);
    width: 16px;
    height: 2px;
    background-color: var(--color-primary);
    border-radius: var(--radius-full);
  }

  .tab-content {
    background: transparent;
    border-radius: var(--radius-xl);
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
      flex-direction: row;
      flex-wrap: wrap;
      gap: var(--space-xs);
      padding: var(--space-xs);
      max-width: 100%;
      justify-content: center;
    }

    .tabs button {
      flex: 0 1 auto;
      text-align: center;
      min-width: 120px;
      margin: var(--space-xxs);
      font-size: var(--text-xs);
      padding: var(--space-xs) var(--space-sm);
    }

    .tabs button.active::after {
      bottom: -2px;
      width: 12px;
      height: 2px;
    }
  }

  /* Extra small screens */
  @media (max-width: 380px) {
    .tabs {
      flex-direction: column;
      align-items: stretch;
    }

    .tabs button {
      width: 100%;
      min-width: 100%;
      margin: var(--space-xxs) 0;
    }
  }

  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-xxl);
    text-align: center;
  }

  .loading-spinner {
    width: 40px;
    height: 40px;
    border: 3px solid var(--color-border);
    border-top: 3px solid var(--color-primary);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: var(--space-lg);
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .loading p {
    color: var(--color-text-secondary);
    font-size: var(--text-base);
    margin: 0;
  }
</style>
