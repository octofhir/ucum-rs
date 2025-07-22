<script lang="ts">
  import BasicValidationCard from './BasicValidationCard.svelte';
  import PropertyValidationCard from './PropertyValidationCard.svelte';
  import UnitCompatibilityCard from './UnitCompatibilityCard.svelte';
</script>

<style>
  .validation-container {
    display: flex;
    flex-direction: column;
    gap: var(--space-xl);
  }
</style>

<div class="validation-container">
  <BasicValidationCard />
  <PropertyValidationCard />
  <UnitCompatibilityCard />
</div>
