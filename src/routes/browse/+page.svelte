<script lang="ts">
  import { onMount } from 'svelte';
  import Header from '$lib/components/Header.svelte';
  import { spoolmanFilaments, spoolmanTotal, spoolmanLoading, spoolmanBrands, searchSpoolman, loadBrands } from '$lib/stores/spoolman';
  import { addFavorite } from '$lib/stores/filaments';
  import type { FilamentProfile } from '$lib/stores/filaments';

  let searchQuery = '';
  let selectedBrand = '';
  let selectedMaterial = '';
  let currentPage = 0;
  const pageSize = 50;

  onMount(() => {
    loadBrands();
    handleSearch();
  });

  async function handleSearch() {
    currentPage = 0;
    await searchSpoolman(
      searchQuery || undefined,
      selectedBrand || undefined,
      selectedMaterial || undefined,
      pageSize,
      0
    );
  }

  async function loadMore() {
    currentPage++;
    await searchSpoolman(
      searchQuery || undefined,
      selectedBrand || undefined,
      selectedMaterial || undefined,
      pageSize,
      currentPage * pageSize
    );
  }

  async function handleAddToFavorites(filament: any) {
    try {
      const profile: FilamentProfile = {
        brand: filament.vendor?.name || 'Unknown',
        material: filament.material || 'PLA',
        color: filament.color_hex || '#888888',
        nozzle_temp: filament.settings_extruder_temp || 220,
        bed_temp: filament.settings_bed_temp || 60,
        density: filament.density || 1.24,
        diameter: filament.diameter || 1.75,
        is_favorite: true,
        is_custom: false,
      };
      
      await addFavorite(profile);
      alert(`Added ${profile.brand} ${profile.material} to favorites!`);
    } catch (error) {
      alert('Failed to add to favorites: ' + error);
    }
  }
</script>

<div class="flex flex-col h-full">
  <Header title="Browse SpoolmanDB" subtitle="1000+ community filament profiles"></Header>

  <div class="p-8 space-y-6">
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <input
        type="text"
        bind:value={searchQuery}
        onkeydown={(e) => e.key === 'Enter' && handleSearch()}
        placeholder="Search by name, brand, material..."
        class="md:col-span-2 px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
      />

      <select
        bind:value={selectedBrand}
        onchange={handleSearch}
        class="px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
      >
        <option value="">All Brands</option>
        {#each $spoolmanBrands as brand}
          <option value={brand}>{brand}</option>
        {/each}
      </select>

      <select
        bind:value={selectedMaterial}
        onchange={handleSearch}
        class="px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
      >
        <option value="">All Materials</option>
        <option value="PLA">PLA</option>
        <option value="PETG">PETG</option>
        <option value="ABS">ABS</option>
        <option value="TPU">TPU</option>
        <option value="ASA">ASA</option>
        <option value="Nylon">Nylon</option>
        <option value="PC">Polycarbonate</option>
      </select>
    </div>

    {#if $spoolmanLoading}
      <div class="text-center py-16">
        <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
        <p class="text-gray-600 dark:text-gray-400 mt-4">Loading filaments...</p>
      </div>
    {:else if $spoolmanFilaments.length === 0}
      <div class="text-center py-16">
        <p class="text-2xl mb-4">🔍</p>
        <p class="text-gray-600 dark:text-gray-400 text-lg">
          No filaments found
        </p>
        <p class="text-gray-500 dark:text-gray-500 mt-2">
          Try different search terms or filters
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each $spoolmanFilaments as filament (filament.id)}
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-lg transition-shadow p-6">
            <div class="flex items-start gap-4">
              <div
                class="w-16 h-16 rounded-lg flex-shrink-0 border-2 border-gray-200 dark:border-gray-700"
                style="background-color: {filament.color_hex ? '#' + filament.color_hex.replace('#', '') : '#888888'}"
              ></div>
              
              <div class="flex-1">
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                  {filament.vendor?.name || 'Unknown Brand'}
                </h3>
                <p class="text-gray-600 dark:text-gray-400 mt-1">
                  {filament.material || 'Unknown'}
                </p>
                {#if filament.name}
                  <p class="text-sm text-gray-500 dark:text-gray-500 mt-1">
                    {filament.name}
                  </p>
                {/if}
                <div class="flex gap-2 mt-2 text-xs text-gray-500 dark:text-gray-400">
                  {#if filament.settings_extruder_temp}
                    <span>🌡️ {filament.settings_extruder_temp}°C</span>
                  {/if}
                  {#if filament.diameter}
                    <span>• {filament.diameter}mm</span>
                  {/if}
                </div>
              </div>
            </div>

            <button
              onclick={() => handleAddToFavorites(filament)}
              class="w-full mt-4 px-4 py-2 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors text-sm font-medium"
            >
              ⭐ Add to Favorites
            </button>
          </div>
        {/each}
      </div>

      <div class="text-center mt-8 space-y-4">
        <p class="text-gray-500 dark:text-gray-400">
          Showing {$spoolmanFilaments.length} of {$spoolmanTotal} filaments
        </p>
        
        {#if $spoolmanFilaments.length < $spoolmanTotal}
          <button
            onclick={loadMore}
            class="px-6 py-3 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors font-medium"
          >
            Load More
          </button>
        {/if}
      </div>
    {/if}
  </div>
</div>
