//! Flat vector grid storage with index-based access.

use super::{bfs_path, flood_fill, TilePos};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
