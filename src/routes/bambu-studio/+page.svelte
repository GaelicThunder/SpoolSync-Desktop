<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import Header from '$lib/components/Header.svelte';
  import { spoolmanFilaments, spoolmanBrands, spoolmanMaterials, searchSpoolman, loadBrands, loadMaterials, type SpoolmanFilament } from '$lib/stores/spoolman';

  let bambuProfiles: string[] = [];
  let loading = false;
  let error = '';
  let success = '';
  
  let showSpoolmanModal = false;
  let searchQuery = '';
  let selectedBrand = '';
  let selectedMaterial = '';
  let selectedFilaments: Set<number> = new Set();
  let importProgress = { current: 0, total: 0 };

  onMount(async () => {
    await loadProfiles();
    await loadBrands();
    await loadMaterials();
    await searchSpoolman(undefined, undefined, undefined, 100, 0);
  });

  async function loadProfiles() {
    loading = true;
    error = '';
    try {
      bambuProfiles = await invoke<string[]>('list_bambu_profiles');
    } catch (e: any) {
      error = e.toString();
    } finally {
      loading = false;
    }
  }

  async function deleteProfile(name: string) {
    if (!confirm(`Delete profile "${name}"?`)) return;
    
    loading = true;
    error = '';
    success = '';
    try {
      await invoke('delete_bambu_profile', { name });
      success = `Deleted ${name}`;
      await loadProfiles();
      setTimeout(() => success = '', 3000);
    } catch (e: any) {
      error = e.toString();
    } finally {
      loading = false;
    }
  }

  function openSpoolmanModal() {
    showSpoolmanModal = true;
  }

  async function handleSearch() {
    await searchSpoolman(
      searchQuery || undefined,
      selectedBrand || undefined,
      selectedMaterial || undefined,
      100,
      0
    );
  }

  function toggleSelection(filamentId: number) {
    if (selectedFilaments.has(filamentId)) {
      selectedFilaments.delete(filamentId);
    } else {
      selectedFilaments.add(filamentId);
    }
    selectedFilaments = selectedFilaments;
  }

  function clearSelection() {
    selectedFilaments.clear();
    selectedFilaments = selectedFilaments;
  }

  async function importSelected() {
    if (selectedFilaments.size === 0) {
      error = 'No filaments selected';
      return;
    }

    const filamentsToImport = $spoolmanFilaments.filter(f => selectedFilaments.has(f.id));
    
    loading = true;
    error = '';
    success = '';
    importProgress = { current: 0, total: filamentsToImport.length };

    let imported = 0;
    let failed = 0;
    
    for (const filament of filamentsToImport) {
      try {
        const vendor = filament.manufacturer || 'Generic';
        const name = filament.name || 'Custom';
        const material = filament.material || 'PLA';
        const colorHex = filament.color_hex ? '#' + filament.color_hex.replace('#', '') : '#FFFFFF';
        const nozzleTemp = filament.extruder_temp || 220;
        const bedTemp = filament.bed_temp || 60;
        const density = filament.density || 1.24;

        await invoke('sync_spoolman_to_bambu_studio', {
          vendor,
          name,
          material,
          colorHex,
          nozzleTemp,
          bedTemp,
          density
        });

        imported++;
        importProgress.current++;
      } catch (e: any) {
        console.error(`Failed to import ${filament.manufacturer} ${filament.material}:`, e);
        failed++;
        importProgress.current++;
      }
    }

    loading = false;
    
    if (failed === 0) {
      success = `✅ Successfully imported ${imported} profile(s)!`;
    } else {
      success = `✅ Imported ${imported} profile(s), ${failed} failed`;
    }
    
    clearSelection();
    await loadProfiles();
    setTimeout(() => {
      success = '';
      importProgress = { current: 0, total: 0 };
    }, 5000);
  }
</script>

