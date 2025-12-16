<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import Header from '$lib/components/Header.svelte';
  import { addFavorite } from '$lib/stores/filaments';
  import type { FilamentProfile } from '$lib/stores/filaments';

  interface FilamentManufacturer {
    id: number;
    name: string;
    website: string | null;
  }

  interface FilamentMaterial {
    id: number;
    name: string;
  }

  interface FilamentColorSwatch {
    id: number;
    color_name: string;
    hex_color: string;
    manufacturer: FilamentManufacturer;
    material: FilamentMaterial;
    image_front: string | null;
    image_back: string | null;
    image_other: string | null;
    notes: string | null;
    amazon_purchase_link: string | null;
    manufacturer_purchase_link: string | null;
    published: boolean;
  }

  interface FilamentColorsResponse {
    count: number;
    next: string | null;
    previous: string | null;
    results: FilamentColorSwatch[];
  }

  let swatches: FilamentColorSwatch[] = [];
  let totalCount = 0;
  let loading = false;
  let currentOffset = 0;
  const limit = 50;

  let selectedManufacturer = '';
  let selectedMaterial = '';
  let manufacturers: string[] = [];
  let materials: string[] = [];

  let selectedSwatch: FilamentColorSwatch | null = null;
  let showModal = false;
  let scrollContainer: HTMLDivElement;

  onMount(async () => {
    await loadSwatches();
    extractFilters();
  });

  function extractFilters() {
    const mfrSet = new Set<string>();
    const matSet = new Set<string>();
    
    swatches.forEach(s => {
      mfrSet.add(s.manufacturer.name);
      matSet.add(s.material.name);
    });
    
    manufacturers = Array.from(mfrSet).sort();
    materials = Array.from(matSet).sort();
  }

  async function loadSwatches(reset = false) {
    if (loading) return;
    
    loading = true;
    try {
      if (reset) {
        currentOffset = 0;
        swatches = [];
      }

      const response = await invoke<FilamentColorsResponse>('get_filament_swatches', {
        manufacturer: selectedManufacturer || null,
        materialType: selectedMaterial || null,
        limit,
        offset: currentOffset,
      });

      if (reset) {
        swatches = response.results;
      } else {
        swatches = [...swatches, ...response.results];
      }
      
      totalCount = response.count;
      currentOffset += response.results.length;
      
      if (reset) extractFilters();
    } catch (error) {
      console.error('Failed to load swatches:', error);
      alert('Failed to load filament swatches: ' + error);
    } finally {
      loading = false;
    }
  }

  async function handleFilter() {
    await loadSwatches(true);
  }

  function handleScroll(e: Event) {
    const target = e.target as HTMLDivElement;
    const scrollPercentage = (target.scrollTop + target.clientHeight) / target.scrollHeight;
    
    if (scrollPercentage > 0.8 && swatches.length < totalCount) {
      loadSwatches();
    }
  }

  function openModal(swatch: FilamentColorSwatch) {
    selectedSwatch = swatch;
    showModal = true;
  }

  function closeModal() {
    showModal = false;
    selectedSwatch = null;
  }

  async function handleAddToFavorites(swatch: FilamentColorSwatch) {
    try {
      const profile: FilamentProfile = {
        brand: swatch.manufacturer.name,
        material: swatch.material.name,
        color: swatch.hex_color.startsWith('#') ? swatch.hex_color : '#' + swatch.hex_color,
        nozzle_temp: 220,
        bed_temp: 60,
        density: 1.24,
        diameter: 1.75,
        is_favorite: true,
        is_custom: false,
      };
      
      await addFavorite(profile);
      alert(`Added ${swatch.color_name} to favorites!`);
    } catch (error) {
      alert('Failed to add to favorites: ' + error);
    }
  }
</script>

