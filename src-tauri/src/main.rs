#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod mqtt;
mod spoolman;
mod filamentcolors;
mod bambu_studio;

use db::{Database, FilamentProfile, Settings};
use mqtt::{BambuMqttClient, BambuPrinterConfig, FilamentSyncCommand, AMSStatus};
use spoolman::{SpoolmanClient, SpoolmanFilament, SpoolmanResponse};
use filamentcolors::{FilamentColorsClient, FilamentColorsResponse};
use bambu_studio::{BambuStudioManager, BambuFilamentProfile};
use std::sync::{Arc, Mutex};
use tauri::State;

struct AppState {
    db: Mutex<Database>,
    mqtt: Mutex<BambuMqttClient>,
    spoolman: Arc<SpoolmanClient>,
    filament_colors: Arc<FilamentColorsClient>,
    bambu_studio: Mutex<Option<BambuStudioManager>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to SpoolSync Desktop.", name)
}

#[tauri::command]
fn get_favorites(state: State<AppState>) -> Result<Vec<FilamentProfile>, String> {
    let db = state.db.lock().unwrap();
    db.get_favorites().map_err(|e| e.to_string())
}

#[tauri::command]
fn get_custom_profiles(state: State<AppState>) -> Result<Vec<FilamentProfile>, String> {
    let db = state.db.lock().unwrap();
    db.get_custom_profiles().map_err(|e| e.to_string())
}

#[tauri::command]
fn get_brands(state: State<AppState>) -> Result<Vec<String>, String> {
    let db = state.db.lock().unwrap();
    db.get_brands().map_err(|e| e.to_string())
}

#[tauri::command]
fn get_materials(state: State<AppState>) -> Result<Vec<String>, String> {
    let db = state.db.lock().unwrap();
    db.get_materials().map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_spoolman_materials(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let spoolman = Arc::clone(&state.spoolman);
    spoolman.get_materials().await
}

#[tauri::command]
fn add_favorite(state: State<AppState>, profile: FilamentProfile) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    db.add_favorite(profile).map_err(|e| e.to_string())
}

