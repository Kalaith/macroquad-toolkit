//! Data loading utilities for JSON-based game data
//!
//! Provides patterns and helpers for loading game data from JSON files,
//! either at compile-time using `include_str!()` or at runtime.
//!
//! # Compile-Time Data Loading
//!
//! For data that should be embedded in the binary:
//!
//! ```rust,ignore
//! use macroquad_toolkit::data_loader::load_embedded_json;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct ItemData {
//!     name: String,
//!     value: i32,
//! }
//!
//! // In your code:
//! const ITEMS_JSON: &str = include_str!("../assets/data/items.json");
//! let items: Vec<ItemData> = load_embedded_json(ITEMS_JSON).expect("Failed to parse items");
//! ```
//!
//! # Runtime Data Loading
//!
//! For data loaded from files at runtime:
//!
//! ```rust,ignore
//! use macroquad_toolkit::data_loader::load_json_file;
//!
//! let items: Vec<ItemData> = load_json_file("assets/data/items.json").await?;
//! ```

use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Load JSON data from an embedded string (compile-time include)
///
/// Use this with `include_str!()` for data that should be compiled into the binary.
///
/// # Example
/// ```rust,ignore
/// const DATA: &str = include_str!("../data/items.json");
/// let items: Vec<Item> = load_embedded_json(DATA)?;
/// ```
pub fn load_embedded_json<T: DeserializeOwned>(json_str: &str) -> Result<T, String> {
    serde_json::from_str(json_str).map_err(|e| format!("JSON parse error: {}", e))
}

/// Load JSON data from an embedded string with a human-readable label in errors.
pub fn load_embedded_json_labeled<T: DeserializeOwned>(
    label: &str,
    json_str: &str,
) -> Result<T, String> {
    serde_json::from_str(json_str).map_err(|e| format!("JSON parse error in '{}': {}", label, e))
}

/// Load several embedded JSON strings that all deserialize to the same type.
pub fn load_many_embedded_json<T: DeserializeOwned>(
    inputs: &[(&str, &str)],
) -> Result<Vec<T>, String> {
    inputs
        .iter()
        .map(|(label, json)| load_embedded_json_labeled(label, json))
        .collect()
}

/// Load JSON data from an embedded string into a HashMap by ID field
///
/// Useful for data files that are arrays of objects with an "id" field.
///
/// # Example
/// ```rust,ignore
/// // items.json: [{"id": "sword", "damage": 10}, {"id": "shield", "defense": 5}]
/// const DATA: &str = include_str!("../data/items.json");
/// let items: HashMap<String, Item> = load_embedded_json_map(DATA, "id")?;
/// ```
pub fn load_embedded_json_map<T: DeserializeOwned + Clone>(
    json_str: &str,
    id_field: &str,
) -> Result<HashMap<String, T>, String> {
    // First parse as array of generic JSON values
    let values: Vec<serde_json::Value> =
        serde_json::from_str(json_str).map_err(|e| format!("JSON parse error: {}", e))?;

    let mut map = HashMap::new();

    for value in values {
        // Extract the ID
        let id = value
            .get(id_field)
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("Missing or invalid '{}' field", id_field))?
            .to_string();

        // Parse the full object
        let item: T = serde_json::from_value(value)
            .map_err(|e| format!("Failed to parse item '{}': {}", id, e))?;

        map.insert(id, item);
    }

    Ok(map)
}

/// Load JSON file at runtime (async, for macroquad)
#[cfg(not(target_arch = "wasm32"))]
pub async fn load_json_file<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("File read error: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("JSON parse error: {}", e))
}

/// Load JSON file at runtime (async, for WASM)
#[cfg(target_arch = "wasm32")]
pub async fn load_json_file<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    let content = macroquad::file::load_string(path)
        .await
        .map_err(|e| format!("File load error: {:?}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("JSON parse error: {}", e))
}

/// Load a data file from "assets/data/{name}.json"
///
/// This provides a convenient shorthand for loading game data files.
///
/// # Example
/// ```rust,ignore
/// let items: Vec<Item> = load_data("items").await?;
/// ```
pub async fn load_data<T: DeserializeOwned>(name: &str) -> Result<T, String> {
    let path = format!("assets/data/{}.json", name);
    load_json_file(&path).await
}

/// Synchronous JSON file loading (native only)
#[cfg(not(target_arch = "wasm32"))]
pub fn load_json_file_sync<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("File read error: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("JSON parse error: {}", e))
}

/// Build a path relative to a crate manifest directory.
pub fn manifest_relative_path(manifest_dir: &str, relative_path: &str) -> PathBuf {
    Path::new(manifest_dir).join(relative_path)
}

/// Resolve the first existing path from candidates.
#[cfg(not(target_arch = "wasm32"))]
pub fn first_existing_path<'a>(candidates: impl IntoIterator<Item = &'a str>) -> Option<PathBuf> {
    candidates
        .into_iter()
        .map(PathBuf::from)
        .find(|path| path.exists())
}

