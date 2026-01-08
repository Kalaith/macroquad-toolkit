//! Audio management module
//!
//! Handles loading and playing sound effects with volume control.

use macroquad::audio::{Sound, PlaySoundParams, play_sound, load_sound};
use std::collections::HashMap;

/// Trait for easier sound indexing (usually an Enum)
pub trait SoundId: std::cmp::Eq + std::hash::Hash + Copy {}
impl<T: std::cmp::Eq + std::hash::Hash + Copy> SoundId for T {}

/// Generic Sound Manager
pub struct SoundManager<T: SoundId> {
    sounds: HashMap<T, Sound>,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub visible: bool, // Can be used to mute when window is hidden
}

impl<T: SoundId> SoundManager<T> {
    /// Create a new empty Sound Manager
    pub fn new() -> Self {
        Self {
            sounds: HashMap::new(),
            sfx_volume: 1.0,
            music_volume: 1.0,
            visible: true,
        }
    }

    /// Load a sound and associate it with an ID
    pub async fn load_sound(&mut self, id: T, path: &str) -> Result<(), String> {
        match load_sound(path).await {
            Ok(sound) => {
                self.sounds.insert(id, sound);
                Ok(())
            }
            Err(e) => Err(format!("Failed to load sound '{}': {:?}", path, e)),
        }
    }

    /// Play a sound effect
    ///
    /// The volume is multiplied by the global sfx_volume.
    pub fn play_sfx(&self, id: T, volume_multiplier: f32) {
        if !self.visible { return; }
        
        if let Some(sound) = self.sounds.get(&id) {
            play_sound(
                sound, // Sound is Copy in macroquad 0.4
                PlaySoundParams {
                    looped: false,
                    volume: self.sfx_volume * volume_multiplier,
                }
            );
        }
    }

    /// Play a sound directly (ignoring volume settings, use carefully)
    pub fn play_raw(&self, id: T, params: PlaySoundParams) {
        if !self.visible { return; }

        if let Some(sound) = self.sounds.get(&id) {
            play_sound(sound, params);
        }
    }
    
    /// Check if a sound is loaded
    pub fn has_sound(&self, id: T) -> bool {
        self.sounds.contains_key(&id)
    }
}

impl<T: SoundId> Default for SoundManager<T> {
    fn default() -> Self {
        Self::new()
    }
}
