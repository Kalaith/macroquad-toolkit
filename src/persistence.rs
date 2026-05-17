//! Persistence module for saving and loading game data
//!
//! Provides multiple storage backends:
//! - Native: JSON files on the filesystem
//! - WASM: web-sys localStorage
//!
//! # Example (Native)
//! ```no_run
//! use serde::{Serialize, Deserialize};
//! use macroquad_toolkit::persistence::{save_json, load_json, get_app_data_path};
//!
//! #[derive(Serialize, Deserialize)]
//! struct SaveData {
//!     score: u32,
//! }
//!
//! # fn main() {
//! let data = SaveData { score: 100 };
//! let path = get_app_data_path("my_game", "save.json").unwrap();
//! save_json(&path, &data).unwrap();
//! # }
//! ```
//!
//! # Example (Cross-Platform Save Slots)
//! ```no_run
//! use serde::{Serialize, Deserialize};
//! use macroquad_toolkit::persistence::{SaveSlot, save_to_slot, load_from_slot};
//!
//! #[derive(Serialize, Deserialize, Clone)]
//! struct GameState {
//!     level: u32,
//!     health: f32,
//! }
//!
//! # fn main() {
//! let state = GameState { level: 5, health: 100.0 };
//! save_to_slot("my_game", "slot_1", &state).unwrap();
//!
//! let loaded: GameState = load_from_slot("my_game", "slot_1").unwrap();
//! # }
//! ```

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

#[cfg(not(target_arch = "wasm32"))]
use std::fs;
use std::path::{Path, PathBuf};

fn sanitize_key(key: &str) -> String {
    key.chars()
        .map(|ch| match ch {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => ch,
        })
        .collect()
}

fn key_file_name(key: &str) -> String {
    let sanitized = sanitize_key(key);
    if sanitized.ends_with(".json") {
        sanitized
    } else {
        format!("{}.json", sanitized)
    }
}

#[cfg(target_arch = "wasm32")]
fn storage_key(game_name: &str, key: &str) -> String {
    format!("{}_{}", sanitize_key(game_name), sanitize_key(key))
}

// ============================================================================
// Basic JSON File Operations (Native)
// ============================================================================

/// Save a serializable object to a JSON file (native only)
#[cfg(not(target_arch = "wasm32"))]
pub fn save_json<T: Serialize, P: AsRef<Path>>(path: P, data: &T) -> Result<(), String> {
    save_json_atomic(path, data)
}

/// Save a serializable object to a JSON file using a temp file and replace.
#[cfg(not(target_arch = "wasm32"))]
pub fn save_json_atomic<T: Serialize, P: AsRef<Path>>(path: P, data: &T) -> Result<(), String> {
    if let Some(parent) = path.as_ref().parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }

    let json =
        serde_json::to_string_pretty(data).map_err(|e| format!("Serialization error: {}", e))?;
    save_string_atomic(path, &json)
}

/// Save raw text to a file using a temp file and replace.
#[cfg(not(target_arch = "wasm32"))]
pub fn save_string_atomic<P: AsRef<Path>>(path: P, content: &str) -> Result<(), String> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }

    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("save");
    let tmp_path = path.with_file_name(format!(".{}.tmp", file_name));

    fs::write(&tmp_path, content).map_err(|e| format!("Temp write error: {}", e))?;

    #[cfg(windows)]
    {
        if path.exists() {
            fs::remove_file(path).map_err(|e| format!("Replace remove error: {}", e))?;
        }
    }

    fs::rename(&tmp_path, path).map_err(|e| {
        let _ = fs::remove_file(&tmp_path);
        format!("Replace error: {}", e)
    })
}

/// Load a deserializable object from a JSON file (native only)
#[cfg(not(target_arch = "wasm32"))]
pub fn load_json<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, String> {
    if !path.as_ref().exists() {
        return Err("File not found".to_string());
    }

    let json = fs::read_to_string(path).map_err(|e| format!("Read error: {}", e))?;
    serde_json::from_str(&json).map_err(|e| format!("Deserialization error: {}", e))
}

