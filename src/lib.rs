//! Macroquad Toolkit
//!
//! A collection of common utilities for Macroquad game development.
//! Extracted from multiple games to reduce duplication and provide
//! consistent patterns.

pub mod input;
pub mod colors;
pub mod events;
pub mod assets;
pub mod sprite;
pub mod camera;
pub mod ui;

/// Convenient re-exports for common usage
pub mod prelude {
    pub use crate::input::*;
    pub use crate::ui::*;
    pub use crate::colors::dark;
    pub use crate::assets::AssetManager;
}
