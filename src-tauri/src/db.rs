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
        
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        let conn = Connection::open(&db_path)?;
        let db = Database { conn };
        db.create_tables()?;
        db.seed_initial_data()?;
        Ok(db)
    }

    fn get_db_path() -> PathBuf {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("SpoolSync");
        data_dir.join("spoolsync.db")
    }

    fn create_tables(&self) -> Result<()> {
        self.conn.execute(
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
                is_custom INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;

        self.conn.execute(
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

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS brands (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS materials (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            )",
            [],
        )?;

        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM settings WHERE id = 1",
            [],
            |row| row.get(0),
        )?;

        if count == 0 {
            self.conn.execute(
                "INSERT INTO settings (id) VALUES (1)",
                [],
            )?;
        }

        Ok(())
    }

    fn seed_initial_data(&self) -> Result<()> {
        let brands = vec![
            "Bambu Lab", "Polymaker", "Prusament", "eSUN", "ColorFabb",
            "Fillamentum", "Hatchbox", "Overture", "Sunlu", "3DJake"
        ];

        for brand in brands {
            self.conn.execute(
                "INSERT OR IGNORE INTO brands (name) VALUES (?1)",
                params![brand],
            ).ok();
        }

        let materials = vec![
            "PLA", "PETG", "ABS", "TPU", "TPE", "ASA", "Nylon", "PC",
            "PLA+", "PETG+", "Carbon Fiber", "Wood Fill", "Silk"
        ];

        for material in materials {
            self.conn.execute(
                "INSERT OR IGNORE INTO materials (name) VALUES (?1)",
                params![material],
            ).ok();
        }

        Ok(())
    }

    pub fn get_brands(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT name FROM brands ORDER BY name")?;
        let brands = stmt.query_map([], |row| row.get(0))?;
        brands.collect()
    }

    pub fn get_materials(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT name FROM materials ORDER BY name")?;
        let materials = stmt.query_map([], |row| row.get(0))?;
        materials.collect()
    }

    pub fn add_brand(&self, name: String) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO brands (name) VALUES (?1)",
            params![name],
        )?;
        Ok(())
    }

    pub fn add_material(&self, name: String) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO materials (name) VALUES (?1)",
            params![name],
        )?;
        Ok(())
    }

    pub fn get_favorites(&self) -> Result<Vec<FilamentProfile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, brand, material, color, nozzle_temp, bed_temp, density, diameter, is_favorite, is_custom 
             FROM profiles 
             WHERE is_favorite = 1
             ORDER BY id DESC",
        )?;

        let profiles = stmt.query_map([], |row| {
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
        })?;

        profiles.collect()
    }

    pub fn get_custom_profiles(&self) -> Result<Vec<FilamentProfile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, brand, material, color, nozzle_temp, bed_temp, density, diameter, is_favorite, is_custom 
             FROM profiles 
             WHERE is_custom = 1
             ORDER BY id DESC",
        )?;

        let profiles = stmt.query_map([], |row| {
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
        })?;

        profiles.collect()
    }

    pub fn add_favorite(&self, mut profile: FilamentProfile) -> Result<i64> {
        profile.is_favorite = true;
        self.conn.execute(
            "INSERT INTO profiles (brand, material, color, nozzle_temp, bed_temp, density, diameter, is_favorite, is_custom)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                profile.brand,
                profile.material,
                profile.color,
                profile.nozzle_temp,
                profile.bed_temp,
                profile.density,
                profile.diameter,
                if profile.is_favorite { 1 } else { 0 },
                if profile.is_custom { 1 } else { 0 },
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn remove_favorite(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM profiles WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn create_custom_profile(&self, mut profile: FilamentProfile) -> Result<i64> {
        profile.is_custom = true;
        profile.is_favorite = true;
        
        self.add_brand(profile.brand.clone())?;
        self.add_material(profile.material.clone())?;
        
        self.add_favorite(profile)
    }

    pub fn update_custom_profile(&self, profile: FilamentProfile) -> Result<()> {
        if let Some(id) = profile.id {
            self.conn.execute(
                "UPDATE profiles 
                 SET brand = ?1, material = ?2, color = ?3, nozzle_temp = ?4, bed_temp = ?5, density = ?6, diameter = ?7
                 WHERE id = ?8",
                params![
                    profile.brand,
                    profile.material,
                    profile.color,
                    profile.nozzle_temp,
                    profile.bed_temp,
                    profile.density,
                    profile.diameter,
                    id,
                ],
            )?;
        }
        Ok(())
    }

    pub fn delete_custom_profile(&self, id: i64) -> Result<()> {
        self.remove_favorite(id)
    }

    pub fn get_settings(&self) -> Result<Settings> {
        self.conn.query_row(
            "SELECT printer_ip, printer_serial, access_code, default_ams, default_tray, auto_sync 
             FROM settings WHERE id = 1",
            [],
            |row| {
                Ok(Settings {
                    printer_ip: row.get(0)?,
                    printer_serial: row.get(1)?,
                    access_code: row.get(2)?,
                    default_ams: row.get(3)?,
                    default_tray: row.get(4)?,
                    auto_sync: row.get::<_, i32>(5)? == 1,
                })
            },
        )
    }

    pub fn save_settings(&self, settings: Settings) -> Result<()> {
        self.conn.execute(
            "UPDATE settings 
             SET printer_ip = ?1, printer_serial = ?2, access_code = ?3, default_ams = ?4, default_tray = ?5, auto_sync = ?6
             WHERE id = 1",
            params![
                settings.printer_ip,
                settings.printer_serial,
                settings.access_code,
                settings.default_ams,
                settings.default_tray,
                if settings.auto_sync { 1 } else { 0 },
            ],
        )?;
        Ok(())
    }
}
