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
    fonts: HashMap<String, Font>,
    placeholder_texture: Option<Texture2D>,
    default_filter: FilterMode,
}

impl AssetManager {
    /// Create a new empty asset manager
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            fonts: HashMap::new(),
            placeholder_texture: None,
            default_filter: FilterMode::Nearest,
        }
    }

    /// Set the filter used by `load_texture`.
    pub fn set_default_filter(&mut self, filter: FilterMode) {
        self.default_filter = filter;
    }

    /// Load a single texture and add it to the cache
    ///
    /// The texture filter is automatically set to Nearest for pixel-perfect rendering.
    /// Returns an error if the texture fails to load.
    pub async fn load_texture(&mut self, name: &str, path: &str) -> Result<(), String> {
        self.load_texture_with_filter(name, path, self.default_filter)
            .await
    }

    /// Load a single texture with an explicit filter and add it to the cache.
    pub async fn load_texture_with_filter(
        &mut self,
        name: &str,
        path: &str,
        filter: FilterMode,
    ) -> Result<(), String> {
        match load_texture(path).await {
            Ok(texture) => {
                texture.set_filter(filter);
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

    /// Load textures from texture config entries.
    pub async fn load_texture_configs(&mut self, textures: &[TextureConfig]) -> usize {
        let mut loaded = 0;
        for texture in textures {
            if self
                .load_texture_with_filter(
                    &texture.key,
                    &texture.path,
                    texture
                        .filter
                        .map(FilterMode::from)
                        .unwrap_or(self.default_filter),
                )
                .await
                .is_ok()
            {
                loaded += 1;
            }
        }
        loaded
    }

    /// Load textures from a JSON manifest string.
    pub async fn load_texture_manifest_json(&mut self, json: &str) -> Result<usize, String> {
        let textures = TextureConfig::from_json(json).map_err(|e| e.to_string())?;
        Ok(self.load_texture_configs(&textures).await)
    }

    /// Load textures from a JSON manifest file.
    pub async fn load_texture_manifest_file(&mut self, path: &str) -> Result<usize, String> {
        let textures = TextureConfig::load_from_file(path).await?;
        Ok(self.load_texture_configs(&textures).await)
    }

    /// Set a named texture as the placeholder returned by `get_texture_or_placeholder`.
    pub fn set_placeholder_texture(&mut self, name: &str) -> bool {
        if let Some(texture) = self.textures.get(name) {
            self.placeholder_texture = Some(texture.clone());
            true
        } else {
            false
        }
    }

    /// Set the placeholder texture directly.
    pub fn set_placeholder_texture_direct(&mut self, texture: Texture2D) {
        self.placeholder_texture = Some(texture);
    }

    /// Get a texture by name. Returns None if not found.
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }

    /// Get a texture by name, falling back to the configured placeholder.
    pub fn get_texture_or_placeholder(&self, name: &str) -> Option<&Texture2D> {
        self.textures
            .get(name)
            .or_else(|| self.placeholder_texture.as_ref())
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

    /// Load and cache a TTF font.
    pub async fn load_font(&mut self, name: &str, path: &str) -> Result<(), String> {
        match load_ttf_font(path).await {
            Ok(font) => {
                self.fonts.insert(name.to_string(), font);
                Ok(())
            }
            Err(e) => Err(format!("Failed to load font '{}': {:?}", path, e)),
        }
    }

    /// Get a cached font by name.
    pub fn get_font(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }

    /// Check if a font with the given name exists.
    pub fn has_font(&self, name: &str) -> bool {
        self.fonts.contains_key(name)
    }

    /// Number of loaded fonts.
    pub fn font_len(&self) -> usize {
        self.fonts.len()
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextureConfig {
    pub key: String,
    pub path: String,
    #[serde(default)]
    pub filter: Option<TextureFilter>,
}

/// Texture manifest wrapper for `{ "textures": [...] }` files.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextureManifest {
    pub textures: Vec<TextureConfig>,
}

/// Serializable texture filter used by texture manifests.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextureFilter {
    Nearest,
    Linear,
}

impl From<TextureFilter> for FilterMode {
    fn from(filter: TextureFilter) -> Self {
        match filter {
            TextureFilter::Nearest => FilterMode::Nearest,
            TextureFilter::Linear => FilterMode::Linear,
        }
    }
}

impl TextureConfig {
    /// Load texture configuration from a JSON string
    pub fn from_json(json: &str) -> Result<Vec<Self>, serde_json::Error> {
        serde_json::from_str(json).or_else(|_| {
            let manifest: TextureManifest = serde_json::from_str(json)?;
            Ok(manifest.textures)
        })
    }

    /// Load texture configuration from a JSON file path
    pub async fn load_from_file(path: &str) -> Result<Vec<Self>, String> {
        match macroquad::prelude::load_string(path).await {
            Ok(json) => Self::from_json(&json).map_err(|e| e.to_string()),
            Err(e) => Err(format!("Failed to load config file: {}", e)),
        }
    }

    /// Return a copy with its path resolved through common native fallbacks.
    pub fn resolved(mut self, prefixes_to_strip: &[&str]) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if std::path::Path::new(&self.path).exists() {
                return self;
            }

            for prefix in prefixes_to_strip {
                if let Some(stripped) = self.path.strip_prefix(prefix) {
                    if std::path::Path::new(stripped).exists() {
                        self.path = stripped.to_string();
                        return self;
                    }
                }
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_config_from_array_json() {
        let json = r#"[{"key":"hero","path":"assets/hero.png","filter":"nearest"}]"#;
        let textures = TextureConfig::from_json(json).unwrap();

        assert_eq!(textures.len(), 1);
        assert_eq!(textures[0].key, "hero");
        assert!(matches!(textures[0].filter, Some(TextureFilter::Nearest)));
    }

    #[test]
    fn test_texture_config_from_manifest_json() {
        let json = r#"{"textures":[{"key":"bg","path":"assets/bg.png","filter":"linear"}]}"#;
        let textures = TextureConfig::from_json(json).unwrap();

        assert_eq!(textures.len(), 1);
        assert_eq!(textures[0].key, "bg");
        assert!(matches!(textures[0].filter, Some(TextureFilter::Linear)));
    }
}
