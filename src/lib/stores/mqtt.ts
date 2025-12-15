import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';

export interface BambuPrinterConfig {
  ip: string;
  serial: string;
  access_code: string;
}

export interface FilamentSyncCommand {
  ams_id: number;
  tray_id: number;
  tray_color: string;
  nozzle_temp_min: number;
  nozzle_temp_max: number;
  tray_type: string;
}

export const connectionStatus = writable<'idle' | 'testing' | 'syncing' | 'success' | 'error'>('idle');
export const connectionMessage = writable<string>('');

export async function testConnection(config: BambuPrinterConfig): Promise<string> {
  connectionStatus.set('testing');
  try {
    const result = await invoke<string>('test_printer_connection', { config });
    connectionStatus.set('success');
    connectionMessage.set(result);
    setTimeout(() => connectionStatus.set('idle'), 3000);
    return result;
  } catch (error) {
    connectionStatus.set('error');
    connectionMessage.set(String(error));
    setTimeout(() => connectionStatus.set('idle'), 5000);
    throw error;
  }
}

export async function syncToAMS(
  config: BambuPrinterConfig,
  command: FilamentSyncCommand
): Promise<string> {
  connectionStatus.set('syncing');
  try {
    const result = await invoke<string>('sync_to_ams', { config, command });
    connectionStatus.set('success');
    connectionMessage.set(result);
    setTimeout(() => connectionStatus.set('idle'), 3000);
    return result;
  } catch (error) {
    connectionStatus.set('error');
    connectionMessage.set(String(error));
    setTimeout(() => connectionStatus.set('idle'), 5000);
    throw error;
  }
}
