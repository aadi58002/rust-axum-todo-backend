<script lang="ts">
  import Navbar from 'organisms/Navbar.svelte';
  import Footer from 'organisms/Footer.svelte';
  import 'uno.css';
</script>

<Navbar />
<slot />
<Footer />

<style>
  :global(body) {
    --at-apply: 'text-black';
  }
  :global(body.dark) {
    --at-apply: 'bg-[#181A1B] text-white';
  }
</style>