/// Check if a file exists
#[cfg(not(target_arch = "wasm32"))]
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

#[cfg(target_arch = "wasm32")]
pub fn file_exists<P: AsRef<Path>>(_path: P) -> bool {
    false // WASM doesn't have traditional file system
}

/// Get a standard application data path for a game
///
/// On Windows: `C:\Users\Username\AppData\Local\game_name\file_name`
/// On Mac/Linux: `~/.local/share/game_name/file_name`
/// On WASM: Returns a virtual path (use save slots instead)
pub fn get_app_data_path(game_name: &str, file_name: &str) -> Option<PathBuf> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        dirs::data_local_dir().map(|path| path.join(game_name).join(file_name))
    }
    #[cfg(target_arch = "wasm32")]
    {
        Some(PathBuf::from(format!("{}/{}", game_name, file_name)))
    }
}

/// Resolve a save path with an optional test environment override.
///
/// If `test_env_var` is set and the environment variable exists, that value is
/// used as the complete file path. Otherwise this returns `get_app_data_path`.
pub fn get_configured_save_path(
    game_name: &str,
    file_name: &str,
    test_env_var: Option<&str>,
) -> Option<PathBuf> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(var) = test_env_var {
            if let Ok(path) = std::env::var(var) {
                return Some(PathBuf::from(path));
            }
        }
    }

    get_app_data_path(game_name, file_name)
}

/// Resolve a nested data-directory path with an optional test override.
///
/// Native builds use the first available base directory from `dirs::data_dir`,
/// `dirs::document_dir`, or the current working directory, then append each
/// segment and the file name. WASM returns a virtual path built from the same
/// segments.
pub fn get_nested_data_path(
    segments: &[&str],
    file_name: &str,
    test_env_var: Option<&str>,
) -> Option<PathBuf> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(var) = test_env_var {
            if let Ok(path) = std::env::var(var) {
                return Some(PathBuf::from(path));
            }
        }

        let mut path = dirs::data_dir()
            .or_else(dirs::document_dir)
            .or_else(|| std::env::current_dir().ok())?;
        for segment in segments {
            path.push(segment);
        }
        path.push(file_name);
        Some(path)
    }

    #[cfg(target_arch = "wasm32")]
    {
        let mut path = PathBuf::new();
        for segment in segments {
            path.push(segment);
        }
        path.push(file_name);
        Some(path)
    }
}

/// Resolve a conventional WebHatchery game app data path.
///
/// Native path shape: `{data_dir}/WebHatchery/game_apps/{game_slug}/{file_name}`.
pub fn get_webhatchery_game_app_path(
    game_slug: &str,
    file_name: &str,
    test_env_var: Option<&str>,
) -> Option<PathBuf> {
    get_nested_data_path(
        &["WebHatchery", "game_apps", game_slug],
        file_name,
        test_env_var,
    )
}

/// Save raw string content to a named JSON key.
pub fn save_string_key(game_name: &str, key: &str, content: &str) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        crate::wasm_storage::storage_set(&storage_key(game_name, key), content);
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = get_app_data_path(game_name, &key_file_name(key))
            .ok_or_else(|| "Could not determine save path".to_string())?;
        save_string_atomic(path, content)
    }
}

/// Load raw string content from a named JSON key.
pub fn load_string_key(game_name: &str, key: &str) -> Result<String, String> {
    #[cfg(target_arch = "wasm32")]
    {
        crate::wasm_storage::storage_get(&storage_key(game_name, key))
            .ok_or_else(|| format!("No data found for key: {}", key))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = get_app_data_path(game_name, &key_file_name(key))
            .ok_or_else(|| "Could not determine save path".to_string())?;
        fs::read_to_string(&path).map_err(|e| format!("File read error: {}", e))
    }
}

/// Save JSON data to a named key.
pub fn save_json_key<T: Serialize>(game_name: &str, key: &str, data: &T) -> Result<(), String> {
    let json =
        serde_json::to_string_pretty(data).map_err(|e| format!("Serialization error: {}", e))?;
    save_string_key(game_name, key, &json)
}

