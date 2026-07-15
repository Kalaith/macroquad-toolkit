//! Breadth-first pathfinding and flood fill over tile grids.

use super::TilePos;
use std::collections::{HashMap, HashSet, VecDeque};

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
