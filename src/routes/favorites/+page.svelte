<script lang="ts">
  import { onMount } from 'svelte';
  import Header from '$lib/components/Header.svelte';
  import { favorites, loadFavorites, removeFavorite } from '$lib/stores/filaments';

  onMount(() => {
    loadFavorites();
  });

  async function handleRemove(id: number | undefined) {
    if (!id) return;
    if (confirm('Remove this filament from favorites?')) {
      await removeFavorite(id);
    }
  }
</script>

<div class="flex flex-col h-full">
  <Header title="Favorites" subtitle="Your favorite filament profiles"></Header>

  <div class="p-8 space-y-6 flex-1 overflow-auto">
    {#if $favorites.length === 0}
      <div class="text-center py-16">
        <p class="text-2xl mb-4">⭐</p>
        <p class="text-gray-600 dark:text-gray-400 text-lg">
          No favorites yet
        </p>
        <p class="text-gray-500 dark:text-gray-500 mt-2">
          Browse SpoolmanDB and add your favorite filaments!
        </p>
        <a
          href="/"
          class="inline-block mt-6 px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold"
        >
          Browse Filaments
        </a>
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each $favorites as profile (profile.id)}
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
            <div class="flex items-start gap-4">
              <div
                class="w-16 h-16 rounded-lg flex-shrink-0 border-2 border-gray-200 dark:border-gray-700"
                style="background-color: {profile.color}"
              ></div>
              
              <div class="flex-1">
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                  {profile.brand}
                </h3>
                <p class="text-gray-600 dark:text-gray-400 mt-1">
                  {profile.material}
                </p>
                <div class="flex gap-2 mt-2 text-xs text-gray-500 dark:text-gray-400">
                  <span>🌡️ {profile.nozzle_temp}°C</span>
                  <span>• 🔥 {profile.bed_temp}°C</span>
                </div>
              </div>
            </div>

            <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
              <button
                onclick={() => handleRemove(profile.id)}
                class="w-full px-4 py-2 bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300 rounded-lg hover:bg-red-200 dark:hover:bg-red-900/50 transition-colors text-sm font-medium"
              >
                ❌ Remove
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
