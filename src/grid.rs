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

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// A position in tile coordinates
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

impl TilePos {
    /// Create a new tile position
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Calculate Manhattan distance to another position
    pub fn manhattan_distance(&self, other: &TilePos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Calculate Euclidean distance to another position
    pub fn distance_to(&self, other: &TilePos) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    /// Get 4-way neighbors (N, S, E, W)
    pub fn neighbors_4way(&self) -> [TilePos; 4] {
        [
            TilePos::new(self.x, self.y - 1),
            TilePos::new(self.x + 1, self.y),
            TilePos::new(self.x, self.y + 1),
            TilePos::new(self.x - 1, self.y),
        ]
    }

    /// Get 8-way neighbors (includes diagonals)
    pub fn neighbors_8way(&self) -> [TilePos; 8] {
        [
            TilePos::new(self.x, self.y - 1),
            TilePos::new(self.x + 1, self.y - 1),
            TilePos::new(self.x + 1, self.y),
            TilePos::new(self.x + 1, self.y + 1),
            TilePos::new(self.x, self.y + 1),
            TilePos::new(self.x - 1, self.y + 1),
            TilePos::new(self.x - 1, self.y),
            TilePos::new(self.x - 1, self.y - 1),
        ]
    }

    /// Convert to (f32, f32) tuple
    pub fn to_f32(&self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }

    /// Check if this position is inside a width/height grid.
    pub fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.x >= 0 && self.y >= 0 && (self.x as usize) < width && (self.y as usize) < height
    }

    /// Convert this position to a flat vector index.
    pub fn to_index(&self, width: usize) -> Option<usize> {
        if self.x < 0 || self.y < 0 {
            return None;
        }
        Some(self.y as usize * width + self.x as usize)
    }

    /// Convert a flat vector index to a tile position.
    pub fn from_index(index: usize, width: usize) -> Self {
        Self::new((index % width) as i32, (index / width) as i32)
    }
}

impl From<(i32, i32)> for TilePos {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

impl From<TilePos> for (i32, i32) {
    fn from(pos: TilePos) -> Self {
        (pos.x, pos.y)
    }
}

/// A 2D grid data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone> Grid<T> {
    /// Create a new grid filled with the default value
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: vec![vec![default; width]; height],
            width,
            height,
        }
    }

    /// Create a grid from an existing 2D vector
    pub fn from_vec(data: Vec<Vec<T>>) -> Self {
        let height = data.len();
        let width = data.first().map(|r| r.len()).unwrap_or(0);
        Self {
            data,
            width,
            height,
        }
    }

    /// Check if a position is within bounds
    pub fn is_valid(&self, pos: TilePos) -> bool {
        pos.x >= 0 && pos.y >= 0 && (pos.x as usize) < self.width && (pos.y as usize) < self.height
    }

    /// Get a tile at position (returns None if out of bounds)
    pub fn get(&self, pos: TilePos) -> Option<&T> {
        if self.is_valid(pos) {
            Some(&self.data[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    /// Get a mutable reference to a tile
    pub fn get_mut(&mut self, pos: TilePos) -> Option<&mut T> {
        if self.is_valid(pos) {
            Some(&mut self.data[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    /// Set a tile at position
    pub fn set(&mut self, pos: TilePos, value: T) -> bool {
        if self.is_valid(pos) {
            self.data[pos.y as usize][pos.x as usize] = value;
            true
        } else {
            false
        }
    }

    /// Get valid 4-way neighbors
    pub fn neighbors_4way(&self, pos: TilePos) -> Vec<TilePos> {
        pos.neighbors_4way()
            .into_iter()
            .filter(|p| self.is_valid(*p))
            .collect()
    }

    /// Get valid 8-way neighbors
    pub fn neighbors_8way(&self, pos: TilePos) -> Vec<TilePos> {
        pos.neighbors_8way()
            .into_iter()
            .filter(|p| self.is_valid(*p))
            .collect()
    }

    /// Iterate over all tiles with their positions
    pub fn iter_with_pos(&self) -> impl Iterator<Item = (TilePos, &T)> {
        self.data.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, tile)| (TilePos::new(x as i32, y as i32), tile))
        })
    }

    /// Iterate over all tiles mutably with their positions
    pub fn iter_mut_with_pos(&mut self) -> impl Iterator<Item = (TilePos, &mut T)> {
        self.data.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, tile)| (TilePos::new(x as i32, y as i32), tile))
        })
    }

    /// Get the underlying data
    pub fn data(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    /// Get mutable access to underlying data
    pub fn data_mut(&mut self) -> &mut Vec<Vec<T>> {
        &mut self.data
    }

    /// Fill the entire grid with a value
    pub fn fill(&mut self, value: T) {
        for row in &mut self.data {
            for cell in row {
                *cell = value.clone();
            }
        }
    }

    /// Fill a rectangular region
    pub fn fill_region(&mut self, min: TilePos, max: TilePos, value: T) {
        let min_x = min.x.max(0) as usize;
        let min_y = min.y.max(0) as usize;
        let max_x = (max.x as usize).min(self.width);
        let max_y = (max.y as usize).min(self.height);

        for y in min_y..max_y {
            for x in min_x..max_x {
                self.data[y][x] = value.clone();
            }
        }
    }
}