/// Load JSON data from a named key.
pub fn load_json_key<T: DeserializeOwned>(game_name: &str, key: &str) -> Result<T, String> {
    let content = load_string_key(game_name, key)?;
    serde_json::from_str(&content).map_err(|e| format!("Deserialization error: {}", e))
}

/// Check if a named JSON key exists.
pub fn json_key_exists(game_name: &str, key: &str) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        crate::wasm_storage::storage_exists(&storage_key(game_name, key))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        get_app_data_path(game_name, &key_file_name(key))
            .map(|path| path.exists())
            .unwrap_or(false)
    }
}

/// Delete a named JSON key.
pub fn delete_json_key(game_name: &str, key: &str) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        crate::wasm_storage::storage_remove(&storage_key(game_name, key));
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = get_app_data_path(game_name, &key_file_name(key))
            .ok_or_else(|| "Could not determine save path".to_string())?;
        if path.exists() {
            fs::remove_file(&path).map_err(|e| format!("Failed to delete: {}", e))?;
        }
        Ok(())
    }
}

/// Extract a common save-version field from parsed JSON.
pub fn peek_version_value(value: &Value) -> Option<String> {
    fn as_version(value: &Value) -> Option<String> {
        match value {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            _ => None,
        }
    }

    value
        .get("version")
        .and_then(as_version)
        .or_else(|| value.get("save_version").and_then(as_version))
        .or_else(|| value.get("schema_version").and_then(as_version))
        .or_else(|| {
            value
                .get("slot")
                .and_then(|slot| slot.get("version"))
                .and_then(as_version)
        })
}

/// Parse JSON and extract a common save-version field.
pub fn peek_version_from_str(json: &str) -> Result<Option<String>, String> {
    let value: Value =
        serde_json::from_str(json).map_err(|e| format!("JSON parse error: {}", e))?;
    Ok(peek_version_value(&value))
}

/// Load raw JSON for a key and extract a common save-version field.
pub fn peek_json_key_version(game_name: &str, key: &str) -> Result<Option<String>, String> {
    let content = load_string_key(game_name, key)?;
    peek_version_from_str(&content)
}

/// Load a JSON key and migrate it when its version differs from `current_version`.
///
/// The migration callback receives the detected version and the raw JSON value.
/// If the version already matches, the value is deserialized directly into `T`.
pub fn load_json_key_with_migration<T, F>(
    game_name: &str,
    key: &str,
    current_version: &str,
    migrate: F,
) -> Result<T, String>
where
    T: DeserializeOwned,
    F: FnOnce(Option<String>, Value) -> Result<T, String>,
{
    let content = load_string_key(game_name, key)?;
    let value: Value =
        serde_json::from_str(&content).map_err(|e| format!("JSON parse error: {}", e))?;
    let version = peek_version_value(&value);

    if version.as_deref() == Some(current_version) {
        serde_json::from_value(value).map_err(|e| format!("Deserialization error: {}", e))
    } else {
        migrate(version, value)
    }
}

// ============================================================================
// Cross-Platform Save Slot System
// ============================================================================

/// Save slot metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveSlot {
    /// Slot name/identifier
    pub name: String,
    /// When the save was created (ISO 8601 string)
    pub save_date: String,
    /// Game version that created this save
    pub version: String,
}

impl SaveSlot {
    /// Create a new save slot
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            save_date: "Unknown".to_string(), // Could use chrono if available
            version: version.into(),
        }
    }
}

/// Wrapper for saving game state with metadata
#[derive(serde::Serialize)]
struct SaveWrapper<'a, T: Serialize> {
    slot: SaveSlot,
    data: &'a T,
}

/// Wrapper for loading game state
#[derive(serde::Deserialize)]
struct LoadWrapper<T> {
    #[allow(dead_code)]
    slot: SaveSlot,
    data: T,
}

/// Save game data to a named slot (cross-platform)
///
/// - Native: Saves to `{app_data}/{game_name}/save_{slot_name}.json`
/// - WASM: Saves to localStorage via quad-storage
pub fn save_to_slot<T: Serialize>(
    game_name: &str,
    slot_name: &str,
    data: &T,
) -> Result<(), String> {
    save_to_slot_with_version(game_name, slot_name, data, "1.0.0")
}

