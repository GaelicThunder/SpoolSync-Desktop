<script lang="ts">
  import { onMount } from 'svelte';
  import Header from '$lib/components/Header.svelte';
  import { spoolmanFilaments, spoolmanTotal, spoolmanLoading, spoolmanBrands, spoolmanMaterials, searchSpoolman, loadBrands, loadMaterials, debugFilament, type SpoolmanFilament } from '$lib/stores/spoolman';
  import { addFavorite } from '$lib/stores/filaments';
  import type { FilamentProfile } from '$lib/stores/filaments';

  let searchQuery = '';
  let selectedBrand = '';
  let selectedMaterial = '';
  let currentPage = 0;
  const pageSize = 50;

  let selectedFilament: SpoolmanFilament | null = null;
  let showModal = false;
  let scrollContainer: HTMLDivElement;
  let copyMessage = '';
  let showJsonModal = false;
  let jsonData = '';

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
    if ($spoolmanLoading || $spoolmanFilaments.length >= $spoolmanTotal) return;
    
    currentPage++;
    await searchSpoolman(
      searchQuery || undefined,
      selectedBrand || undefined,
      selectedMaterial || undefined,
      pageSize,
      currentPage * pageSize
    );
  }

  function handleScroll(e: Event) {
    const target = e.target as HTMLDivElement;
    const scrollPercentage = (target.scrollTop + target.clientHeight) / target.scrollHeight;
    
    if (scrollPercentage > 0.8) {
      loadMore();
    }
  }

  function openModal(filament: SpoolmanFilament) {
    selectedFilament = filament;
    showModal = true;
    debugFilament(filament);
  }

  function closeModal() {
    showModal = false;
    selectedFilament = null;
    copyMessage = '';
  }

  function showJson(filament: SpoolmanFilament) {
    const data = {
      version: 1,
      id: filament.id,
      manufacturer: filament.manufacturer,
      name: filament.name,
      material: filament.material,
      density: filament.density,
      diameter: filament.diameter,
      color_hex: filament.color_hex,
      weight: filament.weight,
      spool_weight: filament.spool_weight,
      extruder_temp: filament.extruder_temp,
      bed_temp: filament.bed_temp,
      translucent: filament.translucent,
      glow: filament.glow,
    };
    jsonData = JSON.stringify(data, null, 2);
    showJsonModal = true;
  }

  async function copyJson() {
    try {
      await navigator.clipboard.writeText(jsonData);
      copyMessage = '✅ JSON copied! Share with others.';
      setTimeout(() => copyMessage = '', 3000);
    } catch (err) {
      copyMessage = '❌ Failed to copy';
    }
  }

  function downloadJson(filament: SpoolmanFilament) {
    const blob = new Blob([jsonData], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${filament.manufacturer}_${filament.material}_${filament.name}.json`.replace(/\s+/g, '_');
    a.click();
    URL.revokeObjectURL(url);
  }

  async function handleAddToFavorites(filament: SpoolmanFilament) {
    try {
      const profile: FilamentProfile = {
        brand: filament.manufacturer || 'Unknown',
        material: filament.material || 'PLA',
        color: filament.color_hex ? '#' + filament.color_hex.replace('#', '') : '#888888',
        nozzle_temp: filament.extruder_temp || 220,
        bed_temp: filament.bed_temp || 60,
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
  <Header title="Dashboard" subtitle="Browse 6900+ filament profiles from SpoolmanDB"></Header>

  <div class="p-8 space-y-6 flex-1 overflow-auto" bind:this={scrollContainer} onscroll={handleScroll}>
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
                  {#if filament.extruder_temp}
                    <span>🌡️ {filament.extruder_temp}°C</span>
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

      <div class="text-center mt-8 py-4">
        <p class="text-gray-500 dark:text-gray-400">
          Showing {$spoolmanFilaments.length} of {$spoolmanTotal} filaments
        </p>
        {#if $spoolmanLoading && $spoolmanFilaments.length > 0}
          <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary mt-4"></div>
        {/if}
      </div>
    {/if}
  </div>
</div>

{#if showModal && selectedFilament}
  <div
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
    role="dialog"
    aria-modal="true"
    onclick={closeModal}
    onkeydown={(e) => e.key === 'Escape' && closeModal()}
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl max-w-3xl w-full p-8 max-h-[90vh] overflow-y-auto"
      role="document"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <div class="flex items-start gap-6 pb-6 border-b border-gray-200 dark:border-gray-700">
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
            <p class="text-gray-500 dark:text-gray-500 mt-2 text-lg">
              {selectedFilament.name}
            </p>
          {/if}
          <div class="mt-4">
            <button
              onclick={() => showJson(selectedFilament)}
              class="px-4 py-2 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 rounded-lg hover:bg-blue-200 dark:hover:bg-blue-900/50 transition-colors text-sm font-medium"
            >
              🔗 Export & Share
            </button>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-6 mt-6">
        <div class="bg-gray-50 dark:bg-gray-700/50 p-4 rounded-lg">
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Nozzle Temperature</p>
          <p class="text-2xl font-bold text-gray-900 dark:text-white">
            {selectedFilament.extruder_temp || 'N/A'}{selectedFilament.extruder_temp ? '°C' : ''}
          </p>
        </div>
        <div class="bg-gray-50 dark:bg-gray-700/50 p-4 rounded-lg">
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Bed Temperature</p>
          <p class="text-2xl font-bold text-gray-900 dark:text-white">
            {selectedFilament.bed_temp || 'N/A'}{selectedFilament.bed_temp ? '°C' : ''}
          </p>
        </div>
        <div class="bg-gray-50 dark:bg-gray-700/50 p-4 rounded-lg">
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Density</p>
          <p class="text-2xl font-bold text-gray-900 dark:text-white">
            {selectedFilament.density} g/cm³
          </p>
        </div>
        <div class="bg-gray-50 dark:bg-gray-700/50 p-4 rounded-lg">
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Diameter</p>
          <p class="text-2xl font-bold text-gray-900 dark:text-white">
            {selectedFilament.diameter} mm
          </p>
        </div>
        {#if selectedFilament.weight}
          <div class="bg-gray-50 dark:bg-gray-700/50 p-4 rounded-lg">
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Net Weight</p>
            <p class="text-2xl font-bold text-gray-900 dark:text-white">
              {selectedFilament.weight}g
            </p>
          </div>
        {/if}
        {#if selectedFilament.spool_weight}
          <div class="bg-gray-50 dark:bg-gray-700/50 p-4 rounded-lg">
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Spool Weight</p>
            <p class="text-2xl font-bold text-gray-900 dark:text-white">
              {selectedFilament.spool_weight}g
            </p>
          </div>
        {/if}
        {#if selectedFilament.color_hex}
          <div class="bg-gray-50 dark:bg-gray-700/50 p-4 rounded-lg">
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Color Hex</p>
            <p class="text-xl font-mono font-bold text-gray-900 dark:text-white">
              #{selectedFilament.color_hex.replace('#', '').toUpperCase()}
            </p>
          </div>
        {/if}
        {#if selectedFilament.translucent}
          <div class="bg-gray-50 dark:bg-gray-700/50 p-4 rounded-lg">
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Properties</p>
            <p class="text-lg font-semibold text-gray-900 dark:text-white">
              ✨ Translucent
            </p>
          </div>
        {/if}
        {#if selectedFilament.glow}
          <div class="bg-gray-50 dark:bg-gray-700/50 p-4 rounded-lg">
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Properties</p>
            <p class="text-lg font-semibold text-gray-900 dark:text-white">
              🌟 Glow in Dark
            </p>
          </div>
        {/if}
      </div>

      <div class="flex gap-3 mt-8 pt-6 border-t border-gray-200 dark:border-gray-700">
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

{#if showJsonModal}
  <div
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
    role="dialog"
    aria-modal="true"
    onclick={() => showJsonModal = false}
    onkeydown={(e) => e.key === 'Escape' && (showJsonModal = false)}
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl max-w-2xl w-full p-8"
      role="document"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
        🔗 Share Filament Data
      </h3>
      <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
        Copy this JSON or download it. Others can import it into their SpoolSync app.
      </p>
      <pre class="bg-gray-100 dark:bg-gray-900 p-4 rounded-lg overflow-auto max-h-96 text-sm font-mono text-gray-900 dark:text-gray-100">{jsonData}</pre>
      
      <div class="flex gap-3 mt-6">
        <button
          onclick={() => showJsonModal = false}
          class="flex-1 px-6 py-3 border-2 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors font-semibold"
        >
          Close
        </button>
        <button
          onclick={copyJson}
          class="flex-1 px-6 py-3 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors font-semibold"
        >
          📋 Copy JSON
        </button>
        <button
          onclick={() => downloadJson(selectedFilament!)}
          class="flex-1 px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold"
        >
          📥 Download
        </button>
      </div>
      {#if copyMessage}
        <p class="text-sm text-green-600 dark:text-green-400 mt-3 text-center">{copyMessage}</p>
      {/if}
    </div>
  </div>
{/if}
