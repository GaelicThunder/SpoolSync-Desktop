<script lang="ts">
  import Header from '$lib/components/Header.svelte';

  let formData = {
    brand: '',
    material: '',
    color: '#FF5733',
    nozzleTemp: 220,
    bedTemp: 60,
    density: 1.24,
    diameter: 1.75,
  };

  function handleSubmit() {
    console.log('Creating custom profile:', formData);
    alert('Custom profile created! (Database integration coming in Phase 3)');
  }
</script>

<div class="flex flex-col h-full">
  <Header title="Custom Profiles" subtitle="Create and manage your own filament profiles" />

  <div class="p-8 max-w-3xl">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8">
      <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
        Create New Profile
      </h3>

      <form on:submit|preventDefault={handleSubmit} class="space-y-6">
        <div class="grid grid-cols-2 gap-6">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Brand
            </label>
            <input
              type="text"
              bind:value={formData.brand}
              placeholder="e.g., Generic PLA"
              required
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Material Type
            </label>
            <select
              bind:value={formData.material}
              required
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            >
              <option value="">Select material</option>
              <option value="PLA">PLA</option>
              <option value="PETG">PETG</option>
              <option value="ABS">ABS</option>
              <option value="TPU">TPU</option>
              <option value="Nylon">Nylon</option>
            </select>
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Color (Hex)
          </label>
          <div class="flex gap-3">
            <input
              type="color"
              bind:value={formData.color}
              class="h-12 w-24 border border-gray-300 dark:border-gray-600 rounded-lg cursor-pointer"
            />
            <input
              type="text"
              bind:value={formData.color}
              placeholder="#FF5733"
              required
              class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>
        </div>

        <div class="grid grid-cols-2 gap-6">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Nozzle Temp (°C)
            </label>
            <input
              type="number"
              bind:value={formData.nozzleTemp}
              min="150"
              max="300"
              required
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Bed Temp (°C)
            </label>
            <input
              type="number"
              bind:value={formData.bedTemp}
              min="0"
              max="120"
              required
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>
        </div>

        <div class="grid grid-cols-2 gap-6">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Density (g/cm³)
            </label>
            <input
              type="number"
              bind:value={formData.density}
              step="0.01"
              min="0.5"
              max="3.0"
              required
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Diameter (mm)
            </label>
            <select
              bind:value={formData.diameter}
              required
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none"
            >
              <option value={1.75}>1.75mm</option>
              <option value={2.85}>2.85mm</option>
            </select>
          </div>
        </div>

        <button
          type="submit"
          class="w-full px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 transition-colors font-semibold text-lg"
        >
          Create Profile
        </button>
      </form>

      <div class="mt-8 pt-8 border-t border-gray-200 dark:border-gray-700">
        <h4 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Your Custom Profiles
        </h4>
        <p class="text-gray-500 dark:text-gray-400 text-center py-8">
          No custom profiles yet. Create your first one above!
        </p>
      </div>
    </div>
  </div>
</div>
