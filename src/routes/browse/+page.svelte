<script lang="ts">
  import { onMount } from 'svelte';
  import Header from '$lib/components/Header.svelte';
  import { spoolmanFilaments, spoolmanTotal, spoolmanLoading, spoolmanBrands, spoolmanMaterials, searchSpoolman, loadBrands, loadMaterials, type SpoolmanFilament } from '$lib/stores/spoolman';
  import { addFavorite } from '$lib/stores/filaments';
  import type { FilamentProfile } from '$lib/stores/filaments';

  let searchQuery = '';
  let selectedBrand = '';
  let selectedMaterial = '';
  let currentPage = 0;
  const pageSize = 50;

  let selectedFilament: SpoolmanFilament | null = null;
  let showModal = false;

  onMount(() => {
    loadBrands();
    loadMaterials();
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

  function openModal(filament: SpoolmanFilament) {
    selectedFilament = filament;
    showModal = true;
  }

  function closeModal() {
    showModal = false;
    selectedFilament = null;
  }

  async function handleAddToFavorites(filament: SpoolmanFilament) {
    try {
      const profile: FilamentProfile = {
        brand: filament.manufacturer || 'Unknown',
        material: filament.material || 'PLA',
        color: filament.color_hex ? '#' + filament.color_hex.replace('#', '') : '#888888',
        nozzle_temp: filament.settings_extruder_temp || 220,
        bed_temp: filament.settings_bed_temp || 60,
        density: filament.density || 1.24,
        diameter: filament.diameter || 1.75,
        is_favorite: true,
        is_custom: false,
      };
      
      await addFavorite(profile);
      alert(`Added ${profile.brand} ${profile.material} to favorites!`);
      closeModal();
    } catch (error) {
      alert('Failed to add to favorites: ' + error);
    }
  }
</script>

<div class="flex flex-col h-full">
  <Header title="Browse SpoolmanDB" subtitle="6900+ community filament profiles"></Header>

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
        <option value="">All Brands ({$spoolmanBrands.length})</option>
        {#each $spoolmanBrands as brand}
          <option value={brand}>{brand}</option>
        {/each}
      </select>

      <select
        bind:value={selectedMaterial}
        onchange={handleSearch}
        class="px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
      >
        <option value="">All Materials ({$spoolmanMaterials.length})</option>
        {#each $spoolmanMaterials as material}
          <option value={material}>{material}</option>
        {/each}
      </select>
    </div>

    {#if $spoolmanLoading && $spoolmanFilaments.length === 0}
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
          <button
            onclick={() => openModal(filament)}
            class="bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-lg transition-all p-6 text-left hover:scale-105 cursor-pointer"
          >
            <div class="flex items-start gap-4">
              <div
                class="w-16 h-16 rounded-lg flex-shrink-0 border-2 border-gray-200 dark:border-gray-700"
                style="background-color: {filament.color_hex ? '#' + filament.color_hex.replace('#', '') : '#888888'}"
              ></div>
              
              <div class="flex-1">
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                  {filament.manufacturer || 'Unknown Brand'}
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
          </button>
        {/each}
      </div>

      <div class="text-center mt-8 space-y-4">
        <p class="text-gray-500 dark:text-gray-400">
          Showing {$spoolmanFilaments.length} of {$spoolmanTotal} filaments
        </p>
        
        {#if $spoolmanFilaments.length < $spoolmanTotal}
          <button
            onclick={loadMore}
            disabled={$spoolmanLoading}
            class="px-6 py-3 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors font-medium disabled:opacity-50"
          >
            {$spoolmanLoading ? 'Loading...' : 'Load More'}
          </button>
        {/if}
      </div>
    {/if}
  </div>
</div>

{#if showModal && selectedFilament}
  <div
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
    onclick={closeModal}
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl max-w-2xl w-full p-8"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="flex items-start gap-6">
        <div
          class="w-24 h-24 rounded-lg flex-shrink-0 border-2 border-gray-300 dark:border-gray-600"
          style="background-color: {selectedFilament.color_hex ? '#' + selectedFilament.color_hex.replace('#', '') : '#888888'}"
        ></div>
        
        <div class="flex-1">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            {selectedFilament.manufacturer || 'Unknown Brand'}
          </h2>
          <p class="text-xl text-gray-600 dark:text-gray-400 mt-1">
            {selectedFilament.material || 'Unknown'}
          </p>
          {#if selectedFilament.name}
            <p class="text-gray-500 dark:text-gray-500 mt-2">
              {selectedFilament.name}
            </p>
          {/if}
        </div>
      </div>

      <div class="grid grid-cols-2 gap-6 mt-6">
        <div>
          <p class="text-sm text-gray-500 dark:text-gray-400">Nozzle Temperature</p>
          <p class="text-2xl font-bold text-gray-900 dark:text-white">
            {selectedFilament.settings_extruder_temp || 'N/A'}°C
          </p>
        </div>
        <div>
          <p class="text-sm text-gray-500 dark:text-gray-400">Bed Temperature</p>
          <p class="text-2xl font-bold text-gray-900 dark:text-white">
            {selectedFilament.settings_bed_temp || 'N/A'}°C
          </p>
        </div>
        <div>
          <p class="text-sm text-gray-500 dark:text-gray-400">Density</p>
          <p class="text-2xl font-bold text-gray-900 dark:text-white">
            {selectedFilament.density || 'N/A'} g/cm³
          </p>
        </div>
        <div>
          <p class="text-sm text-gray-500 dark:text-gray-400">Diameter</p>
          <p class="text-2xl font-bold text-gray-900 dark:text-white">
            {selectedFilament.diameter || 'N/A'} mm
          </p>
        </div>
        {#if selectedFilament.weight}
          <div>
            <p class="text-sm text-gray-500 dark:text-gray-400">Net Weight</p>
            <p class="text-2xl font-bold text-gray-900 dark:text-white">
              {selectedFilament.weight}g
            </p>
          </div>
        {/if}
        {#if selectedFilament.spool_weight}
          <div>
            <p class="text-sm text-gray-500 dark:text-gray-400">Spool Weight</p>
            <p class="text-2xl font-bold text-gray-900 dark:text-white">
              {selectedFilament.spool_weight}g
            </p>
          </div>
        {/if}
      </div>

      <div class="flex gap-3 mt-8">
        <button
          onclick={closeModal}
          class="flex-1 px-6 py-3 border-2 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors font-semibold"
        >
          Close
        </button>
        <button
          onclick={() => handleAddToFavorites(selectedFilament!)}
          class="flex-1 px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold"
        >
          ⭐ Add to Favorites
        </button>
      </div>
    </div>
  </div>
{/if}