<div class="flex flex-col h-full">
  <Header title="Filament Colors" subtitle="Visual library powered by FilamentColors.xyz"></Header>

  <div class="p-8 space-y-6 flex-1 overflow-auto" bind:this={scrollContainer} onscroll={handleScroll}>
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <select
          bind:value={selectedManufacturer}
          onchange={handleFilter}
          class="px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
        >
          <option value="">All Manufacturers ({manufacturers.length})</option>
          {#each manufacturers as mfr}
            <option value={mfr}>{mfr}</option>
          {/each}
        </select>

        <select
          bind:value={selectedMaterial}
          onchange={handleFilter}
          class="px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
        >
          <option value="">All Materials ({materials.length})</option>
          {#each materials as mat}
            <option value={mat}>{mat}</option>
          {/each}
        </select>

        <div class="px-4 py-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg text-center">
          <p class="text-sm font-medium text-blue-700 dark:text-blue-300">
            {totalCount} total swatches
          </p>
        </div>
      </div>
    </div>

    {#if loading && swatches.length === 0}
      <div class="text-center py-16">
        <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
        <p class="text-gray-600 dark:text-gray-400 mt-4">Loading swatches from FilamentColors.xyz...</p>
      </div>
    {:else if swatches.length === 0}
      <div class="text-center py-16">
        <p class="text-2xl mb-4">📸</p>
        <p class="text-gray-600 dark:text-gray-400 text-lg">
          No swatches found
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6">
        {#each swatches as swatch (swatch.id)}
          <button
            onclick={() => openModal(swatch)}
            class="group bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-xl transition-all overflow-hidden"
          >
            {#if swatch.image_front}
              <div class="w-full aspect-square bg-gray-100 dark:bg-gray-700 overflow-hidden">
                <img
                  src={swatch.image_front}
                  alt={swatch.color_name}
                  class="w-full h-full object-cover group-hover:scale-110 transition-transform"
                  loading="lazy"
                />
              </div>
            {:else}
              <div
                class="w-full aspect-square"
                style="background-color: {swatch.hex_color.startsWith('#') ? swatch.hex_color : '#' + swatch.hex_color}"
              ></div>
            {/if}
            
            <div class="p-4">
              <h4 class="font-semibold text-gray-900 dark:text-white text-sm mb-1">
                {swatch.color_name}
              </h4>
              <p class="text-xs text-gray-600 dark:text-gray-400">
                {swatch.manufacturer.name}
              </p>
              <p class="text-xs text-gray-500 dark:text-gray-500 mt-1">
                {swatch.material.name}
              </p>
              <p class="text-xs text-gray-400 dark:text-gray-600 mt-1 font-mono">
                {swatch.hex_color.startsWith('#') ? swatch.hex_color : '#' + swatch.hex_color}
              </p>
            </div>
          </button>
        {/each}
      </div>

      <div class="text-center mt-8 py-4">
        <p class="text-gray-500 dark:text-gray-400">
          Showing {swatches.length} of {totalCount} swatches
        </p>
        {#if loading && swatches.length > 0}
          <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary mt-4"></div>
        {/if}
      </div>
    {/if}
  </div>
</div>

{#if showModal && selectedSwatch}
  <div
    class="fixed inset-0 bg-black/70 flex items-center justify-center z-50 p-4"
    onclick={closeModal}
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl max-w-4xl w-full p-8 max-h-[90vh] overflow-y-auto"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
        <div>
          {#if selectedSwatch.image_front}
            <img
              src={selectedSwatch.image_front}
              alt={selectedSwatch.color_name}
              class="w-full rounded-lg shadow-lg"
            />
          {:else}
            <div
              class="w-full aspect-square rounded-lg"
              style="background-color: {selectedSwatch.hex_color.startsWith('#') ? selectedSwatch.hex_color : '#' + selectedSwatch.hex_color}"
            ></div>
          {/if}
          
          <div class="grid grid-cols-2 gap-4 mt-4">
            {#if selectedSwatch.image_back}
              <img src={selectedSwatch.image_back} alt="Back" class="w-full rounded-lg shadow-md" />
            {/if}
            {#if selectedSwatch.image_other}
              <img src={selectedSwatch.image_other} alt="Other" class="w-full rounded-lg shadow-md" />
            {/if}
          </div>
        </div>
        
        <div>
          <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
            {selectedSwatch.color_name}
          </h2>
          <p class="text-xl text-gray-600 dark:text-gray-400 mb-4">
            {selectedSwatch.manufacturer.name} · {selectedSwatch.material.name}
          </p>
          
          <div class="bg-gray-50 dark:bg-gray-700/50 p-4 rounded-lg mb-6">
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-2">Hex Color</p>
            <p class="text-2xl font-mono font-bold text-gray-900 dark:text-white">
              {selectedSwatch.hex_color.startsWith('#') ? selectedSwatch.hex_color : '#' + selectedSwatch.hex_color}
            </p>
          </div>
          
          {#if selectedSwatch.notes}
            <div class="mb-6">
              <p class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">Notes</p>
              <p class="text-sm text-gray-600 dark:text-gray-400">
                {selectedSwatch.notes}
              </p>
            </div>
          {/if}
          
          <div class="flex flex-wrap gap-3 mb-6">
            {#if selectedSwatch.manufacturer.website}
              <a
                href={selectedSwatch.manufacturer.website}
                target="_blank"
                rel="noopener noreferrer"
                class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors text-sm font-medium"
              >
                🌐 Manufacturer Site
              </a>
            {/if}
            {#if selectedSwatch.manufacturer_purchase_link}
              <a
                href={selectedSwatch.manufacturer_purchase_link}
                target="_blank"
                rel="noopener noreferrer"
                class="px-4 py-2 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 rounded-lg hover:bg-blue-200 dark:hover:bg-blue-900/50 transition-colors text-sm font-medium"
              >
                🛒 Buy Direct
              </a>
            {/if}
            {#if selectedSwatch.amazon_purchase_link}
              <a
                href={selectedSwatch.amazon_purchase_link}
                target="_blank"
                rel="noopener noreferrer"
                class="px-4 py-2 bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-300 rounded-lg hover:bg-orange-200 dark:hover:bg-orange-900/50 transition-colors text-sm font-medium"
              >
                📦 Amazon
              </a>
            {/if}
          </div>
          
          <div class="flex gap-3 pt-6 border-t border-gray-200 dark:border-gray-700">
            <button
              onclick={closeModal}
              class="flex-1 px-6 py-3 border-2 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors font-semibold"
            >
              Close
            </button>
            <button
              onclick={() => handleAddToFavorites(selectedSwatch!)}
              class="flex-1 px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold"
            >
              ⭐ Add to Favorites
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}
