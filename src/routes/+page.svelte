<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let greetMsg = '';
  let name = 'SpoolSync';

  async function greet() {
    greetMsg = await invoke('greet', { name });
  }

  onMount(() => {
    greet();
  });
</script>

<main class="flex flex-col items-center justify-center min-h-screen p-8">
  <div class="text-center">
    <h1 class="text-6xl font-bold mb-4 text-gray-900 dark:text-white">
      SpoolSync Desktop
    </h1>
    <p class="text-xl text-gray-600 dark:text-gray-400 mb-8">
      Manage your 3D printer filament profiles
    </p>

    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8 max-w-md">
      <div class="mb-6">
        <input
          type="text"
          bind:value={name}
          placeholder="Enter name..."
          class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
        />
      </div>

      <button
        onclick={greet}
        class="w-full bg-primary hover:bg-blue-700 text-white font-semibold py-3 px-6 rounded-lg transition-colors"
      >
        Greet
      </button>

      {#if greetMsg}
        <p class="mt-6 text-lg text-gray-700 dark:text-gray-300">
          {greetMsg}
        </p>
      {/if}
    </div>

    <div class="mt-8 text-sm text-gray-500 dark:text-gray-400">
      <p>Built with Tauri v2 + Svelte 5</p>
      <p class="mt-2">Phase 1 Complete - Base setup ready ✅</p>
    </div>
  </div>
</main>
