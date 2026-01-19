//! Persistence module for saving and loading game data
//!
//! Provides multiple storage backends:
//! - Native: JSON files on the filesystem
//! - WASM: quad-storage (localStorage wrapper)
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

use serde::{Serialize, de::DeserializeOwned};

#[cfg(not(target_arch = "wasm32"))]
use std::fs;
use std::path::{Path, PathBuf};

// ============================================================================
// Basic JSON File Operations (Native)
// ============================================================================

/// Save a serializable object to a JSON file (native only)
#[cfg(not(target_arch = "wasm32"))]
pub fn save_json<T: Serialize, P: AsRef<Path>>(path: P, data: &T) -> Result<(), String> {
    if let Some(parent) = path.as_ref().parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }

    let json = serde_json::to_string_pretty(data).map_err(|e| format!("Serialization error: {}", e))?;
    fs::write(path, json).map_err(|e| format!("Write error: {}", e))
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
    let serialized = serde_json::to_string(&wrapper)
        .map_err(|e| format!("Serialization error: {}", e))?;

    let key = format!("save_{}", slot_name);

    #[cfg(target_arch = "wasm32")]
    {
        use quad_storage::STORAGE;
        if let Ok(mut storage) = STORAGE.lock() {
            storage.set(&key, &serialized);
            Ok(())
        } else {
            Err("Failed to lock quad-storage".to_string())
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = get_app_data_path(game_name, &format!("{}.json", key))
            .ok_or_else(|| "Could not determine save path".to_string())?;

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }
        }

        fs::write(&path, serialized)
            .map_err(|e| format!("File write error: {}", e))
    }
}

/// Load game data from a named slot (cross-platform)
pub fn load_from_slot<T: DeserializeOwned>(
    game_name: &str,
    slot_name: &str,
) -> Result<T, String> {
    let key = format!("save_{}", slot_name);

    let content = {
        #[cfg(target_arch = "wasm32")]
        {
            use quad_storage::STORAGE;
            if let Ok(storage) = STORAGE.lock() {
                storage.get(&key)
                    .ok_or_else(|| format!("No save found for slot: {}", slot_name))?
            } else {
                return Err("Failed to lock quad-storage".to_string());
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = get_app_data_path(game_name, &format!("{}.json", key))
                .ok_or_else(|| "Could not determine save path".to_string())?;
            fs::read_to_string(&path)
                .map_err(|e| format!("File read error: {}", e))?
        }
    };

    let wrapper: LoadWrapper<T> = serde_json::from_str(&content)
        .map_err(|e| format!("Deserialization error: {}", e))?;

    Ok(wrapper.data)
}

/// Check if a save slot exists
pub fn slot_exists(game_name: &str, slot_name: &str) -> bool {
    let key = format!("save_{}", slot_name);

    #[cfg(target_arch = "wasm32")]
    {
        use quad_storage::STORAGE;
        if let Ok(storage) = STORAGE.lock() {
            storage.get(&key).is_some()
        } else {
            false
        }
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
        use quad_storage::STORAGE;
        if let Ok(mut storage) = STORAGE.lock() {
            storage.remove(&key);
            Ok(())
        } else {
            Err("Failed to lock quad-storage".to_string())
        }
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

/// Get list of known save slots (checks common slot names)
pub fn get_save_slots(game_name: &str) -> Vec<String> {
    let known_slots = ["slot_1", "slot_2", "slot_3", "autosave", "quicksave"];
    let mut saves = Vec::new();

    for slot in known_slots {
        if slot_exists(game_name, slot) {
            saves.push(slot.to_string());
        }
    }

    saves
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
