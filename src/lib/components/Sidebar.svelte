<script lang="ts">
  import { page } from '$app/stores';

  const navItems = [
    { href: '/browse', label: 'Browse', icon: '🔍' },
    { href: '/ams', label: 'AMS Status', icon: '🎛️' },
    { href: '/', label: 'Favorites', icon: '⭐' },
    { href: '/custom', label: 'Custom', icon: '➕' },
    { href: '/colors', label: 'Colors', icon: '🎨' },
    { href: '/settings', label: 'Settings', icon: '⚙️' },
  ];

  $: currentPath = $page.url.pathname;
</script>

<aside class="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col">
  <div class="p-6">
    <h1 class="text-2xl font-bold text-primary">🧵 SpoolSync</h1>
    <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">Desktop Edition</p>
  </div>

  <nav class="flex-1 px-4">
    <ul class="space-y-2">
      {#each navItems as item}
        <li>
          <a
            href={item.href}
            class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors {currentPath === item.href
              ? 'bg-primary text-white'
              : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
          >
            <span class="text-xl">{item.icon}</span>
            <span class="font-medium">{item.label}</span>
          </a>
        </li>
      {/each}
    </ul>
  </nav>

  <div class="p-4 border-t border-gray-200 dark:border-gray-700">
    <p class="text-xs text-gray-500 dark:text-gray-400 text-center">
      v0.1.0 Beta
    </p>
  </div>
</aside>
