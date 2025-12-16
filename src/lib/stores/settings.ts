import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface Settings {
  printer_name?: string;
  printer_ip: string;
  printer_serial: string;
  printer_access_code: string;
  default_ams: number;
  default_tray: number;
  auto_sync: boolean;
}

export const settings = writable<Settings>({
  printer_name: '',
  printer_ip: '',
  printer_serial: '',
  printer_access_code: '',
  default_ams: 0,
  default_tray: 0,
  auto_sync: false,
});

export async function loadSettings() {
  try {
    const data = await invoke<Settings>('get_settings');
    settings.set(data);
  } catch (error) {
    console.error('Failed to load settings:', error);
  }
}

export async function getSettings(): Promise<Settings> {
  try {
    return await invoke<Settings>('get_settings');
  } catch (error) {
    console.error('Failed to get settings:', error);
    return {
      printer_name: '',
      printer_ip: '',
      printer_serial: '',
      printer_access_code: '',
      default_ams: 0,
      default_tray: 0,
      auto_sync: false,
    };
  }
}

export async function saveSettings(data: Settings) {
  await invoke('save_settings', { settings: data });
  settings.set(data);
}
