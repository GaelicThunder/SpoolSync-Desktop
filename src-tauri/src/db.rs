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

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub printer_ip: String,
    pub printer_serial: String,
    pub access_code: String,
    pub default_ams: i32,
    pub default_tray: i32,
    pub auto_sync: bool,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path();
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS profiles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                brand TEXT NOT NULL,
                material TEXT NOT NULL,
                color TEXT NOT NULL,
                nozzle_temp INTEGER NOT NULL,
                bed_temp INTEGER NOT NULL,
                density REAL NOT NULL,
                diameter REAL NOT NULL,
                is_favorite INTEGER NOT NULL DEFAULT 0,
                is_custom INTEGER NOT NULL DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                printer_ip TEXT NOT NULL DEFAULT '',
                printer_serial TEXT NOT NULL DEFAULT '',
                access_code TEXT NOT NULL DEFAULT '',
                default_ams INTEGER NOT NULL DEFAULT 0,
                default_tray INTEGER NOT NULL DEFAULT 0,
                auto_sync INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;

        conn.execute(
            "INSERT OR IGNORE INTO settings (id) VALUES (1)",
            [],
        )?;

        Ok(Database { conn })
    }

    fn get_db_path() -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("SpoolSync");
        std::fs::create_dir_all(&path).ok();
        path.push("spoolsync.db");
        path
    }

    pub fn get_favorites(&self) -> Result<Vec<FilamentProfile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, brand, material, color, nozzle_temp, bed_temp, density, diameter, is_favorite, is_custom 
             FROM profiles WHERE is_favorite = 1 ORDER BY created_at DESC",
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
             FROM profiles WHERE is_custom = 1 ORDER BY created_at DESC",
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

    pub fn add_favorite(&self, mut profile: FilamentProfile) -> Result<i64> {
        profile.is_favorite = true;
        self.conn.execute(
            "INSERT INTO profiles (brand, material, color, nozzle_temp, bed_temp, density, diameter, is_favorite, is_custom)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1, ?8)",
            params![
                profile.brand,
                profile.material,
                profile.color,
                profile.nozzle_temp,
                profile.bed_temp,
                profile.density,
                profile.diameter,
                if profile.is_custom { 1 } else { 0 }
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn remove_favorite(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE profiles SET is_favorite = 0 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn create_custom_profile(&self, mut profile: FilamentProfile) -> Result<i64> {
        profile.is_custom = true;
        profile.is_favorite = true;
        self.conn.execute(
            "INSERT INTO profiles (brand, material, color, nozzle_temp, bed_temp, density, diameter, is_favorite, is_custom)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1, 1)",
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
            "UPDATE profiles SET brand = ?1, material = ?2, color = ?3, nozzle_temp = ?4, bed_temp = ?5, density = ?6, diameter = ?7
             WHERE id = ?8 AND is_custom = 1",
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
        self.conn.execute(
            "DELETE FROM profiles WHERE id = ?1 AND is_custom = 1",
            params![id],
        )?;
        Ok(())
    }

    pub fn get_settings(&self) -> Result<Settings> {
        let mut stmt = self.conn.prepare(
            "SELECT printer_ip, printer_serial, access_code, default_ams, default_tray, auto_sync FROM settings WHERE id = 1",
        )?;

        let settings = stmt.query_row([], |row| {
            Ok(Settings {
                printer_ip: row.get(0)?,
                printer_serial: row.get(1)?,
                access_code: row.get(2)?,
                default_ams: row.get(3)?,
                default_tray: row.get(4)?,
                auto_sync: row.get::<_, i32>(5)? == 1,
            })
        })?;

        Ok(settings)
    }

    pub fn save_settings(&self, settings: Settings) -> Result<()> {
        self.conn.execute(
            "UPDATE settings SET printer_ip = ?1, printer_serial = ?2, access_code = ?3, default_ams = ?4, default_tray = ?5, auto_sync = ?6 WHERE id = 1",
            params![
                settings.printer_ip,
                settings.printer_serial,
                settings.access_code,
                settings.default_ams,
                settings.default_tray,
                if settings.auto_sync { 1 } else { 0 }
            ],
        )?;
        Ok(())
    }
}