/// Save game data with explicit version
pub fn save_to_slot_with_version<T: Serialize>(
    game_name: &str,
    slot_name: &str,
    data: &T,
    version: &str,
) -> Result<(), String> {
    let slot = SaveSlot::new(slot_name, version);
    let wrapper = SaveWrapper { slot, data };
    let serialized =
        serde_json::to_string(&wrapper).map_err(|e| format!("Serialization error: {}", e))?;

    let key = format!("save_{}", slot_name);

    #[cfg(target_arch = "wasm32")]
    {
        let _ = game_name; // unused in WASM
        crate::wasm_storage::storage_set(&key, &serialized);
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = get_app_data_path(game_name, &format!("{}.json", key))
            .ok_or_else(|| "Could not determine save path".to_string())?;
        save_string_atomic(&path, &serialized)
    }
}

/// Load game data from a named slot (cross-platform)
pub fn load_from_slot<T: DeserializeOwned>(game_name: &str, slot_name: &str) -> Result<T, String> {
    let key = format!("save_{}", slot_name);

    let content = {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = game_name; // unused in WASM
            crate::wasm_storage::storage_get(&key)
                .ok_or_else(|| format!("No save found for slot: {}", slot_name))?
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = get_app_data_path(game_name, &format!("{}.json", key))
                .ok_or_else(|| "Could not determine save path".to_string())?;
            fs::read_to_string(&path).map_err(|e| format!("File read error: {}", e))?
        }
    };

    let wrapper: LoadWrapper<T> =
        serde_json::from_str(&content).map_err(|e| format!("Deserialization error: {}", e))?;

    Ok(wrapper.data)
}

/// Check if a save slot exists
pub fn slot_exists(game_name: &str, slot_name: &str) -> bool {
    let key = format!("save_{}", slot_name);

    #[cfg(target_arch = "wasm32")]
    {
        let _ = game_name; // unused in WASM
        crate::wasm_storage::storage_exists(&key)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(path) = get_app_data_path(game_name, &format!("{}.json", key)) {
            path.exists()
        } else {
            false
        }
    }
}

/// Delete a save slot
pub fn delete_slot(game_name: &str, slot_name: &str) -> Result<(), String> {
    let key = format!("save_{}", slot_name);

    #[cfg(target_arch = "wasm32")]
    {
        let _ = game_name; // unused in WASM
        crate::wasm_storage::storage_remove(&key);
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = get_app_data_path(game_name, &format!("{}.json", key))
            .ok_or_else(|| "Could not determine save path".to_string())?;

        if path.exists() {
            fs::remove_file(&path).map_err(|e| format!("Failed to delete: {}", e))
        } else {
            Ok(()) // Already doesn't exist
        }
    }
}

/// Peek the version recorded in a save slot.
pub fn peek_slot_version(game_name: &str, slot_name: &str) -> Result<Option<String>, String> {
    let key = format!("save_{}", slot_name);

    let content = {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = game_name;
            crate::wasm_storage::storage_get(&key)
                .ok_or_else(|| format!("No save found for slot: {}", slot_name))?
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = get_app_data_path(game_name, &format!("{}.json", key))
                .ok_or_else(|| "Could not determine save path".to_string())?;
            fs::read_to_string(&path).map_err(|e| format!("File read error: {}", e))?
        }
    };

    peek_version_from_str(&content)
}

