<script lang="ts">
  import Darktoggle from 'atoms/Darktoggle.svelte';
</script>

<div flex justify-between bg="gray-100 dark:[#212425]" p="1 y-5">
  <p text-2xl font-extrabold px-5>Rust Svelte</p>
  <div flex items-center px-5>
    <Darktoggle />
  </div>
</div>
