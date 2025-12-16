import { writable } from 'svelte/store';
import type { SpoolmanFilament } from './spoolman';

export const pendingFilamentId = writable<string | null>(null);

export interface FilamentShareData {
  v: 1;
  id: string;
  manufacturer: string;
  name: string;
  material: string;
  density: number;
  diameter: number;
  color_hex?: string;
  weight?: number;
  spool_weight?: number;
  extruder_temp?: number;
  bed_temp?: number;
  translucent?: boolean;
  glow?: boolean;
}

export function encodeFilamentData(filament: SpoolmanFilament): string {
  const data: FilamentShareData = {
    v: 1,
    id: filament.id,
    manufacturer: filament.manufacturer,
    name: filament.name,
    material: filament.material,
    density: filament.density,
    diameter: filament.diameter,
    color_hex: filament.color_hex || undefined,
    weight: filament.weight || undefined,
    spool_weight: filament.spool_weight || undefined,
    extruder_temp: filament.extruder_temp || undefined,
    bed_temp: filament.bed_temp || undefined,
    translucent: filament.translucent || undefined,
    glow: filament.glow || undefined,
  };
  
  const json = JSON.stringify(data);
  return btoa(json);
}

export function decodeFilamentData(encoded: string): FilamentShareData | null {
  try {
    const json = atob(encoded);
    const data = JSON.parse(json) as FilamentShareData;
    
    if (data.v !== 1) {
      console.error('Unsupported filament data version:', data.v);
      return null;
    }
    
    return data;
  } catch (error) {
    console.error('Failed to decode filament data:', error);
    return null;
  }
}

export function generateShareableLink(filament: SpoolmanFilament): string {
  const encoded = encodeFilamentData(filament);
  return `spoolsync://filament/${encoded}`;
}

export function generateShareableURL(filament: SpoolmanFilament): string {
  const encoded = encodeFilamentData(filament);
  return `https://spoolsync.app/f/${encoded}`;
}
