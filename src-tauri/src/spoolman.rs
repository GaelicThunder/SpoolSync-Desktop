use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpoolmanFilament {
    pub id: u32,
    pub name: Option<String>,
    pub vendor: Option<SpoolmanVendor>,
    pub material: Option<String>,
    pub density: f64,
    pub diameter: f64,
    pub color_hex: Option<String>,
    pub spool_weight: Option<f64>,
    pub settings_extruder_temp: Option<i32>,
    pub settings_bed_temp: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpoolmanVendor {
    pub id: u32,
    pub name: String,
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

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch: {}", e))?;

        let mut filaments: Vec<SpoolmanFilament> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse: {}", e))?;

        if let Some(q) = query {
            let q_lower = q.to_lowercase();
            filaments.retain(|f| {
                f.name
                    .as_ref()
                    .map(|n| n.to_lowercase().contains(&q_lower))
                    .unwrap_or(false)
                    || f.vendor
                        .as_ref()
                        .map(|v| v.name.to_lowercase().contains(&q_lower))
                        .unwrap_or(false)
                    || f.material
                        .as_ref()
                        .map(|m| m.to_lowercase().contains(&q_lower))
                        .unwrap_or(false)
            });
        }

        if let Some(v) = vendor {
            let v_lower = v.to_lowercase();
            filaments.retain(|f| {
                f.vendor
                    .as_ref()
                    .map(|vendor| vendor.name.to_lowercase() == v_lower)
                    .unwrap_or(false)
            });
        }

        if let Some(m) = material {
            let m_lower = m.to_lowercase();
            filaments.retain(|f| {
                f.material
                    .as_ref()
                    .map(|mat| mat.to_lowercase() == m_lower)
                    .unwrap_or(false)
            });
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

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch: {}", e))?;

        let filaments: Vec<SpoolmanFilament> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse: {}", e))?;

        let mut brands: Vec<String> = filaments
            .into_iter()
            .filter_map(|f| f.vendor.map(|v| v.name))
            .collect();

        brands.sort();
        brands.dedup();

        Ok(brands)
    }
}
