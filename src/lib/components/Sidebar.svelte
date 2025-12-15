<script lang="ts">
  import { page } from '$app/stores';

  const routes = [
    { path: '/', label: 'Home', icon: '🏠' },
    { path: '/colors', label: 'Colors', icon: '🎨' },
    { path: '/custom', label: 'Custom', icon: '➕' },
    { path: '/settings', label: 'Settings', icon: '⚙️' },
  ];

  $: currentPath = $page.url.pathname;
</script>

<aside class="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col">
  <div class="p-6 border-b border-gray-200 dark:border-gray-700">
    <h1 class="text-2xl font-bold text-gray-900 dark:text-white">SpoolSync</h1>
    <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Filament Manager</p>
  </div>

  <nav class="flex-1 p-4 space-y-2">
    {#each routes as route}
      <a
        href={route.path}
        class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors {currentPath === route.path
          ? 'bg-primary text-white'
          : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
      >
        <span class="text-xl">{route.icon}</span>
        <span class="font-medium">{route.label}</span>
      </a>
    {/each}
  </nav>

  <div class="p-4 border-t border-gray-200 dark:border-gray-700">
    <div class="text-xs text-gray-500 dark:text-gray-400">
      <p>Version 0.1.0</p>
      <p class="mt-1">Phase 2 Complete ✅</p>
    </div>
  </div>
</aside>