<div class="flex flex-col h-full">
  <Header title="Bambu Studio Profiles" subtitle="Import custom filaments to your slicer"></Header>

  <div class="p-8 space-y-6 flex-1 overflow-auto">
    {#if error}
      <div class="bg-red-100 dark:bg-red-900/30 border border-red-400 dark:border-red-700 text-red-700 dark:text-red-400 px-6 py-4 rounded-lg">
        <p class="font-semibold">⚠️ Error</p>
        <p class="text-sm mt-1">{error}</p>
      </div>
    {/if}

    {#if success}
      <div class="bg-green-100 dark:bg-green-900/30 border border-green-400 dark:border-green-700 text-green-700 dark:text-green-400 px-6 py-4 rounded-lg">
        <p class="font-semibold">{success}</p>
        {#if importProgress.total > 0}
          <div class="mt-2">
            <div class="bg-green-200 dark:bg-green-800 h-2 rounded-full overflow-hidden">
              <div 
                class="bg-green-600 dark:bg-green-400 h-full transition-all duration-300"
                style="width: {(importProgress.current / importProgress.total) * 100}%"
              ></div>
            </div>
            <p class="text-xs mt-1">{importProgress.current} / {importProgress.total}</p>
          </div>
        {/if}
      </div>
    {/if}

    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white">Custom Profiles</h2>
        <p class="text-gray-600 dark:text-gray-400 mt-1">
          {bambuProfiles.length} profile{bambuProfiles.length !== 1 ? 's' : ''} in Bambu Studio
        </p>
      </div>
      <button
        onclick={openSpoolmanModal}
        disabled={loading}
        class="px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
      >
        📥 Import from Spoolman
      </button>
    </div>

    {#if loading && bambuProfiles.length === 0}
      <div class="text-center py-16">
        <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
        <p class="text-gray-600 dark:text-gray-400 mt-4">Loading profiles...</p>
      </div>
    {:else if bambuProfiles.length === 0}
      <div class="text-center py-16 bg-gray-50 dark:bg-gray-800 rounded-lg">
        <p class="text-4xl mb-4">🎨</p>
        <p class="text-gray-600 dark:text-gray-400 text-lg font-semibold">
          No custom profiles yet
        </p>
        <p class="text-gray-500 dark:text-gray-500 mt-2">
          Import profiles from Spoolman to get started
        </p>
        <button
          onclick={openSpoolmanModal}
          class="mt-6 px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold"
        >
          📥 Import from Spoolman
        </button>
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each bambuProfiles as profile}
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
            <div class="flex items-start justify-between">
              <div class="flex-1">
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white truncate">
                  {profile}
                </h3>
                <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
                  Custom profile
                </p>
              </div>
              <button
                onclick={() => deleteProfile(profile)}
                disabled={loading}
                class="text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300 p-2 disabled:opacity-50"
                title="Delete profile"
              >
                🗑️
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}

    <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-6 mt-8">
      <h3 class="text-lg font-semibold text-blue-900 dark:text-blue-100 mb-2">ℹ️ How it works</h3>
      <ul class="text-sm text-blue-800 dark:text-blue-200 space-y-2">
        <li>• Profiles are saved in <code class="bg-blue-100 dark:bg-blue-950 px-2 py-1 rounded">~/.config/BambuStudio/user/[user_id]/filament</code></li>
        <li>• Restart Bambu Studio to see the new profiles</li>
        <li>• Profiles inherit base settings from the material type (PLA, PETG, etc.)</li>
        <li>• You can further customize them in Bambu Studio's filament settings</li>
      </ul>
    </div>
  </div>
</div>

{#if showSpoolmanModal}
  <div
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
    onclick={() => showSpoolmanModal = false}
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl max-w-6xl w-full max-h-[90vh] overflow-hidden flex flex-col"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="p-6 border-b border-gray-200 dark:border-gray-700">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-2xl font-bold text-gray-900 dark:text-white">Import from Spoolman</h2>
            <p class="text-gray-600 dark:text-gray-400 mt-1">
              {selectedFilaments.size > 0 ? `${selectedFilaments.size} selected` : 'Select filaments to import'}
            </p>
          </div>
          {#if selectedFilaments.size > 0}
            <div class="flex gap-2">
              <button
                onclick={clearSelection}
                class="px-4 py-2 border-2 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors font-semibold"
              >
                Clear
              </button>
              <button
                onclick={importSelected}
                disabled={loading}
                class="px-6 py-2 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold disabled:opacity-50"
              >
                📥 Import {selectedFilaments.size}
              </button>
            </div>
          {/if}
        </div>
      </div>

      <div class="p-6 border-b border-gray-200 dark:border-gray-700">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <input
            type="text"
            bind:value={searchQuery}
            onkeydown={(e) => e.key === 'Enter' && handleSearch()}
            placeholder="Search..."
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
          />

          <select
            bind:value={selectedBrand}
            onchange={handleSearch}
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
          >
            <option value="">All Brands</option>
            {#each $spoolmanBrands as brand}
              <option value={brand}>{brand}</option>
            {/each}
          </select>

          <select
            bind:value={selectedMaterial}
            onchange={handleSearch}
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
          >
            <option value="">All Materials</option>
            {#each $spoolmanMaterials as material}
              <option value={material}>{material}</option>
            {/each}
          </select>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto p-6">
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {#each $spoolmanFilaments as filament (filament.id)}
            {@const isSelected = selectedFilaments.has(filament.id)}
            <button
              onclick={() => toggleSelection(filament.id)}
              class="bg-white dark:bg-gray-700 rounded-lg shadow hover:shadow-lg transition-all p-4 text-left {isSelected ? 'ring-2 ring-primary' : ''}"
            >
              <div class="flex items-start gap-3">
                <input
                  type="checkbox"
                  checked={isSelected}
                  onchange={() => toggleSelection(filament.id)}
                  onclick={(e) => e.stopPropagation()}
                  class="mt-1 w-5 h-5 text-primary rounded focus:ring-primary"
                />
                
                <div
                  class="w-12 h-12 rounded flex-shrink-0 border-2 border-gray-200 dark:border-gray-600"
                  style="background-color: {filament.color_hex ? '#' + filament.color_hex.replace('#', '') : '#888888'}"
                ></div>
                
                <div class="flex-1 min-w-0">
                  <h3 class="font-semibold text-gray-900 dark:text-white truncate">
                    {filament.manufacturer || 'Unknown'}
                  </h3>
                  <p class="text-sm text-gray-600 dark:text-gray-400 truncate">
                    {filament.material || 'Unknown'}
                  </p>
                  {#if filament.name}
                    <p class="text-xs text-gray-500 dark:text-gray-500 truncate mt-1">
                      {filament.name}
                    </p>
                  {/if}
                  <div class="flex gap-2 mt-2 text-xs text-gray-500 dark:text-gray-400">
                    {#if filament.extruder_temp}
                      <span>🌡️ {filament.extruder_temp}°C</span>
                    {/if}
                  </div>
                </div>
              </div>
            </button>
          {/each}
        </div>
      </div>

      <div class="p-6 border-t border-gray-200 dark:border-gray-700">
        <div class="flex gap-3">
          <button
            onclick={() => showSpoolmanModal = false}
            class="flex-1 px-6 py-3 border-2 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors font-semibold"
          >
            Close
          </button>
          {#if selectedFilaments.size > 0}
            <button
              onclick={importSelected}
              disabled={loading}
              class="flex-1 px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold disabled:opacity-50"
            >
              📥 Import {selectedFilaments.size} Profile{selectedFilaments.size !== 1 ? 's' : ''}
            </button>
          {/if}
        </div>
      </div>
    </div>
  </div>
{/if}
