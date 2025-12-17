use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BambuFilamentProfile {
    pub name: String,
    pub from: Option<String>,
    pub inherits: Option<String>,
    pub version: Option<String>,
    
    pub filament_settings_id: Option<Vec<String>>,
    pub filament_extruder_variant: Option<Vec<String>>,
    
    pub nozzle_temperature: Option<Vec<String>>,
    pub nozzle_temperature_initial_layer: Option<Vec<String>>,
    
    pub filament_max_volumetric_speed: Option<Vec<String>>,
    pub slow_down_for_layer_cooling: Option<Vec<String>>,
}

impl Default for BambuFilamentProfile {
    fn default() -> Self {
        Self {
            name: String::new(),
            from: Some("User".to_string()),
            inherits: Some("Bambu PLA Basic @BBL X1C".to_string()),
            version: Some("1.8.0.13".to_string()),
            filament_settings_id: None,
            filament_extruder_variant: Some(vec!["Direct Drive Standard".to_string()]),
            nozzle_temperature: Some(vec!["220".to_string()]),
            nozzle_temperature_initial_layer: Some(vec!["220".to_string()]),
            filament_max_volumetric_speed: Some(vec!["20".to_string()]),
            slow_down_for_layer_cooling: Some(vec!["0".to_string()]),
        }
    }
}

pub struct BambuStudioManager {
    base_dir: PathBuf,
    user_dirs: Vec<PathBuf>,
}

impl BambuStudioManager {
    pub fn new() -> Result<Self, String> {
        let home = std::env::var("HOME")
            .map_err(|_| "Could not determine HOME directory".to_string())?;
        
        let base_dir = PathBuf::from(home).join(".config/BambuStudio/user");
        
        if !base_dir.exists() {
            return Err(format!(
                "Bambu Studio config directory not found: {}\nMake sure Bambu Studio is installed.",
                base_dir.display()
            ));
        }
        
        let mut user_dirs = Vec::new();
        
        for entry in fs::read_dir(&base_dir)
            .map_err(|e| format!("Failed to read user directory: {}", e))? 
        {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                let filament_dir = path.join("filament");
                if filament_dir.exists() {
                    user_dirs.push(filament_dir);
                }
            }
        }
        
        if user_dirs.is_empty() {
            return Err("No user profiles found in Bambu Studio".to_string());
        }
        
        println!("✅ Found {} Bambu Studio user profile(s)", user_dirs.len());
        
