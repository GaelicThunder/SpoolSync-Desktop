<script lang="ts">
  import { onMount } from 'svelte';
  import Header from '$lib/components/Header.svelte';
  import { favorites, loadFavorites, removeFavorite } from '$lib/stores/filaments';
  
  let searchQuery = '';
  
  onMount(() => {
    loadFavorites();
  });

  $: filteredFilaments = $favorites.filter(f => 
    f.brand.toLowerCase().includes(searchQuery.toLowerCase()) ||
    f.material.toLowerCase().includes(searchQuery.toLowerCase())
  );

  async function handleRemoveFavorite(id: number) {
    if (confirm('Remove from favorites?')) {
      await removeFavorite(id);
    }
  }
</script>

<div class="flex flex-col h-full">
  <Header title="Filament Library" subtitle="Browse and sync your filaments" />

  <div class="p-8 space-y-6">
    <div class="flex gap-4">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search filaments by brand, material, or color..."
        class="flex-1 px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
      />
      
      <button class="px-6 py-3 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors font-medium">
        Filters
      </button>
    </div>

    {#if filteredFilaments.length === 0}
      <div class="text-center py-16">
        <p class="text-2xl mb-4">⭐</p>
        <p class="text-gray-600 dark:text-gray-400 text-lg">
          No favorites yet!
        </p>
        <p class="text-gray-500 dark:text-gray-500 mt-2">
          Create custom profiles or browse SpoolmanDB to add favorites
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each filteredFilaments as filament (filament.id)}
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-lg transition-shadow p-6">
            <div class="flex items-start gap-4">
              <div
                class="w-16 h-16 rounded-lg flex-shrink-0 border-2 border-gray-200 dark:border-gray-700"
                style="background-color: {filament.color}"
              />
              
              <div class="flex-1">
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                  {filament.brand}
                </h3>
                <p class="text-gray-600 dark:text-gray-400 mt-1">
                  {filament.material}
                </p>
                <p class="text-sm text-gray-500 dark:text-gray-500 mt-2">
                  {filament.nozzle_temp}°C
                </p>
                {#if filament.is_custom}
                  <span class="inline-block mt-2 px-2 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 text-xs rounded">
                    Custom
                  </span>
                {/if}
              </div>
            </div>

            <div class="flex gap-2 mt-4">
              <button class="flex-1 px-4 py-2 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors text-sm font-medium">
                Sync to AMS
              </button>
              <button 
                onclick={() => handleRemoveFavorite(filament.id!)}
                class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors text-sm"
                title="Remove from favorites"
              >
                ❌
              </button>
            </div>
          </div>
        {/each}
      </div>

      <div class="text-center text-gray-500 dark:text-gray-400 mt-8">
        <p>Showing {filteredFilaments.length} filament{filteredFilaments.length !== 1 ? 's' : ''}</p>
      </div>
    {/if}
  </div>
</div>
