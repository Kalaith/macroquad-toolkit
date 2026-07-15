//! Fog-of-war state tracking.

use super::{FlatGrid, Grid, TilePos};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Fog of war state for a tile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FogState {
    /// Tile has never been seen
    #[default]
    Hidden,
    /// Tile was seen before but is not currently visible
    Revealed,
    /// Tile is currently visible
    Visible,
}

/// Update fog states from a set of currently visible tiles.
///
/// Previous `Visible` tiles become `Revealed` when not visible this frame.
/// `Hidden` tiles stay hidden until they become visible.
pub fn update_fog_states(grid: &mut Grid<FogState>, visible: &HashSet<TilePos>) {
    for (pos, fog) in grid.iter_mut_with_pos() {
        *fog = if visible.contains(&pos) {
            FogState::Visible
        } else if *fog == FogState::Visible {
            FogState::Revealed
        } else {
            *fog
        };
    }
}

/// Update flat fog states from a set of currently visible tiles.
pub fn update_flat_fog_states(grid: &mut FlatGrid<FogState>, visible: &HashSet<TilePos>) {
    for (pos, fog) in grid.iter_mut_with_pos() {
        *fog = if visible.contains(&pos) {
            FogState::Visible
        } else if *fog == FogState::Visible {
            FogState::Revealed
        } else {
            *fog
        };
    }
}
