<script lang="ts">
  import { page } from '$app/stores';

  const navItems = [
    { name: 'Dashboard', path: '/', icon: '🏠' },
    { name: 'Favorites', path: '/favorites', icon: '⭐' },
    { name: 'Custom Profiles', path: '/custom', icon: '⚙️' },
    { name: 'AMS Sync', path: '/ams', icon: '🔄' },
    { name: 'Settings', path: '/settings', icon: '⚙️' },
  ];

  $: currentPath = $page.url.pathname;
</script>

<aside class="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700">
  <div class="p-6 border-b border-gray-200 dark:border-gray-700">
    <h1 class="text-2xl font-bold text-primary">SpoolSync</h1>
    <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Desktop</p>
  </div>

  <nav class="p-4">
    <ul class="space-y-2">
      {#each navItems as item}
        <li>
          <a
            href={item.path}
            class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors {currentPath === item.path
              ? 'bg-primary text-white'
              : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
          >
            <span class="text-xl">{item.icon}</span>
            <span class="font-medium">{item.name}</span>
          </a>
        </li>
      {/each}
    </ul>
  </nav>
</aside>