/// Load game data from a slot and run a migration callback if the slot version differs.
pub fn load_from_slot_with_migration<T, F>(
    game_name: &str,
    slot_name: &str,
    current_version: &str,
    migrate: F,
) -> Result<T, String>
where
    T: DeserializeOwned,
    F: FnOnce(Option<String>, Value) -> Result<T, String>,
{
    let key = format!("save_{}", slot_name);

    let content = {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = game_name;
            crate::wasm_storage::storage_get(&key)
                .ok_or_else(|| format!("No save found for slot: {}", slot_name))?
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = get_app_data_path(game_name, &format!("{}.json", key))
                .ok_or_else(|| "Could not determine save path".to_string())?;
            fs::read_to_string(&path).map_err(|e| format!("File read error: {}", e))?
        }
    };

    let value: Value =
        serde_json::from_str(&content).map_err(|e| format!("JSON parse error: {}", e))?;
    let version = peek_version_value(&value);
    if version.as_deref() == Some(current_version) {
        let wrapper: LoadWrapper<T> =
            serde_json::from_value(value).map_err(|e| format!("Deserialization error: {}", e))?;
        Ok(wrapper.data)
    } else {
        migrate(version, value)
    }
}

/// Get list of save slots.
///
/// Native builds scan the app data directory for `save_*.json`; WASM falls
/// back to common slot names because localStorage cannot be enumerated through
/// the lightweight storage helper.
pub fn get_save_slots(game_name: &str) -> Vec<String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(dir) = dirs::data_local_dir().map(|path| path.join(game_name)) else {
            return Vec::new();
        };
        let Ok(entries) = fs::read_dir(dir) else {
            return Vec::new();
        };

        let mut saves: Vec<String> = entries
            .flatten()
            .filter_map(|entry| entry.file_name().into_string().ok())
            .filter_map(|name| {
                name.strip_prefix("save_")
                    .and_then(|name| name.strip_suffix(".json"))
                    .map(ToOwned::to_owned)
            })
            .collect();
        saves.sort();
        return saves;
    }

    #[cfg(target_arch = "wasm32")]
    {
        let known_slots = ["slot_1", "slot_2", "slot_3", "autosave", "quicksave"];
        let mut saves = Vec::new();

        for slot in known_slots {
            if slot_exists(game_name, slot) {
                saves.push(slot.to_string());
            }
        }

        saves
    }
}

// ============================================================================
// JsonStorage Trait
// ============================================================================

/// Trait for structs that can be saved/loaded directly
pub trait JsonStorage: Serialize + DeserializeOwned {
    /// Save this struct to the given path (native only)
    #[cfg(not(target_arch = "wasm32"))]
    fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        save_json(path, self)
    }

    /// Load this struct from the given path (native only)
    #[cfg(not(target_arch = "wasm32"))]
    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        load_json(path)
    }

    /// Save to a slot (cross-platform)
    fn save_to_slot(&self, game_name: &str, slot_name: &str) -> Result<(), String> {
        save_to_slot(game_name, slot_name, self)
    }

    /// Load from a slot (cross-platform)
    fn load_from_slot(game_name: &str, slot_name: &str) -> Result<Self, String> {
        load_from_slot(game_name, slot_name)
    }
}

impl<T: Serialize + DeserializeOwned> JsonStorage for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_version_from_common_shapes() {
        assert_eq!(
            peek_version_from_str(r#"{"version":"2.0.0","data":{}}"#).unwrap(),
            Some("2.0.0".to_string())
        );
        assert_eq!(
            peek_version_from_str(r#"{"save_version":6,"state":{}}"#).unwrap(),
            Some("6".to_string())
        );
        assert_eq!(
            peek_version_from_str(r#"{"slot":{"version":"1.1"},"data":{}}"#).unwrap(),
            Some("1.1".to_string())
        );
    }

    #[test]
    fn test_key_file_name_sanitizes_paths() {
        assert_eq!(key_file_name("profile/settings"), "profile_settings.json");
        assert_eq!(key_file_name("save.json"), "save.json");
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_nested_path_uses_override() {
        let path = std::env::temp_dir().join("toolkit_nested_path_override.json");
        std::env::set_var("TOOLKIT_TEST_SAVE_PATH", &path);

        let resolved = get_nested_data_path(
            &["WebHatchery", "game_apps", "test"],
            "save.json",
            Some("TOOLKIT_TEST_SAVE_PATH"),
        )
        .unwrap();

        assert_eq!(resolved, path);
        std::env::remove_var("TOOLKIT_TEST_SAVE_PATH");
    }
}