impl<T: Clone + Default> Grid<T> {
    /// Create a grid filled with the default value of T
    pub fn new_default(width: usize, height: usize) -> Self {
        Self::new(width, height, T::default())
    }
}

/// Flat vector grid storage for games that need cache-friendly iteration or
/// index-based persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatGrid<T> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone> FlatGrid<T> {
    /// Create a new flat grid filled with a default value.
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    /// Fill the entire grid with a value.
    pub fn fill(&mut self, value: T) {
        self.data.fill(value);
    }
}

impl<T> FlatGrid<T> {
    /// Create a flat grid from existing data.
    pub fn from_vec(width: usize, height: usize, data: Vec<T>) -> Result<Self, String> {
        if data.len() != width * height {
            return Err(format!(
                "FlatGrid data length {} does not match {}x{}",
                data.len(),
                width,
                height
            ));
        }

        Ok(Self {
            data,
            width,
            height,
        })
    }

    pub fn is_valid(&self, pos: TilePos) -> bool {
        pos.in_bounds(self.width, self.height)
    }

    pub fn index(&self, pos: TilePos) -> Option<usize> {
        if self.is_valid(pos) {
            pos.to_index(self.width)
        } else {
            None
        }
    }

    pub fn pos_from_index(&self, index: usize) -> Option<TilePos> {
        if index < self.data.len() {
            Some(TilePos::from_index(index, self.width))
        } else {
            None
        }
    }

    pub fn get(&self, pos: TilePos) -> Option<&T> {
        self.index(pos).and_then(|index| self.data.get(index))
    }

    pub fn get_mut(&mut self, pos: TilePos) -> Option<&mut T> {
        self.index(pos).and_then(|index| self.data.get_mut(index))
    }

    pub fn set(&mut self, pos: TilePos, value: T) -> bool {
        if let Some(index) = self.index(pos) {
            self.data[index] = value;
            true
        } else {
            false
        }
    }

    pub fn neighbors_4way(&self, pos: TilePos) -> Vec<TilePos> {
        pos.neighbors_4way()
            .into_iter()
            .filter(|neighbor| self.is_valid(*neighbor))
            .collect()
    }

    pub fn neighbors_8way(&self, pos: TilePos) -> Vec<TilePos> {
        pos.neighbors_8way()
            .into_iter()
            .filter(|neighbor| self.is_valid(*neighbor))
            .collect()
    }

    pub fn iter_with_pos(&self) -> impl Iterator<Item = (TilePos, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(move |(index, value)| (TilePos::from_index(index, self.width), value))
    }

