import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';

export interface SpoolmanFilament {
  id: string;
  manufacturer: string;
  name: string;
  material: string;
  density: number;
  diameter: number;
  weight: number | null;
  spool_weight: number | null;
  color_hex: string | null;
  extruder_temp: number | null;
  bed_temp: number | null;
  extruder_temp_range: number[] | null;
  bed_temp_range: number[] | null;
  translucent: boolean;
  glow: boolean;
}

export interface SpoolmanResponse {
  items: SpoolmanFilament[];
  total: number;
}

export interface ColorGroup {
  hex: string;
  filaments: SpoolmanFilament[];
  materials: Set<string>;
  brands: Set<string>;
  hasTranslucent: boolean;
  hasGlow: boolean;
}

let cachedBrands: string[] | null = null;
let cachedMaterials: string[] | null = null;
let cachedColors: Map<string, ColorGroup> | null = null;

export const spoolmanFilaments = writable<SpoolmanFilament[]>([]);
export const spoolmanTotal = writable<number>(0);
export const spoolmanLoading = writable<boolean>(false);
export const spoolmanBrands = writable<string[]>([]);
export const spoolmanMaterials = writable<string[]>([]);
export const spoolmanColors = writable<Map<string, ColorGroup>>(new Map());

export async function searchSpoolman(
  query?: string,
  vendor?: string,
  material?: string,
  limit: number = 50,
  offset: number = 0
) {
  spoolmanLoading.set(true);
  try {
    const result = await invoke<SpoolmanResponse>('search_spoolman', {
      query: query || null,
      vendor: vendor || null,
      material: material || null,
      limit,
      offset,
    });
    
    if (offset === 0) {
      spoolmanFilaments.set(result.items);
    } else {
      spoolmanFilaments.update(items => [...items, ...result.items]);
    }
    
    spoolmanTotal.set(result.total);
  } catch (error) {
    console.error('Failed to search SpoolmanDB:', error);
    throw error;
  } finally {
    spoolmanLoading.set(false);
  }
}

export async function loadBrands() {
  if (cachedBrands) {
    spoolmanBrands.set(cachedBrands);
    return;
  }
  
  try {
    const brands = await invoke<string[]>('get_spoolman_brands');
    cachedBrands = brands;
    spoolmanBrands.set(brands);
  } catch (error) {
    console.error('Failed to load brands:', error);
  }
}

export async function loadMaterials() {
  if (cachedMaterials) {
    spoolmanMaterials.set(cachedMaterials);
    return;
  }
  
  try {
    const materials = await invoke<string[]>('get_spoolman_materials');
    cachedMaterials = materials;
    spoolmanMaterials.set(materials);
  } catch (error) {
    console.error('Failed to load materials:', error);
  }
}

export async function loadColors() {
  if (cachedColors) {
    console.log('Using cached colors');
    spoolmanColors.set(cachedColors);
    return;
  }
  
  try {
    const result = await invoke<SpoolmanResponse>('search_spoolman', {
      query: null,
      vendor: null,
      material: null,
      limit: 10000,
      offset: 0,
    });
    
    const colorMap = new Map<string, ColorGroup>();
    
    result.items.forEach(filament => {
      if (!filament.color_hex) return;
      
      const hex = filament.color_hex.replace('#', '').toLowerCase();
      
      if (!colorMap.has(hex)) {
        colorMap.set(hex, {
          hex: hex,
          filaments: [],
          materials: new Set(),
          brands: new Set(),
          hasTranslucent: false,
          hasGlow: false,
        });
      }
      
      const group = colorMap.get(hex)!;
      group.filaments.push(filament);
      group.materials.add(filament.material);
      group.brands.add(filament.manufacturer);
      if (filament.translucent) group.hasTranslucent = true;
      if (filament.glow) group.hasGlow = true;
    });
    
    console.log(`✅ Loaded ${colorMap.size} unique colors from SpoolmanDB`);
    cachedColors = colorMap;
    spoolmanColors.set(colorMap);
  } catch (error) {
    console.error('Failed to load colors:', error);
  }
}

export async function syncSpoolmanDB() {
  try {
    await invoke('sync_spoolman_db');
    cachedBrands = null;
    cachedMaterials = null;
    cachedColors = null;
    await loadBrands();
    await loadMaterials();
    await loadColors();
  } catch (error) {
    console.error('Failed to sync SpoolmanDB:', error);
    throw error;
  }
}

export async function debugFilament(filament: SpoolmanFilament) {
  try {
    await invoke('debug_filament', { filament });
  } catch (error) {
    console.error('Debug failed:', error);
  }
}
