<script lang="ts">
  import { onMount } from 'svelte';
  import Header from '$lib/components/Header.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { syncSpoolmanDB } from '$lib/stores/spoolman';

  interface Settings {
    printer_name: string | null;
    printer_ip: string;
    printer_serial: string;
    printer_access_code: string;
    default_ams: number;
    default_tray: number;
    auto_sync: boolean;
  }

  let settings: Settings = {
    printer_name: '',
    printer_ip: '',
    printer_serial: '',
    printer_access_code: '',
    default_ams: 0,
    default_tray: 0,
    auto_sync: false
  };

  let loading = true;
  let saving = false;
  let testing = false;
  let syncing = false;
  let saveMessage = '';
  let testMessage = '';
  let syncMessage = '';

  onMount(async () => {
    try {
      settings = await invoke<Settings>('get_settings');
    } catch (error) {
      console.error('Failed to load settings:', error);
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    saving = true;
    saveMessage = '';
    try {
      await invoke('save_settings', { settings });
      saveMessage = '✅ Settings saved successfully!';
      setTimeout(() => saveMessage = '', 3000);
    } catch (error) {
      saveMessage = '❌ Failed to save: ' + error;
    } finally {
      saving = false;
    }
  }

  async function handleTestConnection() {
    testing = true;
    testMessage = '';
    try {
      const config = {
        ip: settings.printer_ip,
        serial: settings.printer_serial,
        access_code: settings.printer_access_code
      };
      const result = await invoke<string>('test_printer_connection', { config });
      testMessage = '✅ ' + result;
    } catch (error) {
      testMessage = '❌ Connection failed: ' + error;
    } finally {
      testing = false;
    }
  }

  async function handleSyncDB() {
    syncing = true;
    syncMessage = '';
    try {
      await syncSpoolmanDB();
      syncMessage = '✅ SpoolmanDB synced successfully!';
      setTimeout(() => syncMessage = '', 3000);
    } catch (error) {
      syncMessage = '❌ Failed to sync: ' + error;
    } finally {
      syncing = false;
    }
  }
</script>

<div class="flex flex-col h-full">
  <Header title="Settings" subtitle="Printer, AMS, and application configuration"></Header>

  <div class="p-8 max-w-3xl">
    {#if loading}
      <div class="flex justify-center items-center py-12">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
      </div>
    {:else}
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8 space-y-8">
        <div>
          <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
            🖨️ Printer Configuration
          </h3>
          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Printer Name (optional)
              </label>
              <input
                type="text"
                bind:value={settings.printer_name}
                placeholder="My Bambu Lab X1C"
                class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Printer IP Address *
              </label>
              <input
                type="text"
                bind:value={settings.printer_ip}
                placeholder="192.168.1.100"
                required
                class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Printer Serial Number *
              </label>
              <input
                type="text"
                bind:value={settings.printer_serial}
                placeholder="01P00A123B456C78"
                required
                class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Access Code *
              </label>
              <input
                type="password"
                bind:value={settings.printer_access_code}
                placeholder="12345678"
                required
                class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent"
              />
              <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                Found in printer settings → Network → Access Code
              </p>
            </div>

            <div class="flex gap-3">
              <button
                onclick={handleTestConnection}
                disabled={testing || !settings.printer_ip || !settings.printer_serial || !settings.printer_access_code}
                class="px-6 py-2 bg-secondary text-white rounded-lg hover:bg-green-700 transition-colors font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {testing ? '🔄 Testing...' : '🔌 Test Connection'}
              </button>
            </div>

            {#if testMessage}
              <p class="text-sm {testMessage.startsWith('✅') ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'}">
                {testMessage}
              </p>
            {/if}
          </div>
        </div>

        <div class="border-t border-gray-200 dark:border-gray-700 pt-8">
          <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
            🧵 AMS Configuration
          </h3>
          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Default AMS Unit
              </label>
              <select
                bind:value={settings.default_ams}
                class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent"
              >
                <option value={0}>AMS 1</option>
                <option value={1}>AMS 2</option>
                <option value={2}>AMS 3</option>
                <option value={3}>AMS 4</option>
              </select>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Default Tray/Slot
              </label>
              <select
                bind:value={settings.default_tray}
                class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent"
              >
                <option value={0}>Tray 1</option>
                <option value={1}>Tray 2</option>
                <option value={2}>Tray 3</option>
                <option value={3}>Tray 4</option>
              </select>
            </div>

            <div class="flex items-center gap-3">
              <input
                type="checkbox"
                id="auto_sync"
                bind:checked={settings.auto_sync}
                class="w-5 h-5 text-primary bg-gray-100 border-gray-300 rounded focus:ring-primary dark:focus:ring-primary dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
              />
              <label for="auto_sync" class="text-sm font-medium text-gray-700 dark:text-gray-300">
                Auto-sync on startup
              </label>
            </div>
            <p class="text-xs text-gray-500 dark:text-gray-400">
              Automatically connect and sync settings when the app starts
            </p>
          </div>
        </div>

        <div class="border-t border-gray-200 dark:border-gray-700 pt-8">
          <button
            onclick={handleSave}
            disabled={saving}
            class="w-full px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {saving ? '💾 Saving...' : '💾 Save Settings'}
          </button>
          {#if saveMessage}
            <p class="mt-3 text-sm text-center {saveMessage.startsWith('✅') ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'}">
              {saveMessage}
            </p>
          {/if}
        </div>

        <div class="border-t border-gray-200 dark:border-gray-700 pt-8">
          <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
            🗄️ SpoolmanDB
          </h3>
          <p class="text-gray-600 dark:text-gray-400 mb-4">
            The database is downloaded once at startup and cached. Click the button below to manually sync with the latest data.
          </p>
          <button
            onclick={handleSyncDB}
            disabled={syncing}
            class="px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {syncing ? '🔄 Syncing...' : '🔄 Sync SpoolmanDB'}
          </button>
          {#if syncMessage}
            <p class="mt-3 text-sm {syncMessage.startsWith('✅') ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'}">
              {syncMessage}
            </p>
          {/if}
        </div>

        <div class="border-t border-gray-200 dark:border-gray-700 pt-8">
          <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
            ℹ️ About
          </h3>
          <p class="text-gray-600 dark:text-gray-400">
            SpoolSync Desktop v0.1.0-alpha
          </p>
          <p class="text-gray-500 dark:text-gray-500 text-sm mt-2">
            Manage your 3D printer filament profiles and sync with Bambu Lab AMS
          </p>
        </div>
      </div>
    {/if}
  </div>
</div>
