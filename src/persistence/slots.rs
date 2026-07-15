//! Cross-platform save slot system.

#[cfg(not(target_arch = "wasm32"))]
use super::files::{get_app_data_path, save_string_atomic};
use super::version::{peek_version_from_str, peek_version_value};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;

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
        saves
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
