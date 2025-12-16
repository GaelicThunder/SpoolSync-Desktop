<script lang="ts">
  import { onMount } from 'svelte';
  import Header from '$lib/components/Header.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { syncSpoolmanDB } from '$lib/stores/spoolman';

  let syncing = false;
  let syncMessage = '';

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
  <Header title="Settings" subtitle="Application configuration"></Header>

  <div class="p-8 max-w-3xl">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8 space-y-8">
      <div>
        <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
          SpoolmanDB
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
          About
        </h3>
        <p class="text-gray-600 dark:text-gray-400">
          SpoolSync Desktop v0.1.0
        </p>
        <p class="text-gray-500 dark:text-gray-500 text-sm mt-2">
          Manage your 3D printer filament profiles and sync with Bambu Lab AMS
        </p>
      </div>
    </div>
  </div>
</div>
