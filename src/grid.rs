//! Grid utilities for tile-based games
//!
//! Provides generic grid data structures and coordinate conversion utilities
//! for both 2D orthogonal and isometric grid systems.
//!
//! # Example
//! ```
//! use macroquad_toolkit::grid::{Grid, TilePos, world_to_iso, iso_to_world};
//!
//! // Create a grid
//! let mut grid: Grid<i32> = Grid::new(10, 10, 0);
//! grid.set(TilePos::new(5, 5), 1);
//!
//! // Coordinate conversion
//! let (iso_x, iso_y) = world_to_iso(5.0, 3.0, 64.0, 32.0);
//! ```

mod flat_grid;
mod fog;
mod iso;
mod pathfinding;
mod tile_pos;
mod vec_grid;
mod vision;

pub use flat_grid::FlatGrid;
pub use fog::{update_flat_fog_states, update_fog_states, FogState};
pub use iso::{iso_to_world, world_to_iso};
pub use pathfinding::{bfs_path, flood_fill, reachable_within};
pub use tile_pos::TilePos;
pub use vec_grid::Grid;
pub use vision::{calculate_visible_tiles, has_line_of_sight, line_positions, tiles_in_radius};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_tile_pos() {
        let pos1 = TilePos::new(0, 0);
        let pos2 = TilePos::new(3, 4);
        assert_eq!(pos1.manhattan_distance(&pos2), 7);
        assert!((pos1.distance_to(&pos2) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_grid_basic() {
        let mut grid: Grid<i32> = Grid::new(10, 10, 0);

        assert!(grid.is_valid(TilePos::new(5, 5)));
        assert!(!grid.is_valid(TilePos::new(10, 10)));
        assert!(!grid.is_valid(TilePos::new(-1, 0)));

        grid.set(TilePos::new(5, 5), 42);
        assert_eq!(*grid.get(TilePos::new(5, 5)).unwrap(), 42);
    }

    #[test]
    fn test_coordinate_conversion() {
        let tile_width = 64.0;
        let tile_height = 32.0;

        let (iso_x, iso_y) = world_to_iso(5.0, 3.0, tile_width, tile_height);
        let (world_x, world_y) = iso_to_world(iso_x, iso_y, tile_width, tile_height);

        assert!((world_x - 5.0).abs() < 0.001);
        assert!((world_y - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_line_of_sight() {
        // No blockers
        let visible = has_line_of_sight(TilePos::new(0, 0), TilePos::new(5, 5), |_| false);
        assert!(visible);

        // Blocker at (2, 2)
        let blocked = has_line_of_sight(TilePos::new(0, 0), TilePos::new(5, 5), |pos| {
            pos == TilePos::new(2, 2)
        });
        assert!(!blocked);
    }

    #[test]
    fn test_neighbors() {
        let grid: Grid<i32> = Grid::new(5, 5, 0);
        let center = TilePos::new(2, 2);

        let n4 = grid.neighbors_4way(center);
        assert_eq!(n4.len(), 4);

        let n8 = grid.neighbors_8way(center);
        assert_eq!(n8.len(), 8);

        // Corner should have fewer neighbors
        let corner = TilePos::new(0, 0);
        let corner_n4 = grid.neighbors_4way(corner);
        assert_eq!(corner_n4.len(), 2);
    }

    #[test]
    fn test_flat_grid_indexing() {
        let mut grid = FlatGrid::new(4, 3, 0);
        let pos = TilePos::new(2, 1);

        assert_eq!(grid.index(pos), Some(6));
        assert_eq!(grid.pos_from_index(6), Some(pos));

        assert!(grid.set(pos, 9));
        assert_eq!(grid.get(pos), Some(&9));
        assert!(!grid.set(TilePos::new(9, 9), 1));
    }

    #[test]
    fn test_bfs_path_and_flood_fill() {
        let mut grid = FlatGrid::new(5, 5, true);
        for y in 0..4 {
            grid.set(TilePos::new(2, y), false);
        }

        let path = grid
            .bfs_path(TilePos::new(0, 2), TilePos::new(4, 2), false, |_, tile| {
                *tile
            })
            .unwrap();
        assert_eq!(path.first().copied(), Some(TilePos::new(0, 2)));
        assert_eq!(path.last().copied(), Some(TilePos::new(4, 2)));
        assert!(path.contains(&TilePos::new(2, 4)));

        let reachable = grid.flood_fill(TilePos::new(0, 0), false, |_, tile| *tile);
        assert!(reachable.contains(&TilePos::new(4, 4)));
        assert!(!reachable.contains(&TilePos::new(2, 2)));
    }

    #[test]
    fn test_fog_update() {
        let mut grid = Grid::new(3, 3, FogState::Hidden);
        let visible = HashSet::from([TilePos::new(1, 1)]);

        update_fog_states(&mut grid, &visible);
        assert_eq!(grid.get(TilePos::new(1, 1)), Some(&FogState::Visible));

        update_fog_states(&mut grid, &HashSet::new());
        assert_eq!(grid.get(TilePos::new(1, 1)), Some(&FogState::Revealed));
    }
}
