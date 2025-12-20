use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BambuFilamentProfile {
    pub version: String,
    pub from: String,
    #[serde(rename = "type")]
    pub profile_type: String,
    pub name: String,
    pub instantiation: String,
    pub filament_id: Vec<String>,
    pub setting_id: String,
    pub inherits: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_vendor: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_type: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_colour: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nozzle_temperature: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_plate_temp: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_plate_temp_initial_layer: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chamber_temperature: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_flow_ratio: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_max_volumetric_speed: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fan_cooling_layer_time: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fan_max_speed: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fan_min_speed: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_down_for_layer_cooling: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_retraction_length: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_retraction_speed: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_deretraction_speed: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_z_hop: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_z_hop_types: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_printers: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_prints_condition: Option<Vec<String>>,
}

impl BambuFilamentProfile {
    pub fn new_for_material(material: &str, _printer: &str) -> Self {
        let (inherits, filament_id_prefix) = match material {
            "PLA" => ("Bambu PLA Basic @BBL X1C", "GFL"),
            "PETG" => ("Bambu PETG Basic @BBL X1C", "GFB"),
            "ABS" => ("Bambu ABS @BBL X1C", "GFB"),
            "TPU" => ("Bambu TPU 95A @BBL X1C", "GFU"),
            "ASA" => ("Bambu ASA @BBL X1C", "GFB"),
            "PA" | "NYLON" => ("Bambu PA-CF @BBL X1C", "GFN"),
            "PC" => ("Bambu PC @BBL X1C", "GFC"),
            "PVA" => ("Bambu PVA Support @BBL X1C", "GFS"),
            "PLA-CF" => ("Bambu PLA-CF @BBL X1C", "GFL"),
            _ => ("Bambu PLA Basic @BBL X1C", "GFL"),
        };
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let filament_id = format!("{}US{:x}", filament_id_prefix, timestamp % 0xFFFFFF);
        let setting_id = format!("GFSUS{:x}", timestamp % 0xFFFFFFFFFFFF);
        
        Self {
            version: "1.8.0.50".to_string(),
            from: "User".to_string(),
            profile_type: "filament".to_string(),
            name: String::new(),
            instantiation: "true".to_string(),
            filament_id: vec![filament_id],
            setting_id,
            inherits: inherits.to_string(),
            filament_vendor: None,
            filament_type: Some(vec![material.to_string()]),
            filament_colour: None,
            nozzle_temperature: None,
            hot_plate_temp: None,
            hot_plate_temp_initial_layer: None,
            chamber_temperature: None,
            filament_flow_ratio: Some(vec!["1".to_string()]),
            filament_max_volumetric_speed: None,
            fan_cooling_layer_time: None,
            fan_max_speed: None,
            fan_min_speed: None,
            slow_down_for_layer_cooling: Some(vec!["1".to_string()]),
            filament_retraction_length: None,
            filament_retraction_speed: None,
            filament_deretraction_speed: None,
            filament_z_hop: None,
            filament_z_hop_types: None,
            compatible_printers: None,
            compatible_prints_condition: None,
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
                if !filament_dir.exists() {
                    fs::create_dir_all(&filament_dir)
                        .map_err(|e| format!("Failed to create filament dir: {}", e))?;
                }
                user_dirs.push(filament_dir);
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
    
    pub fn create_from_spoolman(
        &self,
        vendor: &str,
        name: &str,
        material: &str,
        color_hex: &str,
        nozzle_temp: u16,
        bed_temp: u16,
        printer: &str,
    ) -> Result<String, String> {
        let profile_name = format!("{} {} {} @{}", vendor, material, name, printer);
        
        let mut profile = BambuFilamentProfile::new_for_material(material, printer);
        profile.name = profile_name.clone();
        profile.filament_vendor = Some(vec![vendor.to_string()]);
        profile.filament_colour = Some(vec![color_hex.trim_start_matches('#').to_uppercase()]);
        profile.nozzle_temperature = Some(vec![nozzle_temp.to_string()]);
        profile.hot_plate_temp = Some(vec![bed_temp.to_string()]);
        profile.hot_plate_temp_initial_layer = Some(vec![bed_temp.to_string()]);
        
        self.create_profile(&profile)
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
            println!("⚠️  Profile '{}' already exists, updating...", profile.name);
        }
        
        let json_content = serde_json::to_string_pretty(profile)
            .map_err(|e| format!("Failed to serialize profile: {}", e))?;
        
        fs::write(&json_path, json_content)
            .map_err(|e| format!("Failed to write profile JSON: {}", e))?;
        
        let user_id = self.extract_user_id(user_dir);
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let info_content = format!(
            "sync_info = create\nuser_id = {}\nsetting_id = {}\nbase_id = {}\nupdated_time = {}\n",
            user_id, 
            profile.setting_id,
            profile.filament_id[0],
            timestamp
        );
        
        fs::write(&info_path, info_content.clone())
            .map_err(|e| format!("Failed to write .info file: {}", e))?;
        
        println!("\n✅ Profilo creato: {}", json_path.display());
        println!("   File ID: {}", profile.filament_id[0]);
        println!("   Setting ID: {}", profile.setting_id);
        println!("   Inherits: {}", profile.inherits);
        println!("\n✅ .info file contents:");
        for (i, line) in info_content.lines().enumerate() {
            println!("   [{}] {}", i, line);
        }
        println!("\n✅ Bambu Studio dovrebbe vederlo automaticamente!");
        println!("   Se non lo vede, riavvia Bambu Studio.\n");
        
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
                
                let info_path = user_dir.join(format!("{}.info", filename));
                let user_id = self.extract_user_id(user_dir);
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                let info_content = format!(
                    "sync_info = create\nuser_id = {}\nsetting_id = {}\nbase_id = {}\nupdated_time = {}\n",
                    user_id, 
                    profile.setting_id,
                    profile.filament_id[0],
                    timestamp
                );
                
                fs::write(&info_path, info_content)
                    .map_err(|e| format!("Failed to update .info file: {}", e))?;
                
                println!("✅ Profilo aggiornato: {}", name);
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
            println!("✅ Profilo eliminato: {}", name);
            Ok(())
        } else {
            Err(format!("Profile '{}' not found", name))
        }
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
}
