<script lang="ts">
  import Header from '$lib/components/Header.svelte';

  const mockColors = [
    { id: 1, name: 'Fire Red', hex: '#FF3333', brand: 'Bambu Lab' },
    { id: 2, name: 'Ocean Blue', hex: '#3366FF', brand: 'Polymaker' },
    { id: 3, name: 'Forest Green', hex: '#33AA33', brand: 'Prusament' },
    { id: 4, name: 'Sunset Orange', hex: '#FF9933', brand: 'eSUN' },
    { id: 5, name: 'Deep Purple', hex: '#9933FF', brand: 'ColorFabb' },
    { id: 6, name: 'Lemon Yellow', hex: '#FFFF33', brand: 'Bambu Lab' },
  ];
</script>

<div class="flex flex-col h-full">
  <Header title="Color Browser" subtitle="Visual gallery of filament colors"></Header>

  <div class="p-8 space-y-6">
    <div class="flex gap-4">
      <select class="px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none">
        <option value="">All Brands</option>
        <option value="bambu">Bambu Lab</option>
        <option value="polymaker">Polymaker</option>
        <option value="prusa">Prusament</option>
      </select>

      <select class="px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary outline-none">
        <option value="">All Materials</option>
        <option value="pla">PLA</option>
        <option value="petg">PETG</option>
        <option value="abs">ABS</option>
      </select>
    </div>

    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-6">
      {#each mockColors as color (color.id)}
        <button class="group bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-xl transition-all p-4">
          <div
            class="w-full aspect-square rounded-lg mb-3 border-2 border-gray-200 dark:border-gray-700 group-hover:border-primary transition-colors"
            style="background-color: {color.hex}"
          ></div>
          
          <h4 class="font-semibold text-gray-900 dark:text-white text-sm">
            {color.name}
          </h4>
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
            {color.brand}
          </p>
          <p class="text-xs text-gray-400 dark:text-gray-500 mt-1 font-mono">
            {color.hex}
          </p>
        </button>
      {/each}
    </div>

    <div class="text-center text-gray-500 dark:text-gray-400 mt-8">
      <p>Showing {mockColors.length} colors (mock data)</p>
      <p class="text-sm mt-2">Filament Colors API integration coming in Phase 5</p>
    </div>
  </div>
</div>
