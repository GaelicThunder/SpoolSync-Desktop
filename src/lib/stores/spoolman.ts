import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';

export interface SpoolmanVendor {
  id: number;
  name: string;
}

export interface SpoolmanFilament {
  id: number;
  name: string | null;
  vendor: SpoolmanVendor | null;
  material: string | null;
  density: number;
  diameter: number;
  color_hex: string | null;
  spool_weight: number | null;
  settings_extruder_temp: number | null;
  settings_bed_temp: number | null;
}

export interface SpoolmanResponse {
  items: SpoolmanFilament[];
  total: number;
}

export const spoolmanFilaments = writable<SpoolmanFilament[]>([]);
export const spoolmanTotal = writable<number>(0);
export const spoolmanLoading = writable<boolean>(false);
export const spoolmanBrands = writable<string[]>([]);

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
    spoolmanFilaments.set(result.items);
    spoolmanTotal.set(result.total);
  } catch (error) {
    console.error('Failed to search SpoolmanDB:', error);
    throw error;
  } finally {
    spoolmanLoading.set(false);
  }
}

export async function loadBrands() {
  try {
    const brands = await invoke<string[]>('get_spoolman_brands');
    spoolmanBrands.set(brands);
  } catch (error) {
    console.error('Failed to load brands:', error);
  }
}
