import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface FilamentProfile {
  id?: number;
  brand: string;
  material: string;
  color: string;
  nozzle_temp: number;
  bed_temp: number;
  density: number;
  diameter: number;
  is_favorite: boolean;
  is_custom: boolean;
}

export const favorites = writable<FilamentProfile[]>([]);
export const customProfiles = writable<FilamentProfile[]>([]);

export async function loadFavorites() {
  try {
    const data = await invoke<FilamentProfile[]>('get_favorites');
    favorites.set(data);
  } catch (error) {
    console.error('Failed to load favorites:', error);
  }
}

export async function loadCustomProfiles() {
  try {
    const data = await invoke<FilamentProfile[]>('get_custom_profiles');
    customProfiles.set(data);
  } catch (error) {
    console.error('Failed to load custom profiles:', error);
  }
}

export async function addFavorite(profile: FilamentProfile): Promise<number> {
  const id = await invoke<number>('add_favorite', { profile });
  await loadFavorites();
  return id;
}

export async function removeFavorite(id: number) {
  await invoke('remove_favorite', { id });
  await loadFavorites();
}

export async function createCustomProfile(profile: FilamentProfile): Promise<number> {
  const id = await invoke<number>('create_custom_profile', { profile });
  await loadCustomProfiles();
  await loadFavorites();
  return id;
}

export async function updateCustomProfile(profile: FilamentProfile) {
  await invoke('update_custom_profile', { profile });
  await loadCustomProfiles();
}

export async function deleteCustomProfile(id: number) {
  await invoke('delete_custom_profile', { id });
  await loadCustomProfiles();
  await loadFavorites();
}
