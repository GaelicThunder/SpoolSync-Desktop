<script lang="ts">
  import Header from '$lib/components/Header.svelte';
  
  let searchQuery = '';
  
  const mockFilaments = [
    { id: 1, brand: 'Bambu Lab', material: 'PLA Basic', color: '#FF5733', temp: 220 },
    { id: 2, brand: 'Polymaker', material: 'PolyTerra PLA', color: '#33FF57', temp: 210 },
    { id: 3, brand: 'Prusament', material: 'PETG', color: '#3357FF', temp: 245 },
  ];
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

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each mockFilaments as filament}
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-lg transition-shadow p-6">
          <div class="flex items-start gap-4">
            <div
              class="w-16 h-16 rounded-lg flex-shrink-0"
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
                {filament.temp}°C
              </p>
            </div>
          </div>

          <div class="flex gap-2 mt-4">
            <button class="flex-1 px-4 py-2 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors text-sm font-medium">
              Sync to AMS
            </button>
            <button class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors text-sm">
              ⭐
            </button>
          </div>
        </div>
      {/each}
    </div>

    <div class="text-center text-gray-500 dark:text-gray-400 mt-8">
      <p>Showing {mockFilaments.length} filaments (mock data)</p>
      <p class="text-sm mt-2">SpoolmanDB integration coming in Phase 4</p>
    </div>
  </div>
</div>
