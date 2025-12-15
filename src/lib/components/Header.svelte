<script lang="ts">
  export let title: string = 'SpoolSync';
  export let subtitle: string | undefined = undefined;
</script>

<header class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-8 py-6">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-3xl font-bold text-gray-900 dark:text-white">{title}</h2>
      {#if subtitle}
        <p class="text-gray-600 dark:text-gray-400 mt-1">{subtitle}</p>
      {/if}
    </div>

    <div class="flex items-center gap-4">
      <button
        class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
        title="Theme toggle (coming soon)"
      >
        <span class="text-2xl">🌙</span>
      </button>

      <button
        class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-medium"
      >
        Sync All
      </button>
    </div>
  </div>
</header>