#[tauri::command]
fn remove_favorite(state: State<AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.remove_favorite(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_custom_profile(state: State<AppState>, profile: FilamentProfile) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    db.create_custom_profile(profile).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_custom_profile(state: State<AppState>, profile: FilamentProfile) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.update_custom_profile(profile).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_custom_profile(state: State<AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.delete_custom_profile(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_settings(state: State<AppState>) -> Result<Settings, String> {
    let db = state.db.lock().unwrap();
    db.get_settings().map_err(|e| e.to_string())
}

#[tauri::command]
fn save_settings(state: State<AppState>, settings: Settings) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.save_settings(settings).map_err(|e| e.to_string())
}

#[tauri::command]
fn test_printer_connection(
    state: State<AppState>,
    config: BambuPrinterConfig,
) -> Result<String, String> {
    let mqtt = state.mqtt.lock().unwrap();
    mqtt.test_connection(config)
}

#[tauri::command]
fn get_ams_status(
    state: State<AppState>,
    config: BambuPrinterConfig,
) -> Result<Vec<AMSStatus>, String> {
    let mqtt = state.mqtt.lock().unwrap();
    mqtt.get_ams_status(config)
}

#[tauri::command]
fn sync_to_ams(
    state: State<AppState>,
    config: BambuPrinterConfig,
    command: FilamentSyncCommand,
) -> Result<String, String> {
    let mqtt = state.mqtt.lock().unwrap();
    mqtt.sync_filament(config, command)
}

#[tauri::command]
async fn search_spoolman(
    state: State<'_, AppState>,
    query: Option<String>,
    vendor: Option<String>,
    material: Option<String>,
    limit: usize,
    offset: usize,
) -> Result<SpoolmanResponse, String> {
    let spoolman = Arc::clone(&state.spoolman);
    spoolman
        .search_filaments(query, vendor, material, limit, offset)
        .await
}

#[tauri::command]
async fn get_spoolman_brands(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let spoolman = Arc::clone(&state.spoolman);
    spoolman.get_brands().await
}

#[tauri::command]
async fn sync_spoolman_db(state: State<'_, AppState>) -> Result<(), String> {
    let spoolman = Arc::clone(&state.spoolman);
    spoolman.sync_database().await
}

#[tauri::command]
async fn get_filament_swatches(
    state: State<'_, AppState>,
    manufacturer: Option<String>,
    material_type: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<FilamentColorsResponse, String> {
    let client = Arc::clone(&state.filament_colors);
    client.get_swatches(manufacturer, material_type, limit, offset).await
}

#[tauri::command]
fn debug_filament(filament: SpoolmanFilament) {
    println!("\n🔍 DEBUG FILAMENT DATA:");
    println!("════════════════════════════════════════");
    println!("ID: {}", filament.id);
    println!("Manufacturer: {}", filament.manufacturer);
    println!("Name: {}", filament.name);
    println!("Material: {}", filament.material);
    println!("Density: {} g/cm³", filament.density);
    println!("Diameter: {} mm", filament.diameter);
    println!("Color Hex: {:?}", filament.color_hex);
    println!("Weight: {:?} g", filament.weight);
    println!("Spool Weight: {:?} g", filament.spool_weight);
    println!("Extruder Temp: {:?}°C", filament.extruder_temp);
    println!("Extruder Temp Range: {:?}", filament.extruder_temp_range);
    println!("Bed Temp: {:?}°C", filament.bed_temp);
    println!("Bed Temp Range: {:?}", filament.bed_temp_range);
    println!("Translucent: {}", filament.translucent);
    println!("Glow: {}", filament.glow);
    println!("════════════════════════════════════════\n");
}

#[tauri::command]
fn list_bambu_profiles(state: State<AppState>) -> Result<Vec<String>, String> {
    let manager_opt = state.bambu_studio.lock().unwrap();
    match manager_opt.as_ref() {
        Some(manager) => manager.list_profiles(),
        None => Err("Bambu Studio not configured".to_string()),
    }
}

#[tauri::command]
fn read_bambu_profile(
    state: State<AppState>,
    name: String,
) -> Result<BambuFilamentProfile, String> {
    let manager_opt = state.bambu_studio.lock().unwrap();
    match manager_opt.as_ref() {
        Some(manager) => manager.read_profile(&name),
        None => Err("Bambu Studio not configured".to_string()),
    }
}

#[tauri::command]
fn create_bambu_profile(
    state: State<AppState>,
    profile: BambuFilamentProfile,
) -> Result<String, String> {
    let manager_opt = state.bambu_studio.lock().unwrap();
    match manager_opt.as_ref() {
        Some(manager) => manager.create_profile(&profile),
        None => Err("Bambu Studio not configured".to_string()),
    }
}

#[tauri::command]
fn update_bambu_profile(
    state: State<AppState>,
    name: String,
    profile: BambuFilamentProfile,
) -> Result<(), String> {
    let manager_opt = state.bambu_studio.lock().unwrap();
    match manager_opt.as_ref() {
        Some(manager) => manager.update_profile(&name, &profile),
        None => Err("Bambu Studio not configured".to_string()),
    }
}

#[tauri::command]
fn delete_bambu_profile(state: State<AppState>, name: String) -> Result<(), String> {
    let manager_opt = state.bambu_studio.lock().unwrap();
    match manager_opt.as_ref() {
        Some(manager) => manager.delete_profile(&name),
        None => Err("Bambu Studio not configured".to_string()),
    }
}

#[tauri::command]
fn sync_spoolman_to_bambu_studio(
    state: State<AppState>,
    vendor: String,
    name: String,
    material: String,
    color_hex: String,
    nozzle_temp: u16,
    bed_temp: u16,
    density: f32,
) -> Result<String, String> {
    let manager_opt = state.bambu_studio.lock().unwrap();
    match manager_opt.as_ref() {
        Some(manager) => manager.create_from_spoolman(
            &vendor,
            &name,
            &material,
            &color_hex,
            nozzle_temp,
            bed_temp,
            density,
        ),
        None => Err("Bambu Studio not configured".to_string()),
    }
}

fn main() {
    let db = Database::new().expect("Failed to initialize database");
    let mqtt = BambuMqttClient::new().expect("Failed to initialize MQTT client");
    let spoolman = Arc::new(SpoolmanClient::new());
    let filament_colors = Arc::new(FilamentColorsClient::new());
    
    let bambu_studio = match BambuStudioManager::new() {
        Ok(manager) => {
            println!("✅ Bambu Studio integration enabled");
            Some(manager)
        }
        Err(e) => {
            println!("⚠️  Bambu Studio integration disabled: {}", e);
            None
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            db: Mutex::new(db),
            mqtt: Mutex::new(mqtt),
            spoolman,
            filament_colors,
            bambu_studio: Mutex::new(bambu_studio),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_favorites,
            get_custom_profiles,
            get_brands,
            get_materials,
            get_spoolman_materials,
            add_favorite,
            remove_favorite,
            create_custom_profile,
            update_custom_profile,
            delete_custom_profile,
            get_settings,
            save_settings,
            test_printer_connection,
            get_ams_status,
            sync_to_ams,
            search_spoolman,
            get_spoolman_brands,
            sync_spoolman_db,
            get_filament_swatches,
            debug_filament,
            list_bambu_profiles,
            read_bambu_profile,
            create_bambu_profile,
            update_bambu_profile,
            delete_bambu_profile,
            sync_spoolman_to_bambu_studio,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
