<script lang="ts">
  import { onMount } from 'svelte';
  import Header from '$lib/components/Header.svelte';
  import { spoolmanColors, spoolmanBrands, spoolmanMaterials, loadColors, loadBrands, loadMaterials, type ColorGroup, type SpoolmanFilament } from '$lib/stores/spoolman';
  import { addFavorite } from '$lib/stores/filaments';
  import type { FilamentProfile } from '$lib/stores/filaments';

  let selectedBrand = '';
  let selectedMaterial = '';
  let showTranslucentOnly = false;
  let showGlowOnly = false;
  let loading = true;

  let selectedColor: ColorGroup | null = null;
  let showModal = false;

  $: filteredColors = Array.from($spoolmanColors.values()).filter(color => {
    if (selectedBrand && !color.brands.has(selectedBrand)) return false;
    if (selectedMaterial && !color.materials.has(selectedMaterial)) return false;
    if (showTranslucentOnly && !color.hasTranslucent) return false;
    if (showGlowOnly && !color.hasGlow) return false;
    return true;
  });

  onMount(async () => {
    await Promise.all([loadColors(), loadBrands(), loadMaterials()]);
    loading = false;
  });

  function openColorModal(color: ColorGroup) {
    selectedColor = color;
    showModal = true;
  }

  function closeModal() {
    showModal = false;
    selectedColor = null;
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
    } catch (error) {
      alert('Failed to add to favorites: ' + error);
    }
  }
</script>

<div class="flex flex-col h-full">
  <Header title="Color Palette" subtitle="Visual gallery of {$spoolmanColors.size} unique colors"></Header>

  <div class="p-8 space-y-6 flex-1 overflow-auto">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <select
          bind:value={selectedBrand}
          class="px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
        >
          <option value="">All Brands ({$spoolmanBrands.length})</option>
          {#each $spoolmanBrands as brand}
            <option value={brand}>{brand}</option>
          {/each}
        </select>

        <select
          bind:value={selectedMaterial}
          class="px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
        >
          <option value="">All Materials ({$spoolmanMaterials.length})</option>
          {#each $spoolmanMaterials as material}
            <option value={material}>{material}</option>
          {/each}
        </select>

        <label class="flex items-center gap-2 px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-600 transition-colors">
          <input type="checkbox" bind:checked={showTranslucentOnly} class="w-4 h-4" />
          <span class="text-sm font-medium">✨ Translucent</span>
        </label>

        <label class="flex items-center gap-2 px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-600 transition-colors">
          <input type="checkbox" bind:checked={showGlowOnly} class="w-4 h-4" />
          <span class="text-sm font-medium">🌟 Glow</span>
        </label>
      </div>
    </div>

    {#if loading}
      <div class="text-center py-16">
        <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
        <p class="text-gray-600 dark:text-gray-400 mt-4">Loading color palette...</p>
      </div>
    {:else if filteredColors.length === 0}
      <div class="text-center py-16">
        <p class="text-2xl mb-4">🎨</p>
        <p class="text-gray-600 dark:text-gray-400 text-lg">
          No colors found
        </p>
        <p class="text-gray-500 dark:text-gray-500 mt-2">
          Try different filters
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8 gap-4">
        {#each filteredColors as color (color.hex)}
          <button
            onclick={() => openColorModal(color)}
            class="group bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-xl transition-all p-3"
          >
            <div
              class="w-full aspect-square rounded-lg mb-2 border-2 border-gray-200 dark:border-gray-700 group-hover:border-primary transition-colors relative"
              style="background-color: #{color.hex}"
            >
              <div class="absolute top-1 right-1 flex gap-1">
                {#if color.hasTranslucent}
                  <span class="text-xs bg-white/80 dark:bg-black/80 px-1 rounded">✨</span>
                {/if}
                {#if color.hasGlow}
                  <span class="text-xs bg-white/80 dark:bg-black/80 px-1 rounded">🌟</span>
                {/if}
              </div>
            </div>
            
            <p class="text-xs text-gray-400 dark:text-gray-500 font-mono text-center">
              #{color.hex.toUpperCase()}
            </p>
            <p class="text-xs text-gray-500 dark:text-gray-400 text-center mt-1">
              {color.filaments.length} filament{color.filaments.length !== 1 ? 's' : ''}
            </p>
          </button>
        {/each}
      </div>

      <div class="text-center text-gray-500 dark:text-gray-400 mt-8">
        <p>Showing {filteredColors.length} of {$spoolmanColors.size} colors</p>
      </div>
    {/if}
  </div>
</div>

{#if showModal && selectedColor}
  <div
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
    onclick={closeModal}
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl max-w-4xl w-full p-8 max-h-[90vh] overflow-y-auto"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="flex items-start gap-6 pb-6 border-b border-gray-200 dark:border-gray-700">
        <div
          class="w-32 h-32 rounded-lg flex-shrink-0 border-2 border-gray-300 dark:border-gray-600"
          style="background-color: #{selectedColor.hex}"
        ></div>
        
        <div class="flex-1">
          <p class="text-3xl font-mono font-bold text-gray-900 dark:text-white">
            #{selectedColor.hex.toUpperCase()}
          </p>
          <div class="flex flex-wrap gap-2 mt-4">
            <span class="px-3 py-1 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 rounded-full text-sm font-medium">
              {selectedColor.filaments.length} filaments
            </span>
            <span class="px-3 py-1 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300 rounded-full text-sm font-medium">
              {selectedColor.materials.size} materials
            </span>
            <span class="px-3 py-1 bg-purple-100 dark:bg-purple-900/30 text-purple-700 dark:text-purple-300 rounded-full text-sm font-medium">
              {selectedColor.brands.size} brands
            </span>
            {#if selectedColor.hasTranslucent}
              <span class="px-3 py-1 bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-300 rounded-full text-sm font-medium">
                ✨ Translucent
              </span>
            {/if}
            {#if selectedColor.hasGlow}
              <span class="px-3 py-1 bg-pink-100 dark:bg-pink-900/30 text-pink-700 dark:text-pink-300 rounded-full text-sm font-medium">
                🌟 Glow in Dark
              </span>
            {/if}
          </div>
          <div class="mt-4">
            <p class="text-sm text-gray-600 dark:text-gray-400">
              <strong>Materials:</strong> {Array.from(selectedColor.materials).join(', ')}
            </p>
          </div>
        </div>
      </div>

      <div class="mt-6">
        <h3 class="text-lg font-bold text-gray-900 dark:text-white mb-4">
          Available Filaments
        </h3>
        <div class="space-y-2 max-h-96 overflow-y-auto">
          {#each selectedColor.filaments as filament (filament.id)}
            <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors">
              <div class="flex-1">
                <p class="font-semibold text-gray-900 dark:text-white">
                  {filament.manufacturer}
                </p>
                <p class="text-sm text-gray-600 dark:text-gray-400">
                  {filament.material} · {filament.name}
                  {#if filament.extruder_temp}
                    · {filament.extruder_temp}°C
                  {/if}
                </p>
              </div>
              <button
                onclick={() => handleAddToFavorites(filament)}
                class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors text-sm font-medium"
              >
                ⭐ Add
              </button>
            </div>
          {/each}
        </div>
      </div>

      <div class="flex gap-3 mt-6 pt-6 border-t border-gray-200 dark:border-gray-700">
        <button
          onclick={closeModal}
          class="flex-1 px-6 py-3 border-2 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors font-semibold"
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}
