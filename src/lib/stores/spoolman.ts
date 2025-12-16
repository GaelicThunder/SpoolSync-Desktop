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
}

export interface SpoolmanResponse {
  items: SpoolmanFilament[];
  total: number;
}

let cachedBrands: string[] | null = null;
let cachedMaterials: string[] | null = null;

export const spoolmanFilaments = writable<SpoolmanFilament[]>([]);
export const spoolmanTotal = writable<number>(0);
export const spoolmanLoading = writable<boolean>(false);
export const spoolmanBrands = writable<string[]>([]);
export const spoolmanMaterials = writable<string[]>([]);

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

export async function syncSpoolmanDB() {
  try {
    await invoke('sync_spoolman_db');
    cachedBrands = null;
    cachedMaterials = null;
    await loadBrands();
    await loadMaterials();
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