/// Load JSON from the first existing native path, or from embedded JSON if no path exists.
#[cfg(not(target_arch = "wasm32"))]
pub fn load_json_with_fallback_sync<T: DeserializeOwned>(
    label: &str,
    path_candidates: &[PathBuf],
    embedded_json: &str,
) -> Result<T, String> {
    for path in path_candidates {
        if path.exists() {
            let content = std::fs::read_to_string(path)
                .map_err(|e| format!("File read error in '{}': {}", path.display(), e))?;
            return serde_json::from_str(&content)
                .map_err(|e| format!("JSON parse error in '{}': {}", path.display(), e));
        }
    }

    load_embedded_json_labeled(label, embedded_json)
}

/// WASM fallback: parse the embedded JSON, since native filesystem paths are unavailable.
#[cfg(target_arch = "wasm32")]
pub fn load_json_with_fallback_sync<T: DeserializeOwned>(
    label: &str,
    _path_candidates: &[PathBuf],
    embedded_json: &str,
) -> Result<T, String> {
    load_embedded_json_labeled(label, embedded_json)
}

/// Helper macro for defining data types with automatic JSON loading
///
/// This macro generates a struct and associated loading functions.
///
/// # Example
/// ```rust,ignore
/// define_data_type! {
///     /// Item definition loaded from JSON
///     pub struct ItemData {
///         pub id: String,
///         pub name: String,
///         pub value: i32,
///         pub stackable: bool,
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_game_data {
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                pub $field:ident: $type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            $(
                $(#[$field_meta])*
                pub $field: $type
            ),*
        }
    };
}

/// Registry for game data that can be loaded from multiple sources
#[derive(Debug, Clone)]
pub struct DataRegistry<T> {
    data: HashMap<String, T>,
}

impl<T: Clone> DataRegistry<T> {
    /// Create an empty registry
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Create a registry from a HashMap
    pub fn from_map(data: HashMap<String, T>) -> Self {
        Self { data }
    }

    /// Get an item by ID
    pub fn get(&self, id: &str) -> Option<&T> {
        self.data.get(id)
    }

    /// Check if an ID exists
    pub fn contains(&self, id: &str) -> bool {
        self.data.contains_key(id)
    }

    /// Get all IDs
    pub fn ids(&self) -> impl Iterator<Item = &String> {
        self.data.keys()
    }

    /// Get all items
    pub fn iter(&self) -> impl Iterator<Item = (&String, &T)> {
        self.data.iter()
    }

    /// Get the number of items
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Insert or update an item
    pub fn insert(&mut self, id: String, item: T) {
        self.data.insert(id, item);
    }

    /// Remove an item
    pub fn remove(&mut self, id: &str) -> Option<T> {
        self.data.remove(id)
    }

    /// Merge another registry into this one
    pub fn merge(&mut self, other: DataRegistry<T>) {
        self.data.extend(other.data);
    }
}

impl<T: Clone> Default for DataRegistry<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + DeserializeOwned> DataRegistry<T> {
    /// Load from embedded JSON array with ID field
    pub fn from_embedded_json(json_str: &str, id_field: &str) -> Result<Self, String> {
        let map = load_embedded_json_map(json_str, id_field)?;
        Ok(Self::from_map(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Clone, Deserialize, PartialEq)]
    struct TestItem {
        id: String,
        name: String,
        value: i32,
    }

    #[test]
    fn test_load_embedded_json() {
        let json = r#"[
            {"id": "sword", "name": "Iron Sword", "value": 100},
            {"id": "shield", "name": "Wooden Shield", "value": 50}
        ]"#;

        let items: Vec<TestItem> = load_embedded_json(json).unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].id, "sword");
    }

    #[test]
    fn test_load_embedded_json_map() {
        let json = r#"[
            {"id": "sword", "name": "Iron Sword", "value": 100},
            {"id": "shield", "name": "Wooden Shield", "value": 50}
        ]"#;

        let items: HashMap<String, TestItem> = load_embedded_json_map(json, "id").unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items.get("sword").unwrap().value, 100);
        assert_eq!(items.get("shield").unwrap().name, "Wooden Shield");
    }

    #[test]
    fn test_data_registry() {
        let json = r#"[
            {"id": "sword", "name": "Iron Sword", "value": 100},
            {"id": "shield", "name": "Wooden Shield", "value": 50}
        ]"#;

        let registry: DataRegistry<TestItem> =
            DataRegistry::from_embedded_json(json, "id").unwrap();

        assert_eq!(registry.len(), 2);
        assert!(registry.contains("sword"));
        assert_eq!(registry.get("sword").unwrap().name, "Iron Sword");
    }
}
