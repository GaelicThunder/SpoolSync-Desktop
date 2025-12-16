use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilamentProfile {
    pub id: Option<i64>,
    pub brand: String,
    pub material: String,
    pub color: String,
    pub nozzle_temp: i32,
    pub bed_temp: i32,
    pub density: f64,
    pub diameter: f64,
    pub is_favorite: bool,
    pub is_custom: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub printer_name: Option<String>,
    pub printer_ip: String,
    pub printer_serial: String,
    pub printer_access_code: String,
    pub default_ams: i32,
    pub default_tray: i32,
    pub auto_sync: bool,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let data_dir = dirs::data_local_dir()
            .ok_or_else(|| rusqlite::Error::InvalidPath(PathBuf::from("No data directory")))?;
        let app_dir = data_dir.join("spoolsync-desktop");
        std::fs::create_dir_all(&app_dir)?;

        let db_path = app_dir.join("spoolsync.db");
        println!("📂 Database path: {:?}", db_path);

        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS filament_profiles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                brand TEXT NOT NULL,
                material TEXT NOT NULL,
                color TEXT NOT NULL,
                nozzle_temp INTEGER NOT NULL,
                bed_temp INTEGER NOT NULL,
                density REAL NOT NULL,
                diameter REAL NOT NULL,
                is_favorite INTEGER NOT NULL DEFAULT 0,
                is_custom INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                printer_name TEXT,
                printer_ip TEXT NOT NULL DEFAULT '',
                printer_serial TEXT NOT NULL DEFAULT '',
                printer_access_code TEXT NOT NULL DEFAULT '',
                default_ams INTEGER NOT NULL DEFAULT 0,
                default_tray INTEGER NOT NULL DEFAULT 0,
                auto_sync INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;

        conn.execute(
            "INSERT OR IGNORE INTO settings (id, printer_ip, printer_serial, printer_access_code) VALUES (1, '', '', '')",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn get_favorites(&self) -> Result<Vec<FilamentProfile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, brand, material, color, nozzle_temp, bed_temp, density, diameter, is_favorite, is_custom 
             FROM filament_profiles WHERE is_favorite = 1",
        )?;

        let profiles = stmt
            .query_map([], |row| {
                Ok(FilamentProfile {
                    id: Some(row.get(0)?),
                    brand: row.get(1)?,
                    material: row.get(2)?,
                    color: row.get(3)?,
                    nozzle_temp: row.get(4)?,
                    bed_temp: row.get(5)?,
                    density: row.get(6)?,
                    diameter: row.get(7)?,
                    is_favorite: row.get::<_, i32>(8)? == 1,
                    is_custom: row.get::<_, i32>(9)? == 1,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(profiles)
    }

    pub fn get_custom_profiles(&self) -> Result<Vec<FilamentProfile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, brand, material, color, nozzle_temp, bed_temp, density, diameter, is_favorite, is_custom 
             FROM filament_profiles WHERE is_custom = 1",
        )?;

        let profiles = stmt
            .query_map([], |row| {
                Ok(FilamentProfile {
                    id: Some(row.get(0)?),
                    brand: row.get(1)?,
                    material: row.get(2)?,
                    color: row.get(3)?,
                    nozzle_temp: row.get(4)?,
                    bed_temp: row.get(5)?,
                    density: row.get(6)?,
                    diameter: row.get(7)?,
                    is_favorite: row.get::<_, i32>(8)? == 1,
                    is_custom: row.get::<_, i32>(9)? == 1,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(profiles)
    }

    pub fn get_brands(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT DISTINCT brand FROM filament_profiles ORDER BY brand")?;
        let brands = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>>>()?;
        Ok(brands)
    }

    pub fn get_materials(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT DISTINCT material FROM filament_profiles ORDER BY material")?;
        let materials = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>>>()?;
        Ok(materials)
    }

    pub fn add_favorite(&self, profile: FilamentProfile) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO filament_profiles (brand, material, color, nozzle_temp, bed_temp, density, diameter, is_favorite, is_custom)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1, 0)",
            params![
                profile.brand,
                profile.material,
                profile.color,
                profile.nozzle_temp,
                profile.bed_temp,
                profile.density,
                profile.diameter,
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn remove_favorite(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM filament_profiles WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn create_custom_profile(&self, profile: FilamentProfile) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO filament_profiles (brand, material, color, nozzle_temp, bed_temp, density, diameter, is_favorite, is_custom)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, 1)",
            params![
                profile.brand,
                profile.material,
                profile.color,
                profile.nozzle_temp,
                profile.bed_temp,
                profile.density,
                profile.diameter,
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn update_custom_profile(&self, profile: FilamentProfile) -> Result<()> {
        self.conn.execute(
            "UPDATE filament_profiles SET brand = ?1, material = ?2, color = ?3, nozzle_temp = ?4, bed_temp = ?5, density = ?6, diameter = ?7
             WHERE id = ?8",
            params![
                profile.brand,
                profile.material,
                profile.color,
                profile.nozzle_temp,
                profile.bed_temp,
                profile.density,
                profile.diameter,
                profile.id,
            ],
        )?;
        Ok(())
    }

    pub fn delete_custom_profile(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM filament_profiles WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_settings(&self) -> Result<Settings> {
        let mut stmt = self.conn.prepare(
            "SELECT printer_name, printer_ip, printer_serial, printer_access_code, default_ams, default_tray, auto_sync FROM settings WHERE id = 1",
        )?;

        let settings = stmt.query_row([], |row| {
            Ok(Settings {
                printer_name: row.get(0)?,
                printer_ip: row.get(1)?,
                printer_serial: row.get(2)?,
                printer_access_code: row.get(3)?,
                default_ams: row.get(4)?,
                default_tray: row.get(5)?,
                auto_sync: row.get::<_, i32>(6)? == 1,
            })
        })?;

        Ok(settings)
    }

    pub fn save_settings(&self, settings: Settings) -> Result<()> {
        self.conn.execute(
            "UPDATE settings SET printer_name = ?1, printer_ip = ?2, printer_serial = ?3, printer_access_code = ?4, default_ams = ?5, default_tray = ?6, auto_sync = ?7 WHERE id = 1",
            params![
                settings.printer_name,
                settings.printer_ip,
                settings.printer_serial,
                settings.printer_access_code,
                settings.default_ams,
                settings.default_tray,
                settings.auto_sync as i32,
            ],
        )?;
        Ok(())
    }
}
