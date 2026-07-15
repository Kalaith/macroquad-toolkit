//! Line of sight, visibility, and radius queries.

use super::TilePos;
use std::collections::HashSet;

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
