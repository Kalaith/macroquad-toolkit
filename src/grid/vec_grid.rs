//! Row-major `Vec<Vec<T>>` grid storage.

use super::TilePos;
use serde::{Deserialize, Serialize};

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