    pub fn iter_mut_with_pos(&mut self) -> impl Iterator<Item = (TilePos, &mut T)> {
        let width = self.width;
        self.data
            .iter_mut()
            .enumerate()
            .map(move |(index, value)| (TilePos::from_index(index, width), value))
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Find an unweighted shortest path across this grid.
    pub fn bfs_path<F>(
        &self,
        start: TilePos,
        goal: TilePos,
        allow_diagonals: bool,
        can_enter: F,
    ) -> Option<Vec<TilePos>>
    where
        F: Fn(TilePos, &T) -> bool,
    {
        bfs_path(
            start,
            goal,
            allow_diagonals,
            |pos| self.is_valid(pos),
            |pos| self.get(pos).is_some_and(|tile| can_enter(pos, tile)),
        )
    }

    /// Return all positions reachable from `start`.
    pub fn flood_fill<F>(
        &self,
        start: TilePos,
        allow_diagonals: bool,
        can_enter: F,
    ) -> HashSet<TilePos>
    where
        F: Fn(TilePos, &T) -> bool,
    {
        flood_fill(
            start,
            allow_diagonals,
            |pos| self.is_valid(pos),
            |pos| self.get(pos).is_some_and(|tile| can_enter(pos, tile)),
        )
    }
}

impl<T: Clone + Default> FlatGrid<T> {
    /// Create a grid filled with the default value of T.
    pub fn new_default(width: usize, height: usize) -> Self {
        Self::new(width, height, T::default())
    }
}

// Coordinate conversion functions

/// Convert world coordinates to isometric screen coordinates
pub fn world_to_iso(x: f32, y: f32, tile_width: f32, tile_height: f32) -> (f32, f32) {
    let iso_x = (x - y) * tile_width / 2.0;
    let iso_y = (x + y) * tile_height / 2.0;
    (iso_x, iso_y)
}

/// Convert isometric screen coordinates to world coordinates
pub fn iso_to_world(iso_x: f32, iso_y: f32, tile_width: f32, tile_height: f32) -> (f32, f32) {
    let x = (iso_x / (tile_width / 2.0) + iso_y / (tile_height / 2.0)) / 2.0;
    let y = (iso_y / (tile_height / 2.0) - iso_x / (tile_width / 2.0)) / 2.0;
    (x, y)
}

/// Line of sight check using Bresenham's line algorithm
///
/// Returns true if there is a clear line of sight between two positions.
/// The `blocks_vision` function should return true for positions that block vision.
pub fn has_line_of_sight<F>(from: TilePos, to: TilePos, blocks_vision: F) -> bool
where
    F: Fn(TilePos) -> bool,
{
    let mut x0 = from.x;
    let mut y0 = from.y;
    let x1 = to.x;
    let y1 = to.y;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        // Check if current tile blocks vision (skip the start position)
        if (x0, y0) != (from.x, from.y) {
            let pos = TilePos::new(x0, y0);
            if blocks_vision(pos) {
                return false;
            }
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }

    true
}

/// Get all positions along a line using Bresenham's algorithm
pub fn line_positions(from: TilePos, to: TilePos) -> Vec<TilePos> {
    let mut positions = Vec::new();
    let mut x0 = from.x;
    let mut y0 = from.y;
    let x1 = to.x;
    let y1 = to.y;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        positions.push(TilePos::new(x0, y0));

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }

    positions
}

/// Calculate visible tiles from a position given a sight radius
///
/// Returns a set of all positions visible from `center` within `radius`,
/// using the `blocks_vision` function to determine which tiles block sight.
pub fn calculate_visible_tiles<F>(
    center: TilePos,
    radius: i32,
    blocks_vision: F,
) -> HashSet<TilePos>
where
    F: Fn(TilePos) -> bool,
{
    let mut visible = HashSet::new();

    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let distance_sq = dx * dx + dy * dy;
            if distance_sq <= radius * radius {
                let target = TilePos::new(center.x + dx, center.y + dy);
                if has_line_of_sight(center, target, &blocks_vision) {
                    visible.insert(target);
                }
            }
        }
    }

    visible
}

