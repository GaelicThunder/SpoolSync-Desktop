use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BambuFilamentProfile {
    pub name: String,
    pub from: Option<String>,
    pub instantiation: Option<String>,
    pub inherits: Option<String>,
    
    pub filament_type: Vec<String>,
    pub filament_vendor: Vec<String>,
    pub filament_colour: Vec<String>,
    
    pub nozzle_temperature: Vec<String>,
    pub nozzle_temperature_range_low: Vec<String>,
    pub nozzle_temperature_range_high: Vec<String>,
    
    pub hot_plate_temp: Vec<String>,
    pub hot_plate_temp_initial_layer: Vec<String>,
    
    pub filament_flow_ratio: Vec<String>,
    pub filament_density: Vec<String>,
    pub filament_diameter: Vec<String>,
    pub filament_cost: Vec<String>,
    
    pub compatible_printers: Vec<String>,
    pub compatible_printers_condition: Vec<String>,
    pub compatible_prints: Vec<String>,
    pub compatible_prints_condition: Vec<String>,
}

impl Default for BambuFilamentProfile {
    fn default() -> Self {
        Self {
            name: String::new(),
            from: Some("User".to_string()),
            instantiation: Some("true".to_string()),
            inherits: Some("Bambu PLA Basic @BBL X1C".to_string()),
            
            filament_type: vec!["PLA".to_string()],
            filament_vendor: vec!["Generic".to_string()],
            filament_colour: vec!["#FFFFFF".to_string()],
            
            nozzle_temperature: vec!["220".to_string()],
            nozzle_temperature_range_low: vec!["190".to_string()],
            nozzle_temperature_range_high: vec!["250".to_string()],
            
            hot_plate_temp: vec!["55".to_string()],
            hot_plate_temp_initial_layer: vec!["55".to_string()],
            
            filament_flow_ratio: vec!["1".to_string()],
            filament_density: vec!["1.24".to_string()],
            filament_diameter: vec!["1.75".to_string()],
            filament_cost: vec!["0".to_string()],
            
            compatible_printers: vec![],
            compatible_printers_condition: vec![],
            compatible_prints: vec![],
            compatible_prints_condition: vec![],
        }
    }
}

pub struct BambuStudioManager {
    config_dir: PathBuf,
}

impl BambuStudioManager {
    pub fn new() -> Result<Self, String> {
        let home = std::env::var("HOME")
            .map_err(|_| "Could not determine HOME directory".to_string())?;
        
        let config_dir = PathBuf::from(home)
            .join(".config/BambuStudio/user/default/filament");
        
        if !config_dir.exists() {
            return Err(format!(
                "Bambu Studio config directory not found: {}\nMake sure Bambu Studio is installed.",
                config_dir.display()
            ));
        }
        
        Ok(Self { config_dir })
    }
    
    pub fn list_profiles(&self) -> Result<Vec<String>, String> {
        let mut profiles = Vec::new();
        
        let entries = fs::read_dir(&self.config_dir)
            .map_err(|e| format!("Failed to read config directory: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    profiles.push(name.to_string());
                }
            }
        }
        
        Ok(profiles)
    }
    
    pub fn read_profile(&self, name: &str) -> Result<BambuFilamentProfile, String> {
        let path = self.config_dir.join(format!("{}.json", name));
        
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read profile {}: {}", name, e))?;
        
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse profile {}: {}", name, e))
    }
    
    pub fn create_profile(
        &self,
        profile: &BambuFilamentProfile,
    ) -> Result<String, String> {
        let filename = self.sanitize_filename(&profile.name);
        let json_path = self.config_dir.join(format!("{}.json", filename));
        let info_path = self.config_dir.join(format!("{}.json.info", filename));
        
        if json_path.exists() {
            return Err(format!("Profile '{}' already exists", profile.name));
        }
        
        let json_content = serde_json::to_string_pretty(profile)
            .map_err(|e| format!("Failed to serialize profile: {}", e))?;
        
        fs::write(&json_path, json_content)
            .map_err(|e| format!("Failed to write profile JSON: {}", e))?;
        
        let info_content = format!("name = {}\nfrom = User\n", profile.name);
        fs::write(&info_path, info_content)
            .map_err(|e| format!("Failed to write .info file: {}", e))?;
        
        println!("✅ Created Bambu Studio profile: {}", json_path.display());
        Ok(filename)
    }
    
    pub fn update_profile(
        &self,
        name: &str,
        profile: &BambuFilamentProfile,
    ) -> Result<(), String> {
        let filename = self.sanitize_filename(name);
        let json_path = self.config_dir.join(format!("{}.json", filename));
        
        if !json_path.exists() {
            return Err(format!("Profile '{}' does not exist", name));
        }
        
        let json_content = serde_json::to_string_pretty(profile)
            .map_err(|e| format!("Failed to serialize profile: {}", e))?;
        
        fs::write(&json_path, json_content)
            .map_err(|e| format!("Failed to update profile: {}", e))?;
        
        println!("✅ Updated Bambu Studio profile: {}", name);
        Ok(())
    }
    
    pub fn delete_profile(&self, name: &str) -> Result<(), String> {
        let filename = self.sanitize_filename(name);
        let json_path = self.config_dir.join(format!("{}.json", filename));
        let info_path = self.config_dir.join(format!("{}.json.info", filename));
        
        if json_path.exists() {
            fs::remove_file(&json_path)
                .map_err(|e| format!("Failed to delete JSON: {}", e))?;
        }
        
        if info_path.exists() {
            fs::remove_file(&info_path)
                .map_err(|e| format!("Failed to delete .info: {}", e))?;
        }
        
        println!("✅ Deleted Bambu Studio profile: {}", name);
        Ok(())
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
        
        let mut profile = BambuFilamentProfile::default();
        profile.name = profile_name.clone();
        profile.filament_vendor = vec![vendor.to_string()];
        profile.filament_type = vec![material.to_string()];
        profile.filament_colour = vec![color_hex.to_string()];
        
        profile.nozzle_temperature = vec![nozzle_temp.to_string()];
        profile.nozzle_temperature_range_low = vec![(nozzle_temp.saturating_sub(30)).to_string()];
        profile.nozzle_temperature_range_high = vec![(nozzle_temp + 30).to_string()];
        
        profile.hot_plate_temp = vec![bed_temp.to_string()];
        profile.hot_plate_temp_initial_layer = vec![bed_temp.to_string()];
        
        profile.filament_density = vec![density.to_string()];
        
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
}
