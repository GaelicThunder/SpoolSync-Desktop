<script lang="ts">
  import '../app.css';
  import Sidebar from '$lib/components/Sidebar.svelte';
</script>

<div class="flex h-screen bg-gray-50 dark:bg-gray-900">
  <Sidebar />
  
  <main class="flex-1 overflow-auto">
    <slot />
  </main>
</div>