fn neighbor_positions(pos: TilePos, allow_diagonals: bool) -> Vec<TilePos> {
    if allow_diagonals {
        pos.neighbors_8way().to_vec()
    } else {
        pos.neighbors_4way().to_vec()
    }
}

/// Find an unweighted shortest path using breadth-first search.
pub fn bfs_path<FValid, FEnter>(
    start: TilePos,
    goal: TilePos,
    allow_diagonals: bool,
    is_valid: FValid,
    can_enter: FEnter,
) -> Option<Vec<TilePos>>
where
    FValid: Fn(TilePos) -> bool,
    FEnter: Fn(TilePos) -> bool,
{
    if !is_valid(start) || !is_valid(goal) || !can_enter(start) || !can_enter(goal) {
        return None;
    }

    if start == goal {
        return Some(vec![start]);
    }

    let mut frontier = VecDeque::new();
    let mut came_from: HashMap<TilePos, TilePos> = HashMap::new();
    let mut visited = HashSet::new();

    frontier.push_back(start);
    visited.insert(start);

    while let Some(current) = frontier.pop_front() {
        for neighbor in neighbor_positions(current, allow_diagonals) {
            if visited.contains(&neighbor) || !is_valid(neighbor) || !can_enter(neighbor) {
                continue;
            }

            came_from.insert(neighbor, current);

            if neighbor == goal {
                let mut path = vec![goal];
                let mut cursor = goal;
                while let Some(parent) = came_from.get(&cursor).copied() {
                    path.push(parent);
                    cursor = parent;
                    if cursor == start {
                        break;
                    }
                }
                path.reverse();
                return Some(path);
            }

            visited.insert(neighbor);
            frontier.push_back(neighbor);
        }
    }

    None
}

/// Return every tile reachable from `start`.
pub fn flood_fill<FValid, FEnter>(
    start: TilePos,
    allow_diagonals: bool,
    is_valid: FValid,
    can_enter: FEnter,
) -> HashSet<TilePos>
where
    FValid: Fn(TilePos) -> bool,
    FEnter: Fn(TilePos) -> bool,
{
    let mut visited = HashSet::new();

    if !is_valid(start) || !can_enter(start) {
        return visited;
    }

    let mut frontier = VecDeque::new();
    frontier.push_back(start);
    visited.insert(start);

    while let Some(current) = frontier.pop_front() {
        for neighbor in neighbor_positions(current, allow_diagonals) {
            if visited.contains(&neighbor) || !is_valid(neighbor) || !can_enter(neighbor) {
                continue;
            }
            visited.insert(neighbor);
            frontier.push_back(neighbor);
        }
    }

    visited
}

/// Return reachable tiles and their step distance from `start`.
pub fn reachable_within<FValid, FEnter>(
    start: TilePos,
    max_steps: usize,
    allow_diagonals: bool,
    is_valid: FValid,
    can_enter: FEnter,
) -> HashMap<TilePos, usize>
where
    FValid: Fn(TilePos) -> bool,
    FEnter: Fn(TilePos) -> bool,
{
    let mut distances = HashMap::new();

    if !is_valid(start) || !can_enter(start) {
        return distances;
    }

    let mut frontier = VecDeque::new();
    frontier.push_back(start);
    distances.insert(start, 0);

    while let Some(current) = frontier.pop_front() {
        let current_distance = distances[&current];
        if current_distance >= max_steps {
            continue;
        }

        for neighbor in neighbor_positions(current, allow_diagonals) {
            if distances.contains_key(&neighbor) || !is_valid(neighbor) || !can_enter(neighbor) {
                continue;
            }
            distances.insert(neighbor, current_distance + 1);
            frontier.push_back(neighbor);
        }
    }

    distances
}

/// Tiles in a circular radius without line-of-sight checks.
pub fn tiles_in_radius(center: TilePos, radius: i32) -> HashSet<TilePos> {
    let mut tiles = HashSet::new();

    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy <= radius * radius {
                tiles.insert(TilePos::new(center.x + dx, center.y + dy));
            }
        }
    }

    tiles
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
