use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
    pub spool_weight: Option<u32>,
    #[serde(default)]
    pub spool_type: Option<String>,
    #[serde(rename = "extruder_temp", default)]
    pub settings_extruder_temp: Option<i32>,
    #[serde(rename = "extruder_temp_range", default)]
    pub extruder_temp_range: Option<Vec<i32>>,
    #[serde(rename = "bed_temp", default)]
    pub settings_bed_temp: Option<i32>,
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
}

impl SpoolmanClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://donkie.github.io/SpoolmanDB".to_string(),
        }
    }

    pub async fn search_filaments(
        &self,
        query: Option<String>,
        vendor: Option<String>,
        material: Option<String>,
        limit: usize,
        offset: usize,
    ) -> Result<SpoolmanResponse, String> {
        let url = format!("{}/filaments.json", self.base_url);
        println!("Fetching SpoolmanDB from: {}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch: {}", e))?;

        println!("SpoolmanDB response status: {}", response.status());

        let mut filaments: Vec<SpoolmanFilament> = response
            .json()
            .await
            .map_err(|e| {
                eprintln!("JSON parse error details: {:?}", e);
                format!("Failed to parse JSON: {}", e)
            })?;

        println!("Loaded {} filaments from SpoolmanDB", filaments.len());

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
        let url = format!("{}/filaments.json", self.base_url);
        println!("Fetching brands from SpoolmanDB...");

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch: {}", e))?;

        let filaments: Vec<SpoolmanFilament> = response
            .json()
            .await
            .map_err(|e| {
                eprintln!("JSON parse error details: {:?}", e);
                format!("Failed to parse JSON: {}", e)
            })?;

        let mut brands: Vec<String> = filaments
            .into_iter()
            .map(|f| f.manufacturer)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        brands.sort();
        println!("Loaded {} unique brands", brands.len());

        Ok(brands)
    }

    pub async fn get_materials(&self) -> Result<Vec<String>, String> {
        let url = format!("{}/filaments.json", self.base_url);
        println!("Fetching materials from SpoolmanDB...");

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch: {}", e))?;

        let filaments: Vec<SpoolmanFilament> = response
            .json()
            .await
            .map_err(|e| {
                eprintln!("JSON parse error details: {:?}", e);
                format!("Failed to parse JSON: {}", e)
            })?;

        let materials_set: HashSet<String> = filaments
            .into_iter()
            .map(|f| f.material)
            .collect();

        let mut materials: Vec<String> = materials_set.into_iter().collect();
        materials.sort();

        println!("Loaded {} unique materials from SpoolmanDB", materials.len());

        Ok(materials)
    }
}
