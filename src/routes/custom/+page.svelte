<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import Header from '$lib/components/Header.svelte';
  import { customProfiles, loadCustomProfiles, createCustomProfile, deleteCustomProfile, type FilamentProfile } from '$lib/stores/filaments';
  import { invoke } from '@tauri-apps/api/core';

  let formData = {
    brand: '',
    material: '',
    color: '#FF5733',
    nozzle_temp: 220,
    bed_temp: 60,
    density: 1.24,
    diameter: 1.75,
  };

  let brands: string[] = [];
  let materials: string[] = [];
  let customBrand = '';
  let customMaterial = '';
  let showCustomBrand = false;
  let showCustomMaterial = false;

  onMount(async () => {
    console.log('Custom page mounted, loading data...');
    await loadCustomProfiles();
    await loadBrandsAndMaterials();
  });

  async function loadBrandsAndMaterials() {
    try {
      brands = await invoke<string[]>('get_brands');
      materials = await invoke<string[]>('get_materials');
      console.log('Loaded brands:', brands);
      console.log('Loaded materials:', materials);
      
      if (brands.length > 0 && !formData.brand) {
        formData.brand = brands[0];
      }
      if (materials.length > 0 && !formData.material) {
        formData.material = materials[0];
      }
    } catch (error) {
      console.error('Failed to load brands/materials:', error);
    }
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    try {
      const profile: FilamentProfile = {
        brand: showCustomBrand && customBrand ? customBrand : formData.brand,
        material: showCustomMaterial && customMaterial ? customMaterial : formData.material,
        color: formData.color,
        nozzle_temp: formData.nozzle_temp,
        bed_temp: formData.bed_temp,
        density: formData.density,
        diameter: formData.diameter,
        is_favorite: true,
        is_custom: true,
      };
      
      console.log('Creating profile:', profile);
      await createCustomProfile(profile);
      
      await loadBrandsAndMaterials();
      
      formData = {
        brand: brands[0] || '',
        material: materials[0] || '',
        color: '#FF5733',
        nozzle_temp: 220,
        bed_temp: 60,
        density: 1.24,
        diameter: 1.75,
      };
      customBrand = '';
      customMaterial = '';
      showCustomBrand = false;
      showCustomMaterial = false;
    } catch (error) {
      console.error('Failed to create profile:', error);
      alert('Failed to create profile: ' + error);
    }
  }

  async function handleDelete(id: number) {
    if (confirm('Delete this custom profile?')) {
      await deleteCustomProfile(id);
    }
  }

  function handleCancel() {
    goto('/');
  }
</script>

<div class="flex flex-col h-full">
  <Header title="Custom Profiles" subtitle="Create and manage your own filament profiles"></Header>

  <div class="p-8 max-w-3xl">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8">
      <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
        Create New Profile
      </h3>

      <form onsubmit={handleSubmit} class="space-y-6">
        <div class="grid grid-cols-2 gap-6">
          <div>
            <label for="brand" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Brand
            </label>
            {#if showCustomBrand}
              <input
                id="brand"
                type="text"
                bind:value={customBrand}
                placeholder="Enter custom brand"
                required
                class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
              />
              <button
                type="button"
                onclick={() => showCustomBrand = false}
                class="text-xs text-primary hover:underline mt-1"
              >
                ← Choose from list
              </button>
            {:else}
              <select
                id="brand"
                bind:value={formData.brand}
                required
                class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
              >
                {#each brands as brand}
                  <option value={brand}>{brand}</option>
                {/each}
              </select>
              <button
                type="button"
                onclick={() => showCustomBrand = true}
                class="text-xs text-primary hover:underline mt-1"
              >
                + Add custom brand
              </button>
            {/if}
          </div>

          <div>
            <label for="material" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Material Type
            </label>
            {#if showCustomMaterial}
              <input
                id="material"
                type="text"
                bind:value={customMaterial}
                placeholder="Enter custom material"
                required
                class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
              />
              <button
                type="button"
                onclick={() => showCustomMaterial = false}
                class="text-xs text-primary hover:underline mt-1"
              >
                ← Choose from list
              </button>
            {:else}
              <select
                id="material"
                bind:value={formData.material}
                required
                class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
              >
                {#each materials as material}
                  <option value={material}>{material}</option>
                {/each}
              </select>
              <button
                type="button"
                onclick={() => showCustomMaterial = true}
                class="text-xs text-primary hover:underline mt-1"
              >
                + Add custom material
              </button>
            {/if}
          </div>
        </div>

        <div>
          <label for="color" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Color (Hex)
          </label>
          <div class="flex gap-3">
            <input
              type="color"
              bind:value={formData.color}
              class="h-12 w-24 border border-gray-300 dark:border-gray-600 rounded-lg cursor-pointer"
            />
            <input
              id="color"
              type="text"
              bind:value={formData.color}
              placeholder="#FF5733"
              required
              class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>
        </div>

        <div class="grid grid-cols-2 gap-6">
          <div>
            <label for="nozzle" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Nozzle Temp (°C)
            </label>
            <input
              id="nozzle"
              type="number"
              bind:value={formData.nozzle_temp}
              min="150"
              max="300"
              required
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>

          <div>
            <label for="bed" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Bed Temp (°C)
            </label>
            <input
              id="bed"
              type="number"
              bind:value={formData.bed_temp}
              min="0"
              max="120"
              required
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>
        </div>

        <div class="grid grid-cols-2 gap-6">
          <div>
            <label for="density" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Density (g/cm³)
            </label>
            <input
              id="density"
              type="number"
              bind:value={formData.density}
              step="0.01"
              min="0.5"
              max="3.0"
              required
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>

          <div>
            <label for="diameter" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Diameter (mm)
            </label>
            <select
              id="diameter"
              bind:value={formData.diameter}
              required
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            >
              <option value={1.75}>1.75mm</option>
              <option value={2.85}>2.85mm</option>
            </select>
          </div>
        </div>

        <div class="flex gap-3">
          <button
            type="button"
            onclick={handleCancel}
            class="flex-1 px-6 py-3 border-2 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors font-semibold text-lg"
          >
            Cancel
          </button>
          <button
            type="submit"
            class="flex-1 px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold text-lg"
          >
            Create Profile
          </button>
        </div>
      </form>

      <div class="mt-8 pt-8 border-t border-gray-200 dark:border-gray-700">
        <h4 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Your Custom Profiles ({$customProfiles.length})
        </h4>
        
        {#if $customProfiles.length === 0}
          <p class="text-gray-500 dark:text-gray-400 text-center py-8">
            No custom profiles yet. Create your first one above!
          </p>
        {:else}
          <div class="space-y-3">
            {#each $customProfiles as profile (profile.id)}
              <div class="flex items-center gap-4 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                <div
                  class="w-12 h-12 rounded-lg flex-shrink-0 border-2 border-gray-300 dark:border-gray-600"
                  style="background-color: {profile.color}"
                ></div>
                <div class="flex-1">
                  <p class="font-semibold text-gray-900 dark:text-white">{profile.brand}</p>
                  <p class="text-sm text-gray-600 dark:text-gray-400">{profile.material} · {profile.nozzle_temp}°C</p>
                </div>
                <button
                  onclick={() => handleDelete(profile.id!)}
                  class="px-4 py-2 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors"
                >
                  Delete
                </button>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