        Ok(Self { base_dir, user_dirs })
    }
    
    pub fn list_profiles(&self) -> Result<Vec<String>, String> {
        let mut profiles = Vec::new();
        
        for user_dir in &self.user_dirs {
            let entries = match fs::read_dir(user_dir) {
                Ok(e) => e,
                Err(_) => continue,
            };
            
            for entry in entries {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                let path = entry.path();
                
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        if !profiles.contains(&name.to_string()) {
                            profiles.push(name.to_string());
                        }
                    }
                }
            }
        }
        
        Ok(profiles)
    }
    
    pub fn read_profile(&self, name: &str) -> Result<BambuFilamentProfile, String> {
        for user_dir in &self.user_dirs {
            let path = user_dir.join(format!("{}.json", name));
            
            if path.exists() {
                let content = fs::read_to_string(&path)
                    .map_err(|e| format!("Failed to read profile {}: {}", name, e))?;
                
                return serde_json::from_str(&content)
                    .map_err(|e| format!("Failed to parse profile {}: {}", name, e));
            }
        }
        
        Err(format!("Profile '{}' not found", name))
    }
    
    pub fn create_profile(&self, profile: &BambuFilamentProfile) -> Result<String, String> {
        if self.user_dirs.is_empty() {
            return Err("No user directories found".to_string());
        }
        
        let user_dir = &self.user_dirs[0];
        
        let filename = self.sanitize_filename(&profile.name);
        let json_path = user_dir.join(format!("{}.json", filename));
        let info_path = user_dir.join(format!("{}.info", filename));
        
        if json_path.exists() {
            return Err(format!("Profile '{}' already exists", profile.name));
        }
        
        let json_content = serde_json::to_string_pretty(profile)
            .map_err(|e| format!("Failed to serialize profile: {}", e))?;
        
        fs::write(&json_path, json_content)
            .map_err(|e| format!("Failed to write profile JSON: {}", e))?;
        
        let user_id = self.extract_user_id(user_dir);
        let setting_id = self.generate_setting_id();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let info_content = format!(
            "sync_info =\n\nuser_id = {}\n\nsetting_id = {}\n\nbase_id = GFSG00_06\n\nupdated_time = {}\n",
            user_id, setting_id, timestamp
        );
        
        fs::write(&info_path, info_content)
            .map_err(|e| format!("Failed to write .info file: {}", e))?;
        
        println!("✅ Created Bambu Studio profile: {}", json_path.display());
        Ok(filename)
    }
    
    pub fn update_profile(&self, name: &str, profile: &BambuFilamentProfile) -> Result<(), String> {
        for user_dir in &self.user_dirs {
            let filename = self.sanitize_filename(name);
            let json_path = user_dir.join(format!("{}.json", filename));
            
            if json_path.exists() {
                let json_content = serde_json::to_string_pretty(profile)
                    .map_err(|e| format!("Failed to serialize profile: {}", e))?;
                
                fs::write(&json_path, json_content)
                    .map_err(|e| format!("Failed to update profile: {}", e))?;
                
                println!("✅ Updated Bambu Studio profile: {}", name);
                return Ok(());
            }
        }
        
        Err(format!("Profile '{}' not found", name))
    }
    
    pub fn delete_profile(&self, name: &str) -> Result<(), String> {
        let filename = self.sanitize_filename(name);
        let mut deleted = false;
        
        for user_dir in &self.user_dirs {
            let json_path = user_dir.join(format!("{}.json", filename));
            let info_path = user_dir.join(format!("{}.info", filename));
            
            if json_path.exists() {
                fs::remove_file(&json_path)
                    .map_err(|e| format!("Failed to delete JSON: {}", e))?;
                deleted = true;
            }
            
            if info_path.exists() {
                fs::remove_file(&info_path)
                    .map_err(|e| format!("Failed to delete .info: {}", e))?;
            }
        }
        
        if deleted {
            println!("✅ Deleted Bambu Studio profile: {}", name);
            Ok(())
        } else {
            Err(format!("Profile '{}' not found", name))
        }
    }
    
    pub fn create_from_spoolman(
        &self,
        vendor: &str,
        name: &str,
        material: &str,
        color_hex: &str,
        nozzle_temp: u16,
        bed_temp: u16,
        density: f32,
    ) -> Result<String, String> {
        let profile_name = format!("{} {} {}", vendor, material, name);
        let setting_id = format!("{} {} {}", vendor, material, name);
        
        let mut profile = BambuFilamentProfile::default();
        profile.name = profile_name.clone();
        profile.filament_settings_id = Some(vec![setting_id]);
        
        profile.nozzle_temperature = Some(vec![nozzle_temp.to_string()]);
        profile.nozzle_temperature_initial_layer = Some(vec![nozzle_temp.to_string()]);
        
        let inherit = match material {
            "PLA" => "Bambu PLA Basic @BBL X1C",
            "PETG" => "Bambu PETG Basic @BBL X1C",
            "ABS" => "Bambu ABS @BBL X1C",
            "TPU" => "Bambu TPU 95A @BBL X1C",
            "ASA" => "Bambu ASA @BBL X1C",
            "PVA" => "Bambu PVA Support @BBL X1C",
            _ => "Bambu PLA Basic @BBL X1C",
        };
        profile.inherits = Some(inherit.to_string());
        
        self.create_profile(&profile)
    }
    
    fn sanitize_filename(&self, name: &str) -> String {
        name.chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                _ => c,
            })
            .collect()
    }
    
    fn extract_user_id(&self, user_dir: &PathBuf) -> String {
        user_dir
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("default")
            .to_string()
    }
    
    fn generate_setting_id(&self) -> String {
        use std::time::SystemTime;
        let timestamp = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("PFUS{:x}", timestamp % 0xFFFFFFFFFFFF)
    }
}
