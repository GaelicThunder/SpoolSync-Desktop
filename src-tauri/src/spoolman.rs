use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpoolmanFilament {
    pub id: String,
    pub manufacturer: String,
    pub name: String,
    pub material: String,
    pub density: f64,
    pub diameter: f64,
    #[serde(default)]
    pub color_hex: Option<String>,
    #[serde(default)]
    pub color_hexes: Option<Vec<String>>,
    #[serde(default)]
    pub weight: Option<f64>,
    #[serde(default)]
    pub spool_weight: Option<f64>,
    #[serde(default)]
    pub spool_type: Option<String>,
    #[serde(rename = "extruder_temp", default)]
    pub extruder_temp: Option<i32>,
    #[serde(rename = "extruder_temp_range", default)]
    pub extruder_temp_range: Option<Vec<i32>>,
    #[serde(rename = "bed_temp", default)]
    pub bed_temp: Option<i32>,
    #[serde(rename = "bed_temp_range", default)]
    pub bed_temp_range: Option<Vec<i32>>,
    #[serde(default)]
    pub finish: Option<String>,
    #[serde(default)]
    pub multi_color_direction: Option<String>,
    #[serde(default)]
    pub pattern: Option<String>,
    #[serde(default)]
    pub translucent: bool,
    #[serde(default)]
    pub glow: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpoolmanResponse {
    pub items: Vec<SpoolmanFilament>,
    pub total: usize,
}

pub struct SpoolmanClient {
    client: reqwest::Client,
    base_url: String,
    cache: Mutex<Option<Vec<SpoolmanFilament>>>,
}

impl SpoolmanClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://donkie.github.io/SpoolmanDB".to_string(),
            cache: Mutex::new(None),
        }
    }

    async fn ensure_cache(&self) -> Result<(), String> {
        let mut cache = self.cache.lock().unwrap();
        
        if cache.is_some() {
            println!("✅ Using cached SpoolmanDB data");
            return Ok(());
        }

        drop(cache);

        let url = format!("{}/filaments.json", self.base_url);
        println!("📥 Downloading SpoolmanDB from: {}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch: {}", e))?;

        println!("✅ SpoolmanDB response status: {}", response.status());

        let filaments: Vec<SpoolmanFilament> = response
            .json()
            .await
            .map_err(|e| {
                eprintln!("JSON parse error: {:?}", e);
                format!("Failed to parse JSON: {}", e)
            })?;

        println!("✅ Cached {} filaments from SpoolmanDB", filaments.len());

        let mut cache = self.cache.lock().unwrap();
        *cache = Some(filaments);

        Ok(())
    }

    pub async fn search_filaments(
        &self,
        query: Option<String>,
        vendor: Option<String>,
        material: Option<String>,
        limit: usize,
        offset: usize,
    ) -> Result<SpoolmanResponse, String> {
        self.ensure_cache().await?;

        let cache = self.cache.lock().unwrap();
        let mut filaments = cache.as_ref().unwrap().clone();
        drop(cache);

        if let Some(q) = query {
            let q_lower = q.to_lowercase();
            filaments.retain(|f| {
                f.name.to_lowercase().contains(&q_lower)
                    || f.manufacturer.to_lowercase().contains(&q_lower)
                    || f.material.to_lowercase().contains(&q_lower)
            });
        }

        if let Some(v) = vendor {
            let v_lower = v.to_lowercase();
            filaments.retain(|f| f.manufacturer.to_lowercase() == v_lower);
        }

        if let Some(m) = material {
            let m_lower = m.to_lowercase();
            filaments.retain(|f| f.material.to_lowercase() == m_lower);
        }

        let total = filaments.len();
        let items = filaments
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect::<Vec<_>>();

        Ok(SpoolmanResponse { items, total })
    }

    pub async fn get_brands(&self) -> Result<Vec<String>, String> {
        self.ensure_cache().await?;

        let cache = self.cache.lock().unwrap();
        let filaments = cache.as_ref().unwrap();

        let mut brands: Vec<String> = filaments
            .iter()
            .map(|f| f.manufacturer.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        brands.sort();
        println!("✅ Loaded {} unique brands from cache", brands.len());

        Ok(brands)
    }

    pub async fn get_materials(&self) -> Result<Vec<String>, String> {
        self.ensure_cache().await?;

        let cache = self.cache.lock().unwrap();
        let filaments = cache.as_ref().unwrap();

        let materials_set: HashSet<String> = filaments
            .iter()
            .map(|f| f.material.clone())
            .collect();

        let mut materials: Vec<String> = materials_set.into_iter().collect();
        materials.sort();

        println!("✅ Loaded {} unique materials from cache", materials.len());

        Ok(materials)
    }

    pub async fn sync_database(&self) -> Result<(), String> {
        println!("🔄 Force syncing SpoolmanDB...");
        let mut cache = self.cache.lock().unwrap();
        *cache = None;
        drop(cache);
        
        self.ensure_cache().await?;
        println!("✅ SpoolmanDB synced successfully");
        Ok(())
    }
}
