<script lang="ts">
  import Header from '$lib/components/Header.svelte';

  let settings = {
    printerIp: '',
    printerSerial: '',
    accessCode: '',
    defaultAms: 0,
    defaultTray: 0,
    autoSync: false,
    darkMode: false,
  };

  function testConnection() {
    alert('Testing connection... (MQTT integration coming in Phase 7)');
  }

  function saveSettings() {
    console.log('Saving settings:', settings);
    alert('Settings saved! (Database integration coming in Phase 3)');
  }
</script>

<div class="flex flex-col h-full">
  <Header title="Settings" subtitle="Configure your printer and app preferences" />

  <div class="p-8 max-w-4xl">
    <div class="space-y-6">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8">
        <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-6">
          🖨️ Bambu Lab Printer
        </h3>

        <div class="space-y-6">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Printer IP Address
            </label>
            <input
              type="text"
              bind:value={settings.printerIp}
              placeholder="192.168.1.100"
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Serial Number
            </label>
            <input
              type="text"
              bind:value={settings.printerSerial}
              placeholder="01S00A12345678"
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Access Code
            </label>
            <input
              type="password"
              bind:value={settings.accessCode}
              placeholder="********"
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
              Found in printer settings under Network → LAN Mode
            </p>
          </div>

          <button
            on:click={testConnection}
            class="w-full px-6 py-3 border-2 border-primary text-primary rounded-lg hover:bg-primary hover:text-white transition-colors font-semibold"
          >
            Test Connection
          </button>
        </div>
      </div>

      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8">
        <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-6">
          🎛️ Default AMS Settings
        </h3>

        <div class="grid grid-cols-2 gap-6">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Default AMS Unit
            </label>
            <select
              bind:value={settings.defaultAms}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            >
              <option value={0}>AMS 1</option>
              <option value={1}>AMS 2</option>
              <option value={2}>AMS 3</option>
              <option value={3}>AMS 4</option>
            </select>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Default Tray
            </label>
            <select
              bind:value={settings.defaultTray}
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
          <label class="flex items-center justify-between cursor-pointer">
            <div>
              <span class="font-medium text-gray-900 dark:text-white">Auto-sync on startup</span>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
                Automatically sync favorites when app starts
              </p>
            </div>
            <input
              type="checkbox"
              bind:checked={settings.autoSync}
              class="w-6 h-6 text-primary focus:ring-primary border-gray-300 rounded"
            />
          </label>

          <label class="flex items-center justify-between cursor-pointer">
            <div>
              <span class="font-medium text-gray-900 dark:text-white">Dark mode</span>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
                Use dark theme (coming soon)
              </p>
            </div>
            <input
              type="checkbox"
              bind:checked={settings.darkMode}
              disabled
              class="w-6 h-6 text-primary focus:ring-primary border-gray-300 rounded opacity-50"
            />
          </label>
        </div>
      </div>

      <button
        on:click={saveSettings}
        class="w-full px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold text-lg"
      >
        Save Settings
      </button>
    </div>
  </div>
</div>
