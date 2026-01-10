//! Asset management utilities for loading and caching textures

use macroquad::prelude::*;
use std::collections::HashMap;

/// Manages texture assets with caching
///
/// # Example
/// ```no_run
/// use macroquad_toolkit::assets::AssetManager;
///
/// # async fn example() {
/// let mut assets = AssetManager::new();
/// assets.load_texture("player", "assets/player.png").await.ok();
///
/// if let Some(texture) = assets.get_texture("player") {
///     // Use texture
/// }
/// # }
/// ```
pub struct AssetManager {
    textures: HashMap<String, Texture2D>,
}

impl AssetManager {
    /// Create a new empty asset manager
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// Load a single texture and add it to the cache
    ///
    /// The texture filter is automatically set to Nearest for pixel-perfect rendering.
    /// Returns an error if the texture fails to load.
    pub async fn load_texture(&mut self, name: &str, path: &str) -> Result<(), String> {
        match load_texture(path).await {
            Ok(texture) => {
                texture.set_filter(FilterMode::Nearest);
                self.textures.insert(name.to_string(), texture);
                Ok(())
            }
            Err(e) => Err(format!("Failed to load texture '{}': {:?}", path, e)),
        }
    }

    /// Load multiple textures at once
    ///
    /// Each tuple should be (name, path). Returns the number of successfully loaded textures.
    pub async fn load_textures(&mut self, assets: &[(String, String)]) -> usize {
        let mut loaded = 0;
        for (name, path) in assets {
            if self.load_texture(name, path).await.is_ok() {
                loaded += 1;
            }
        }
        loaded
    }

    /// Get a texture by name. Returns None if not found.
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }

    /// Check if a texture with the given name exists
    pub fn has_texture(&self, name: &str) -> bool {
        self.textures.contains_key(name)
    }

    /// Get the number of loaded textures
    pub fn len(&self) -> usize {
        self.textures.len()
    }

    /// Check if any textures are loaded
    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Load a texture with a specific filter mode
pub async fn load_texture_filtered(path: &str, filter: FilterMode) -> Result<Texture2D, String> {
    match load_texture(path).await {
        Ok(texture) => {
            texture.set_filter(filter);
            Ok(texture)
        }
        Err(e) => Err(format!("Failed to load texture '{}': {:?}", path, e)),
    }
}

/// Load a texture with Nearest filter (pixel-perfect)
pub async fn load_texture_nearest(path: &str) -> Result<Texture2D, String> {
    load_texture_filtered(path, FilterMode::Nearest).await
}

/// Configuration for loading textures from data files (JSON)
#[derive(Debug, serde::Deserialize)]
pub struct TextureConfig {
    pub key: String,
    pub path: String,
}

impl TextureConfig {
    /// Load texture configuration from a JSON string
    pub fn from_json(json: &str) -> Result<Vec<Self>, serde_json::Error> {
        serde_json::from_str(json)
    }
    
    /// Load texture configuration from a JSON file path
    pub async fn load_from_file(path: &str) -> Result<Vec<Self>, String> {
        match macroquad::prelude::load_string(path).await {
            Ok(json) => Self::from_json(&json).map_err(|e| e.to_string()),
            Err(e) => Err(format!("Failed to load config file: {}", e)),
        }
    }
}
