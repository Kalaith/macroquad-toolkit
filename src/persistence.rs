//! Persistence module for saving and loading game data using JSON
//!
//! Provides a `JsonStorage` trait and helper functions for easy serialization/deserialization.
//! 
//! # Example
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

use serde::{Serialize, de::DeserializeOwned};
use std::fs;
use std::path::{Path, PathBuf};

/// Save a serializable object to a JSON file
pub fn save_json<T: Serialize, P: AsRef<Path>>(path: P, data: &T) -> Result<(), String> {
    // Ensure parent directory exists
    if let Some(parent) = path.as_ref().parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }

    let json = serde_json::to_string_pretty(data).map_err(|e| format!("Serialization error: {}", e))?;
    fs::write(path, json).map_err(|e| format!("Write error: {}", e))
}

/// Load a deserializable object from a JSON file
pub fn load_json<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, String> {
    if !path.as_ref().exists() {
        return Err("File not found".to_string());
    }

    let json = fs::read_to_string(path).map_err(|e| format!("Read error: {}", e))?;
    serde_json::from_str(&json).map_err(|e| format!("Deserialization error: {}", e))
}

/// Check if a file exists
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// Get a standard application data path for a game
///
/// On Windows: `C:\Users\Username\AppData\Local\game_name\file_name`
/// On Mac/Linux: `~/.local/share/game_name/file_name`
pub fn get_app_data_path(game_name: &str, file_name: &str) -> Option<PathBuf> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        dirs::data_local_dir().map(|path| path.join(game_name).join(file_name))
    }
    #[cfg(target_arch = "wasm32")]
    {
        // WASM doesn't have a filesystem path concept in the same way, 
        // but we return a virtual path that could be used as a LocalStorage key
        Some(PathBuf::from(format!("{}/{}", game_name, file_name)))
    }
}

/// Trait for structs that can be saved/loaded directly
pub trait JsonStorage: Serialize + DeserializeOwned {
    /// Save this struct to the given path
    fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        save_json(path, self)
    }

    /// Load this struct from the given path
    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        load_json(path)
    }
}

// Implement JsonStorage for any type that implements Serialize + DeserializeOwned
impl<T: Serialize + DeserializeOwned> JsonStorage for T {}
