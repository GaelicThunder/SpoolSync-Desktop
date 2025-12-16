<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import Header from '$lib/components/Header.svelte';
  import { getFavorites, getCustomProfiles, type FilamentProfile } from '$lib/stores/filaments';
  import { getSettings, saveSettings, type Settings } from '$lib/stores/settings';

  interface AMSSlot {
    slot_id: number;
    filament?: FilamentProfile;
  }

  let settings: Settings | null = null;
  let slots: AMSSlot[] = Array.from({ length: 4 }, (_, i) => ({ slot_id: i }));
  let favorites: FilamentProfile[] = [];
  let customProfiles: FilamentProfile[] = [];
  let allProfiles: FilamentProfile[] = [];

  let testingConnection = false;
  let syncingSlot: number | null = null;
  let connectionStatus = '';

  onMount(async () => {
    settings = await getSettings();
    favorites = await getFavorites();
    customProfiles = await getCustomProfiles();
    allProfiles = [...favorites, ...customProfiles];
  });

  async function testConnection() {
    if (!settings?.printer_ip || !settings?.printer_access_code || !settings?.printer_serial) {
      connectionStatus = '❌ Configure printer in Settings first';
      return;
    }

    testingConnection = true;
    connectionStatus = '🔌 Testing connection...';

    try {
      const result = await invoke<string>('test_printer_connection', {
        config: {
          name: settings.printer_name || 'Bambu Printer',
          ip_address: settings.printer_ip,
          access_code: settings.printer_access_code,
          serial_number: settings.printer_serial,
        },
      });
      connectionStatus = '✅ ' + result;
    } catch (error) {
      connectionStatus = '❌ ' + error;
    } finally {
      testingConnection = false;
    }
  }

  async function syncSlot(slotId: number) {
    const slot = slots[slotId];
    if (!slot.filament) {
      alert('Select a filament for this slot first');
      return;
    }

    if (!settings?.printer_ip || !settings?.printer_access_code || !settings?.printer_serial) {
      alert('Configure printer in Settings first');
      return;
    }

    syncingSlot = slotId;

    try {
      const result = await invoke<string>('sync_to_ams', {
        config: {
          name: settings.printer_name || 'Bambu Printer',
          ip_address: settings.printer_ip,
          access_code: settings.printer_access_code,
          serial_number: settings.printer_serial,
        },
        command: {
          slot_id: slotId,
          brand: slot.filament.brand,
          material: slot.filament.material,
          color: slot.filament.color,
          nozzle_temp: slot.filament.nozzle_temp,
          bed_temp: slot.filament.bed_temp,
        },
      });
      alert('✅ ' + result);
    } catch (error) {
      alert('❌ Failed to sync: ' + error);
    } finally {
      syncingSlot = null;
    }
  }

  function assignFilament(slotId: number, profile: FilamentProfile) {
    slots[slotId].filament = profile;
    slots = [...slots];
  }

  function clearSlot(slotId: number) {
    slots[slotId].filament = undefined;
    slots = [...slots];
  }
</script>

<div class="flex flex-col h-full">
  <Header title="AMS Sync" subtitle="Sync filament profiles to Bambu Lab AMS"></Header>

  <div class="p-8 space-y-6 flex-1 overflow-auto">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
      <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4">Printer Connection</h2>
      
      {#if !settings?.printer_ip}
        <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
          <p class="text-yellow-800 dark:text-yellow-200">
            ⚠️ Please configure your printer in <a href="/settings" class="underline font-semibold">Settings</a> first.
          </p>
        </div>
      {:else}
        <div class="space-y-3">
          <div class="grid grid-cols-2 gap-4 text-sm">
            <div>
              <span class="text-gray-500 dark:text-gray-400">Printer:</span>
              <span class="ml-2 font-medium text-gray-900 dark:text-white">{settings.printer_name || 'N/A'}</span>
            </div>
            <div>
              <span class="text-gray-500 dark:text-gray-400">IP:</span>
              <span class="ml-2 font-medium text-gray-900 dark:text-white">{settings.printer_ip}</span>
            </div>
            <div>
              <span class="text-gray-500 dark:text-gray-400">Serial:</span>
              <span class="ml-2 font-mono text-gray-900 dark:text-white">{settings.printer_serial}</span>
            </div>
          </div>
          
          <button
            onclick={testConnection}
            disabled={testingConnection}
            class="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {testingConnection ? '🔌 Testing...' : '🔌 Test Connection'}
          </button>
          
          {#if connectionStatus}
            <p class="text-sm {connectionStatus.startsWith('✅') ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'}">
              {connectionStatus}
            </p>
          {/if}
        </div>
      {/if}
    </div>

    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
      <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4">AMS Slots</h2>
      
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        {#each slots as slot (slot.slot_id)}
          <div class="border-2 border-gray-200 dark:border-gray-700 rounded-lg p-4">
            <div class="flex items-center justify-between mb-3">
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                Slot {slot.slot_id + 1}
              </h3>
              {#if slot.filament}
                <button
                  onclick={() => clearSlot(slot.slot_id)}
                  class="text-red-600 dark:text-red-400 hover:text-red-700 dark:hover:text-red-300 text-sm"
                >
                  ❌ Clear
                </button>
              {/if}
            </div>

            {#if slot.filament}
              <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-4 mb-3">
                <div class="flex items-center gap-3 mb-2">
                  <div
                    class="w-12 h-12 rounded-lg border-2 border-gray-300 dark:border-gray-600"
                    style="background-color: {slot.filament.color}"
                  ></div>
                  <div>
                    <p class="font-semibold text-gray-900 dark:text-white">
                      {slot.filament.brand}
                    </p>
                    <p class="text-sm text-gray-600 dark:text-gray-400">
                      {slot.filament.material}
                    </p>
                  </div>
                </div>
                <div class="grid grid-cols-2 gap-2 text-xs text-gray-600 dark:text-gray-400">
                  <div>🌡️ Nozzle: {slot.filament.nozzle_temp}°C</div>
                  <div>🔥 Bed: {slot.filament.bed_temp}°C</div>
                  <div>📏 Density: {slot.filament.density}</div>
                  <div>📍 Diameter: {slot.filament.diameter}mm</div>
                </div>
              </div>

              <button
                onclick={() => syncSlot(slot.slot_id)}
                disabled={syncingSlot === slot.slot_id}
                class="w-full px-4 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {syncingSlot === slot.slot_id ? '🔄 Syncing...' : '🔄 Sync to AMS'}
              </button>
            {:else}
              <select
                onchange={(e) => {
                  const index = parseInt(e.currentTarget.value);
                  if (!isNaN(index)) assignFilament(slot.slot_id, allProfiles[index]);
                }}
                class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
              >
                <option value="">Select filament...</option>
                {#if favorites.length > 0}
                  <optgroup label="⭐ Favorites">
                    {#each favorites as profile, i}
                      <option value={i}>
                        {profile.brand} {profile.material} - {profile.color}
                      </option>
                    {/each}
                  </optgroup>
                {/if}
                {#if customProfiles.length > 0}
                  <optgroup label="⚙️ Custom">
                    {#each customProfiles as profile, i}
                      <option value={favorites.length + i}>
                        {profile.brand} {profile.material} - {profile.color}
                      </option>
                    {/each}
                  </optgroup>
                {/if}
              </select>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>
