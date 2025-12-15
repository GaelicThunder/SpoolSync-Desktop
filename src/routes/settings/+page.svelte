<script lang="ts">
  import { onMount } from 'svelte';
  import Header from '$lib/components/Header.svelte';
  import { settings, loadSettings, saveSettings, type Settings } from '$lib/stores/settings';
  import { testConnection, connectionStatus, connectionMessage } from '$lib/stores/mqtt';

  let localSettings: Settings = {
    printer_ip: '',
    printer_serial: '',
    access_code: '',
    default_ams: 0,
    default_tray: 0,
    auto_sync: false,
  };

  onMount(async () => {
    await loadSettings();
    localSettings = { ...$settings };
  });

  async function handleTestConnection() {
    if (!localSettings.printer_ip || !localSettings.printer_serial || !localSettings.access_code) {
      alert('Please fill in all printer connection details first.');
      return;
    }

    try {
      await testConnection({
        ip: localSettings.printer_ip,
        serial: localSettings.printer_serial,
        access_code: localSettings.access_code,
      });
    } catch (error) {
      console.error('Connection test failed:', error);
    }
  }

  async function handleSave() {
    try {
      await saveSettings(localSettings);
      alert('Settings saved successfully!');
    } catch (error) {
      alert('Failed to save settings: ' + error);
    }
  }
</script>

<div class="flex flex-col h-full">
  <Header title="Settings" subtitle="Configure your printer and app preferences"></Header>

  <div class="p-8 max-w-4xl">
    <div class="space-y-6">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8">
        <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-6">
          🖨️ Bambu Lab Printer
        </h3>

        <div class="space-y-6">
          <div>
            <label for="ip" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Printer IP Address
            </label>
            <input
              id="ip"
              type="text"
              bind:value={localSettings.printer_ip}
              placeholder="192.168.1.100"
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>

          <div>
            <label for="serial" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Serial Number
            </label>
            <input
              id="serial"
              type="text"
              bind:value={localSettings.printer_serial}
              placeholder="01S00A12345678"
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>

          <div>
            <label for="access" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Access Code
            </label>
            <input
              id="access"
              type="password"
              bind:value={localSettings.access_code}
              placeholder="********"
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
              Found in printer settings under Network → LAN Mode
            </p>
          </div>

          {#if $connectionStatus === 'testing'}
            <div class="p-3 bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300 rounded-lg text-sm">
              🔄 Testing connection...
            </div>
          {/if}

          {#if $connectionStatus === 'success'}
            <div class="p-3 bg-green-50 dark:bg-green-900/20 text-green-700 dark:text-green-300 rounded-lg text-sm">
              ✅ {$connectionMessage}
            </div>
          {/if}

          {#if $connectionStatus === 'error'}
            <div class="p-3 bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-300 rounded-lg text-sm">
              ❌ {$connectionMessage}
            </div>
          {/if}

          <button
            onclick={handleTestConnection}
            class="w-full px-6 py-3 border-2 border-primary text-primary rounded-lg hover:bg-primary hover:text-white transition-colors font-semibold disabled:opacity-50"
            disabled={$connectionStatus === 'testing'}
          >
            {$connectionStatus === 'testing' ? 'Testing...' : 'Test Connection'}
          </button>
        </div>
      </div>

      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8">
        <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-6">
          🎛️ Default AMS Settings
        </h3>

        <div class="grid grid-cols-2 gap-6">
          <div>
            <label for="ams" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Default AMS Unit
            </label>
            <select
              id="ams"
              bind:value={localSettings.default_ams}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            >
              <option value={0}>AMS 1</option>
              <option value={1}>AMS 2</option>
              <option value={2}>AMS 3</option>
              <option value={3}>AMS 4</option>
            </select>
          </div>

          <div>
            <label for="tray" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Default Tray
            </label>
            <select
              id="tray"
              bind:value={localSettings.default_tray}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            >
              <option value={0}>Tray 1</option>
              <option value={1}>Tray 2</option>
              <option value={2}>Tray 3</option>
              <option value={3}>Tray 4</option>
            </select>
          </div>
        </div>
      </div>

      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8">
        <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-6">
          ⚡ App Preferences
        </h3>

        <div class="space-y-4">
          <label for="autosync" class="flex items-center justify-between cursor-pointer">
            <div>
              <span class="font-medium text-gray-900 dark:text-white">Auto-sync on startup</span>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
                Automatically sync favorites when app starts
              </p>
            </div>
            <input
              id="autosync"
              type="checkbox"
              bind:checked={localSettings.auto_sync}
              class="w-6 h-6 text-primary focus:ring-primary border-gray-300 rounded"
            />
          </label>
        </div>
      </div>

      <button
        onclick={handleSave}
        class="w-full px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold text-lg"
      >
        Save Settings
      </button>
    </div>
  </div>
</div>
