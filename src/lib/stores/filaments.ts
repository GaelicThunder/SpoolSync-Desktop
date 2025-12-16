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
    console.log('Loading favorites...');
    const data = await invoke<FilamentProfile[]>('get_favorites');
    console.log('Favorites loaded:', data);
    favorites.set(data);
  } catch (error) {
    console.error('Failed to load favorites:', error);
  }
}

export async function loadCustomProfiles() {
  try {
    console.log('Loading custom profiles...');
    const data = await invoke<FilamentProfile[]>('get_custom_profiles');
    console.log('Custom profiles loaded:', data);
    customProfiles.set(data);
  } catch (error) {
    console.error('Failed to load custom profiles:', error);
  }
}

export async function addFavorite(profile: FilamentProfile): Promise<number> {
  console.log('Adding favorite:', profile);
  const id = await invoke<number>('add_favorite', { profile });
  console.log('Favorite added with ID:', id);
  await loadFavorites();
  return id;
}

export async function removeFavorite(id: number) {
  console.log('Removing favorite:', id);
  await invoke('remove_favorite', { id });
  await loadFavorites();
}

export async function createCustomProfile(profile: FilamentProfile): Promise<number> {
  console.log('Creating custom profile:', profile);
  const id = await invoke<number>('create_custom_profile', { profile });
  console.log('Custom profile created with ID:', id);
  await loadCustomProfiles();
  await loadFavorites();
  return id;
}

export async function updateCustomProfile(profile: FilamentProfile) {
  console.log('Updating custom profile:', profile);
  await invoke('update_custom_profile', { profile });
  await loadCustomProfiles();
}

export async function deleteCustomProfile(id: number) {
  console.log('Deleting custom profile:', id);
  await invoke('delete_custom_profile', { id });
  await loadCustomProfiles();
  await loadFavorites();
}
