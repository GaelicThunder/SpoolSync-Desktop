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
  settings_extruder_temp: number | null;
  settings_bed_temp: number | null;
}

export interface SpoolmanResponse {
  items: SpoolmanFilament[];
  total: number;
}

let cachedFilaments: SpoolmanFilament[] | null = null;
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
    console.log('Using cached brands');
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
    console.log('Using cached materials');
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
