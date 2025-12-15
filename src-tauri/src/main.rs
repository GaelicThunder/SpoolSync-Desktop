#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use db::Database;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FilamentProfile {
    id: Option<i64>,
    brand: String,
    material: String,
    color: String,
    nozzle_temp: i32,
    bed_temp: i32,
    density: f64,
    diameter: f64,
    is_favorite: bool,
    is_custom: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    printer_ip: String,
    printer_serial: String,
    access_code: String,
    default_ams: i32,
    default_tray: i32,
    auto_sync: bool,
}

struct AppState {
    db: Mutex<Database>,
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

fn main() {
    let db = Database::new().expect("Failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState { db: Mutex::new(db) })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_favorites,
            get_custom_profiles,
            add_favorite,
            remove_favorite,
            create_custom_profile,
            update_custom_profile,
            delete_custom_profile,
            get_settings,
            save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
