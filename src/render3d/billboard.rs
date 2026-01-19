//! 3D Billboard rendering for 2D sprites in 3D space
//!
//! Provides Y-axis cylindrical billboarding for rendering 2D sprites
//! that always face the camera in 3D environments.

use macroquad::prelude::*;

/// Draw a billboard sprite in 3D space that always faces the camera (Y-axis rotation only)
///
/// This is useful for rendering 2D sprites in isometric or 3D games where
/// sprites should always face the camera horizontally.
///
/// # Parameters
/// - `pos`: World position of the billboard center
/// - `size`: Size of the billboard (width, height)
/// - `texture`: The texture to render
/// - `camera_pos`: The camera's world position
///
/// # Example
/// ```no_run
/// use macroquad::prelude::*;
/// use macroquad_toolkit::render3d::billboard::draw_billboard;
///
/// # async fn example() {
/// # let texture = load_texture("sprite.png").await.unwrap();
/// let camera_pos = vec3(10.0, 20.0, 10.0);
/// let sprite_pos = vec3(5.0, 0.0, 5.0);
/// draw_billboard(sprite_pos, vec2(1.0, 2.0), &texture, camera_pos);
/// # }
/// ```
pub fn draw_billboard(pos: Vec3, size: Vec2, texture: &Texture2D, camera_pos: Vec3) {
    let to_cam = camera_pos - pos;
    // Y-axis billboarding (cylindrical)
    let dist_sq = to_cam.x * to_cam.x + to_cam.z * to_cam.z;

    // Robustness: If strictly above (dist_sq ~ 0), default to looking South (Z+) or arbitrary valid vector
    let fwd = if dist_sq < 0.001 {
        vec3(0.0, 0.0, 1.0)
    } else {
        vec3(to_cam.x, 0.0, to_cam.z).normalize()
    };

    let right = vec3(0.0, 1.0, 0.0).cross(fwd).normalize();
    let up = vec3(0.0, 1.0, 0.0);

    let half_w = size.x * 0.5;
    let half_h = size.y * 0.5;

    // Top Left
    let v1 = pos - right * half_w + up * half_h;
    // Top Right
    let v2 = pos + right * half_w + up * half_h;
    // Bottom Right
    let v3 = pos + right * half_w - up * half_h;
    // Bottom Left
    let v4 = pos - right * half_w - up * half_h;

    let mesh = Mesh {
        vertices: vec![
            Vertex { position: v1, uv: vec2(0.0, 0.0), color: WHITE.into(), normal: vec4(fwd.x, fwd.y, fwd.z, 0.0) },
            Vertex { position: v2, uv: vec2(1.0, 0.0), color: WHITE.into(), normal: vec4(fwd.x, fwd.y, fwd.z, 0.0) },
            Vertex { position: v3, uv: vec2(1.0, 1.0), color: WHITE.into(), normal: vec4(fwd.x, fwd.y, fwd.z, 0.0) },
            Vertex { position: v4, uv: vec2(0.0, 1.0), color: WHITE.into(), normal: vec4(fwd.x, fwd.y, fwd.z, 0.0) },
        ],
        // Double sided: draw CW and CCW
        indices: vec![
            0, 1, 2, 0, 2, 3, // Front
            0, 2, 1, 0, 3, 2  // Back
        ],
        texture: Some(texture.clone()),
    };

    draw_mesh(&mesh);
}

/// Draw a tinted billboard sprite in 3D space
///
/// Same as `draw_billboard` but with color tinting support.
pub fn draw_billboard_tinted(pos: Vec3, size: Vec2, texture: &Texture2D, camera_pos: Vec3, color: Color) {
    let to_cam = camera_pos - pos;
    let dist_sq = to_cam.x * to_cam.x + to_cam.z * to_cam.z;

    let fwd = if dist_sq < 0.001 {
        vec3(0.0, 0.0, 1.0)
    } else {
        vec3(to_cam.x, 0.0, to_cam.z).normalize()
    };

    let right = vec3(0.0, 1.0, 0.0).cross(fwd).normalize();
    let up = vec3(0.0, 1.0, 0.0);

    let half_w = size.x * 0.5;
    let half_h = size.y * 0.5;

    let v1 = pos - right * half_w + up * half_h;
    let v2 = pos + right * half_w + up * half_h;
    let v3 = pos + right * half_w - up * half_h;
    let v4 = pos - right * half_w - up * half_h;

    let mesh = Mesh {
        vertices: vec![
            Vertex { position: v1, uv: vec2(0.0, 0.0), color: color.into(), normal: vec4(fwd.x, fwd.y, fwd.z, 0.0) },
            Vertex { position: v2, uv: vec2(1.0, 0.0), color: color.into(), normal: vec4(fwd.x, fwd.y, fwd.z, 0.0) },
            Vertex { position: v3, uv: vec2(1.0, 1.0), color: color.into(), normal: vec4(fwd.x, fwd.y, fwd.z, 0.0) },
            Vertex { position: v4, uv: vec2(0.0, 1.0), color: color.into(), normal: vec4(fwd.x, fwd.y, fwd.z, 0.0) },
        ],
        indices: vec![
            0, 1, 2, 0, 2, 3,
            0, 2, 1, 0, 3, 2
        ],
        texture: Some(texture.clone()),
    };

    draw_mesh(&mesh);
}
