use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Manifest {
    #[serde(default)]
    pub product_list: Vec<ProductGroup>,
    #[serde(default)]
    pub firmware_list: Vec<Firmware>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProductGroup {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub product_id: Option<String>,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub image_url: String,
    #[serde(default)]
    pub mcu: Option<String>,
    #[serde(default)]
    pub github_repo: Option<String>,
    #[serde(default)]
    pub product_page: Option<String>,
    #[serde(default)]
    pub products: Option<Vec<Product>>,
    #[serde(default)]
    pub bin_files: Option<Vec<BinFile>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Product {
    pub product_id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub mcu: String,
    #[serde(default)]
    pub github_repo: String,
    #[serde(default)]
    pub product_page: String,
    #[serde(default)]
    pub image_url: String,
    #[serde(default)]
    pub bin_files: Option<Vec<BinFile>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BinFile {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub compressed_size: Option<u64>,
    #[serde(default)]
    pub oss_url: Option<String>,
    #[serde(default)]
    pub md5: Option<String>,
    #[serde(default)]
    pub sha256: Option<String>,
    #[serde(default)]
    pub release_tag: Option<String>,
    #[serde(default)]
    pub release_name: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub source_code_url: Option<String>,
    #[serde(default)]
    pub author_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Firmware {
    #[serde(default)]
    pub supported_product_ids: Vec<String>,
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default, rename = "type")]
    pub fw_type: String,
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub download_url: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub oss_url: Option<String>,
    #[serde(default)]
    pub md5: Option<String>,
    #[serde(default)]
    pub sha256: Option<String>,
    #[serde(default)]
    pub source_code_url: Option<String>,
}

/// A flattened product entry for display in the product list
#[derive(Debug, Clone)]
pub struct FlatProduct {
    pub product_id: String,
    pub name: String,
    pub description: String,
    pub mcu: String,
    pub series_name: Option<String>,
    pub github_repo: String,
    pub product_page: String,
    pub image_url: String,
    pub bin_files: Vec<BinFile>,
}

impl Manifest {
    /// Flatten the product_list hierarchy into a simple list of products
    pub fn flat_products(&self) -> Vec<FlatProduct> {
        let mut result = Vec::new();
        for group in &self.product_list {
            if let Some(products) = &group.products {
                for p in products {
                    result.push(FlatProduct {
                        product_id: p.product_id.clone(),
                        name: p.name.clone(),
                        description: p.description.clone(),
                        mcu: p.mcu.clone(),
                        series_name: Some(group.name.clone()),
                        github_repo: p.github_repo.clone(),
                        product_page: p.product_page.clone(),
                        image_url: p.image_url.clone(),
                        bin_files: p.bin_files.clone().unwrap_or_default(),
                    });
                }
            } else if let Some(pid) = &group.product_id {
                result.push(FlatProduct {
                    product_id: pid.clone(),
                    name: group.name.clone(),
                    description: group.description.clone(),
                    mcu: group.mcu.clone().unwrap_or_default(),
                    series_name: None,
                    github_repo: group.github_repo.clone().unwrap_or_default(),
                    product_page: group.product_page.clone().unwrap_or_default(),
                    image_url: group.image_url.clone(),
                    bin_files: group.bin_files.clone().unwrap_or_default(),
                });
            }
        }
        result
    }

    /// Get firmware list for a specific product (from firmware_list + bin_files)
    pub fn firmware_for_product(&self, product_id: &str) -> Vec<FirmwareItem> {
        let mut items = Vec::new();

        // From curated firmware_list
        for fw in &self.firmware_list {
            if fw.supported_product_ids.contains(&product_id.to_string()) {
                items.push(FirmwareItem {
                    name: fw.name.clone(),
                    version: fw.version.clone(),
                    fw_type: fw.fw_type.clone(),
                    filename: fw.filename.clone(),
                    download_url: fw.download_url.clone(),
                    description: fw.description.clone(),
                    size: fw.size,
                    oss_url: fw.oss_url.clone(),
                    md5: fw.md5.clone(),
                });
            }
        }

        // From product bin_files
        let flat = self.flat_products();
        if let Some(product) = flat.iter().find(|p| p.product_id == product_id) {
            for bf in &product.bin_files {
                items.push(FirmwareItem {
                    name: bf.release_name.clone().unwrap_or_else(|| bf.name.clone()),
                    version: bf.release_tag.clone().unwrap_or_else(|| "—".to_string()),
                    fw_type: "bin".to_string(),
                    filename: bf.name.clone(),
                    download_url: bf.url.clone(),
                    description: bf.path.clone().unwrap_or_default(),
                    size: bf.size,
                    oss_url: bf.oss_url.clone(),
                    md5: bf.md5.clone(),
                });
            }
        }

        items
    }
}

/// A unified firmware item for display
#[derive(Debug, Clone)]
pub struct FirmwareItem {
    pub name: String,
    pub version: String,
    pub fw_type: String,
    pub filename: String,
    pub download_url: String,
    pub description: String,
    pub size: Option<u64>,
    pub oss_url: Option<String>,
    pub md5: Option<String>,
}

impl FirmwareItem {
    pub fn size_display(&self) -> String {
        match self.size {
            Some(bytes) if bytes >= 1024 * 1024 => format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0)),
            Some(bytes) if bytes >= 1024 => format!("{:.1} KB", bytes as f64 / 1024.0),
            Some(bytes) => format!("{} B", bytes),
            None => "—".to_string(),
        }
    }
}

/// Load manifest from the local LILYGO-Spark project (for development)
pub fn load_manifest_from_file(path: &str) -> Result<Manifest, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("Failed to read manifest: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse manifest: {}", e))
}
