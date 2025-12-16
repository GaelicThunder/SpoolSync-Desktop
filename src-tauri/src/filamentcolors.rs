use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilamentColorSwatch {
    pub id: i32,
    pub color_name: String,
    pub hex_color: String,
    pub manufacturer: FilamentManufacturer,
    #[serde(rename = "filament_type")]
    pub material: FilamentMaterial,
    pub image_front: Option<String>,
    pub image_back: Option<String>,
    pub image_other: Option<String>,
    pub notes: Option<String>,
    pub amazon_purchase_link: Option<String>,
    pub manufacturer_purchase_link: Option<String>,
    pub published: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilamentManufacturer {
    pub id: i32,
    pub name: String,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilamentMaterial {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilamentColorsResponse {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<FilamentColorSwatch>,
}

pub struct FilamentColorsClient {
    client: reqwest::Client,
    base_url: String,
}

impl FilamentColorsClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://filamentcolors.xyz".to_string(),
        }
    }

    pub async fn get_swatches(
        &self,
        manufacturer: Option<String>,
        material_type: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<FilamentColorsResponse, String> {
        let mut url = format!("{}/api/swatch/", self.base_url);
        let mut params = vec![];

        if let Some(mfr) = manufacturer {
            params.push(format!("manufacturer__name={}", urlencoding::encode(&mfr)));
        }
        if let Some(mat) = material_type {
            params.push(format!("filament_type__name={}", urlencoding::encode(&mat)));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }
        if let Some(off) = offset {
            params.push(format!("offset={}", off));
        }

        if !params.is_empty() {
            url.push_str("?");
            url.push_str(&params.join("&"));
        }

        println!("📸 Fetching FilamentColors.xyz: {}", url);

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "SpoolSync/1.0")
            .send()
            .await
            .map_err(|e| format!("Failed to fetch: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {} - {}", response.status(), response.text().await.unwrap_or_default()));
        }

        let data: FilamentColorsResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        println!("✅ Loaded {} swatches (total: {})", data.results.len(), data.count);

        Ok(data)
    }
}
