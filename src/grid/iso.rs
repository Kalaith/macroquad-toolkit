//! Isometric coordinate conversion.

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
