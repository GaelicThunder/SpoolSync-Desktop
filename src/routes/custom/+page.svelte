<script lang="ts">
  import { onMount } from 'svelte';
  import Header from '$lib/components/Header.svelte';
  import { customProfiles, loadCustomProfiles, deleteCustomProfile, createCustomProfile, updateCustomProfile, type FilamentProfile } from '$lib/stores/filaments';

  let showCreateModal = false;
  let editingProfile: FilamentProfile | null = null;

  let formData = {
    brand: '',
    material: 'PLA',
    color: '#FF0000',
    nozzle_temp: 220,
    bed_temp: 60,
    density: 1.24,
    diameter: 1.75,
  };

  onMount(() => {
    loadCustomProfiles();
  });

  function openCreateModal() {
    editingProfile = null;
    formData = {
      brand: '',
      material: 'PLA',
      color: '#FF0000',
      nozzle_temp: 220,
      bed_temp: 60,
      density: 1.24,
      diameter: 1.75,
    };
    showCreateModal = true;
  }

  function openEditModal(profile: FilamentProfile) {
    editingProfile = profile;
    formData = {
      brand: profile.brand,
      material: profile.material,
      color: profile.color,
      nozzle_temp: profile.nozzle_temp,
      bed_temp: profile.bed_temp,
      density: profile.density,
      diameter: profile.diameter,
    };
    showCreateModal = true;
  }

  async function handleSave() {
    const profile: FilamentProfile = {
      ...formData,
      is_favorite: false,
      is_custom: true,
    };

    try {
      if (editingProfile && editingProfile.id) {
        await updateCustomProfile({ ...profile, id: editingProfile.id });
      } else {
        await createCustomProfile(profile);
      }
      showCreateModal = false;
    } catch (error) {
      alert('Failed to save: ' + error);
    }
  }

  async function handleDelete(id: number | undefined) {
    if (!id) return;
    if (confirm('Delete this custom profile?')) {
      await deleteCustomProfile(id);
    }
  }
</script>

<div class="flex flex-col h-full">
  <Header title="Custom Profiles" subtitle="Your custom filament profiles"></Header>

  <div class="p-8 space-y-6 flex-1 overflow-auto">
    <button
      onclick={openCreateModal}
      class="px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold"
    >
      ➕ Create Custom Profile
    </button>

    {#if $customProfiles.length === 0}
      <div class="text-center py-16">
        <p class="text-2xl mb-4">⚙️</p>
        <p class="text-gray-600 dark:text-gray-400 text-lg">
          No custom profiles yet
        </p>
        <p class="text-gray-500 dark:text-gray-500 mt-2">
          Create your own filament profiles with custom settings
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each $customProfiles as profile (profile.id)}
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
            <div class="flex items-start gap-4 mb-4">
              <div
                class="w-16 h-16 rounded-lg flex-shrink-0 border-2 border-gray-200 dark:border-gray-700"
                style="background-color: {profile.color}"
              ></div>
              
              <div class="flex-1">
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                  {profile.brand}
                </h3>
                <p class="text-gray-600 dark:text-gray-400 mt-1">
                  {profile.material}
                </p>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-2 text-xs text-gray-600 dark:text-gray-400 mb-4">
              <div>🌡️ Nozzle: {profile.nozzle_temp}°C</div>
              <div>🔥 Bed: {profile.bed_temp}°C</div>
              <div>📏 Density: {profile.density}</div>
              <div>📐 Diameter: {profile.diameter}mm</div>
            </div>

            <div class="flex gap-2">
              <button
                onclick={() => openEditModal(profile)}
                class="flex-1 px-4 py-2 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 rounded-lg hover:bg-blue-200 dark:hover:bg-blue-900/50 transition-colors text-sm font-medium"
              >
                ✏️ Edit
              </button>
              <button
                onclick={() => handleDelete(profile.id)}
                class="flex-1 px-4 py-2 bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300 rounded-lg hover:bg-red-200 dark:hover:bg-red-900/50 transition-colors text-sm font-medium"
              >
                🗑️ Delete
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

{#if showCreateModal}
  <div
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
    role="dialog"
    aria-modal="true"
    onclick={() => showCreateModal = false}
    onkeydown={(e) => e.key === 'Escape' && (showCreateModal = false)}
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl max-w-md w-full p-8"
      role="document"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
        {editingProfile ? 'Edit' : 'Create'} Custom Profile
      </h2>

      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            Brand
          </label>
          <input
            type="text"
            bind:value={formData.brand}
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            Material
          </label>
          <input
            type="text"
            bind:value={formData.material}
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            Color
          </label>
          <input
            type="color"
            bind:value={formData.color}
            class="w-full h-12 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 cursor-pointer"
          />
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Nozzle Temp (°C)
            </label>
            <input
              type="number"
              bind:value={formData.nozzle_temp}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Bed Temp (°C)
            </label>
            <input
              type="number"
              bind:value={formData.bed_temp}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Density (g/cm³)
            </label>
            <input
              type="number"
              step="0.01"
              bind:value={formData.density}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Diameter (mm)
            </label>
            <input
              type="number"
              step="0.05"
              bind:value={formData.diameter}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>
        </div>
      </div>

      <div class="flex gap-3 mt-6">
        <button
          onclick={() => showCreateModal = false}
          class="flex-1 px-6 py-3 border-2 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors font-semibold"
        >
          Cancel
        </button>
        <button
          onclick={handleSave}
          class="flex-1 px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold"
        >
          {editingProfile ? 'Update' : 'Create'}
        </button>
      </div>
    </div>
  </div>
{/if}
