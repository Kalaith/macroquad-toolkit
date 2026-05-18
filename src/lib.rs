//! Macroquad Toolkit
//!
//! A comprehensive collection of utilities for Macroquad game development.
//! Extracted from multiple games to reduce duplication and provide consistent patterns.
//!
//! # Module Organization
//!
//! ## 2D Game Development
//! - [`camera`] - 2D camera with pan and zoom
//! - [`sprite`] - Sprite rendering utilities
//! - [`ui`] - UI components (buttons, panels, progress bars)
//! - [`input`] - Input handling helpers
//! - [`colors`] - Color palettes (dark theme, rarity colors)
//!
//! ## 3D Game Development
//! - [`render3d`] - 3D rendering utilities
//!   - [`render3d::billboard`] - Billboard sprites for 3D
//!   - [`render3d::camera`] - Isometric and orbit cameras
//!
//! ## Grid & Pathfinding
//! - [`grid`] - Grid data structures, coordinate conversion, fog of war
//! - [`pathfinding`] - A* pathfinding with caching
//!
//! ## Game Systems
//! - [`entities`] - Generic entity management system
//! - [`notifications`] - Toast notification system
//! - [`states`] - Game state machine helpers
//! - [`rng`] - Random number utilities
//!
//! ## Data & Persistence
//! - [`persistence`] - Save/load system (native + WASM)
//! - [`data_loader`] - JSON data loading patterns
//! - [`assets`] - Asset management and texture loading
//!
//! ## Other
//! - [`audio`] - Audio playback utilities
//! - [`events`] - Event handling utilities
//! - [`db`] - Database support (optional, requires `db` feature)

// Core 2D modules (existing)
pub mod assets;
pub mod audio;
pub mod camera;
pub mod colors;
pub mod events;
pub mod input;
pub mod rng;
pub mod sprite;
pub mod states;
pub mod ui;

// New modules extracted from dungeon_manager
pub mod data_loader;
pub mod entities;
pub mod grid;
pub mod notifications;
pub mod pathfinding;
pub mod persistence;
pub mod render3d;

// Optional database support
#[cfg(feature = "db")]
pub mod db;

// WASM storage support
#[cfg(target_arch = "wasm32")]
pub mod wasm_storage;

/// Convenient re-exports for common 2D game development usage
pub mod prelude {
    // Input and UI
    pub use crate::colors::dark;
    pub use crate::input::*;
    pub use crate::ui::*;

    // Assets and rendering
    pub use crate::assets::{AssetManager, AssetPack};
    pub use crate::camera::{Camera2D, Camera2DConfig, CameraBounds};
    pub use crate::sprite::Sprite;

    // Persistence
    pub use crate::persistence::*;

    // Game systems
    pub use crate::notifications::{NotificationManager, NotificationType};
    pub use crate::rng::*;
    pub use crate::states::*;
}

/// Re-exports for 3D game development
pub mod prelude_3d {
    pub use crate::entities::*;
    pub use crate::grid::*;
    pub use crate::pathfinding::*;
    pub use crate::render3d::*;
}

/// Re-exports for data-driven game development
pub mod prelude_data {
    pub use crate::data_loader::*;
    pub use crate::persistence::*;
}
