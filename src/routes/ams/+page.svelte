<script lang="ts">
  import Header from '$lib/components/Header.svelte';

  const mockAMS = [
    {
      id: 0,
      name: 'AMS 1',
      trays: [
        { id: 0, brand: 'Bambu Lab', material: 'PLA', color: '#FF5733', temp: 220, loaded: true },
        { id: 1, brand: 'Polymaker', material: 'PETG', color: '#3366FF', temp: 240, loaded: true },
        { id: 2, brand: null, material: null, color: null, temp: null, loaded: false },
        { id: 3, brand: 'eSUN', material: 'ABS', color: '#FFD700', temp: 250, loaded: true },
      ],
    },
    {
      id: 1,
      name: 'AMS 2',
      trays: [
        { id: 0, brand: null, material: null, color: null, temp: null, loaded: false },
        { id: 1, brand: null, material: null, color: null, temp: null, loaded: false },
        { id: 2, brand: null, material: null, color: null, temp: null, loaded: false },
        { id: 3, brand: null, material: null, color: null, temp: null, loaded: false },
      ],
    },
  ];
</script>

<div class="flex flex-col h-full">
  <Header title="AMS Status" subtitle="Current filament loaded in your printer"></Header>

  <div class="p-8 space-y-8">
    <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
      <p class="text-yellow-800 dark:text-yellow-200 text-sm">
        🚧 <strong>Coming Soon:</strong> Real-time AMS status via MQTT. Currently showing mock data.
      </p>
    </div>

    {#each mockAMS as ams}
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
        <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-6">
          {ams.name}
        </h3>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {#each ams.trays as tray}
            <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-4 border-2 {tray.loaded ? 'border-primary' : 'border-gray-300 dark:border-gray-600'}">
              <div class="flex items-center justify-between mb-3">
                <span class="text-sm font-semibold text-gray-600 dark:text-gray-400">
                  Tray {tray.id + 1}
                </span>
                {#if tray.loaded}
                  <span class="text-xs bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300 px-2 py-1 rounded">
                    Loaded
                  </span>
                {:else}
                  <span class="text-xs bg-gray-200 dark:bg-gray-600 text-gray-600 dark:text-gray-400 px-2 py-1 rounded">
                    Empty
                  </span>
                {/if}
              </div>

              {#if tray.loaded}
                <div class="flex items-center gap-3 mb-3">
                  <div
                    class="w-12 h-12 rounded-lg border-2 border-gray-300 dark:border-gray-600 flex-shrink-0"
                    style="background-color: {tray.color}"
                  ></div>
                  <div class="flex-1 min-w-0">
                    <p class="font-semibold text-gray-900 dark:text-white truncate">
                      {tray.brand}
                    </p>
                    <p class="text-sm text-gray-600 dark:text-gray-400">
                      {tray.material}
                    </p>
                  </div>
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  🌡️ {tray.temp}°C
                </div>
              {:else}
                <div class="h-20 flex items-center justify-center text-gray-400 dark:text-gray-500">
                  <span class="text-3xl">➕</span>
                </div>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    {/each}

    <div class="text-center text-gray-500 dark:text-gray-400 text-sm">
      <p>Connect your printer in Settings to see real-time AMS status</p>
    </div>
  </div>
</div>
