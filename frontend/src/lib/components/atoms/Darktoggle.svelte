<script lang="ts">
  import { theme } from '$lib/stores/themes';
</script>

{#key $theme}
  <button aria-label="Dark Mode Switch" class="bg-transparent border-0 dark:text-white" on:click={theme.toggle}>
    <div class="i-carbon-sun dark:i-carbon-moon" />
  </button>
{/key}
