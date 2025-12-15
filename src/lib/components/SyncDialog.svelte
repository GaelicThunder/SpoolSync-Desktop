<script lang="ts">
  import { settings } from '$lib/stores/settings';
  import { syncToAMS, connectionStatus, connectionMessage } from '$lib/stores/mqtt';
  import type { FilamentProfile } from '$lib/stores/filaments';

  export let profile: FilamentProfile;
  export let onClose: () => void;

  let selectedAms = $settings.default_ams;
  let selectedTray = $settings.default_tray;
  let syncing = false;

  async function handleSync() {
    if (!$settings.printer_ip || !$settings.printer_serial || !$settings.access_code) {
      alert('Please configure printer settings first!');
      return;
    }

    syncing = true;
    try {
      const result = await syncToAMS(
        {
          ip: $settings.printer_ip,
          serial: $settings.printer_serial,
          access_code: $settings.access_code,
        },
        {
          ams_id: selectedAms,
          tray_id: selectedTray,
          tray_color: profile.color.replace('#', '') + 'FF',
          nozzle_temp_min: profile.nozzle_temp - 10,
          nozzle_temp_max: profile.nozzle_temp + 10,
          tray_type: profile.material,
        }
      );
      alert(result);
      onClose();
    } catch (error) {
      alert('Sync failed: ' + error);
    } finally {
      syncing = false;
    }
  }
</script>

<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" onclick={onClose}>
  <div
    class="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-8 max-w-md w-full m-4"
    onclick={(e) => e.stopPropagation()}
  >
    <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
      Sync to AMS
    </h3>

    <div class="mb-6">
      <div class="flex items-center gap-4 mb-4">
        <div
          class="w-16 h-16 rounded-lg border-2 border-gray-300 dark:border-gray-600"
          style="background-color: {profile.color}"
        ></div>
        <div>
          <p class="font-semibold text-gray-900 dark:text-white">{profile.brand}</p>
          <p class="text-gray-600 dark:text-gray-400">{profile.material}</p>
          <p class="text-sm text-gray-500">{profile.nozzle_temp}°C</p>
        </div>
      </div>
    </div>

    <div class="space-y-4 mb-6">
      <div>
        <label for="ams-select" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          AMS Unit
        </label>
        <select
          id="ams-select"
          bind:value={selectedAms}
          class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
        >
          <option value={0}>AMS 1</option>
          <option value={1}>AMS 2</option>
          <option value={2}>AMS 3</option>
          <option value={3}>AMS 4</option>
        </select>
      </div>

      <div>
        <label for="tray-select" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Tray
        </label>
        <select
          id="tray-select"
          bind:value={selectedTray}
          class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
        >
          <option value={0}>Tray 1</option>
          <option value={1}>Tray 2</option>
          <option value={2}>Tray 3</option>
          <option value={3}>Tray 4</option>
        </select>
      </div>
    </div>

    {#if $connectionStatus === 'syncing'}
      <div class="mb-4 p-3 bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300 rounded-lg text-sm">
        🔄 Syncing to printer...
      </div>
    {/if}

    {#if $connectionStatus === 'error'}
      <div class="mb-4 p-3 bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-300 rounded-lg text-sm">
        ❌ {$connectionMessage}
      </div>
    {/if}

    <div class="flex gap-3">
      <button
        onclick={onClose}
        class="flex-1 px-6 py-3 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors font-semibold"
        disabled={syncing}
      >
        Cancel
      </button>
      <button
        onclick={handleSync}
        class="flex-1 px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
        disabled={syncing}
      >
        {syncing ? 'Syncing...' : 'Sync'}
      </button>
    </div>
  </div>
</div>
