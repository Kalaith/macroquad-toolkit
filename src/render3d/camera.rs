//! 3D Camera systems for isometric and perspective views
//!
//! Provides camera state management with smooth zoom, predefined zoom levels,
//! and easy Camera3D integration.

use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

/// 3D isometric-style camera with smooth zoom transitions
///
/// Features:
/// - Predefined zoom levels for consistent user experience
/// - Smooth zoom interpolation
/// - Easy conversion to macroquad's Camera3D
///
/// # Example
/// ```no_run
/// use macroquad_toolkit::render3d::camera::IsometricCamera;
/// use macroquad::prelude::*;
///
/// let mut camera = IsometricCamera::new(50.0, 50.0);
///
/// // In game loop:
/// camera.update(get_frame_time());
/// set_camera(&camera.get_camera3d());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsometricCamera {
    /// Camera target position (x, y, z)
    pub target: (f32, f32, f32),
    /// Current distance from target
    pub distance: f32,
    /// Target distance for smooth zoom
    pub target_distance: f32,
    /// Rotation angle around Y axis
    pub angle: f32,
    /// Additional zoom multiplier
    pub zoom: f32,
    /// Current zoom level index
    pub zoom_index: usize,
}

impl IsometricCamera {
    /// Predefined zoom distance levels
    pub const ZOOM_LEVELS: &'static [f32] = &[5.0, 8.0, 12.0, 16.0, 20.0, 25.0, 32.0, 40.0, 50.0];

    /// Create a new isometric camera centered on the given world position
    ///
    /// # Parameters
    /// - `center_x`: X coordinate of map center
    /// - `center_z`: Z coordinate of map center (depth axis)
    pub fn new(center_x: f32, center_z: f32) -> Self {
        // Find index of default distance (20.0)
        let default_dist = 20.0;
        let index = Self::ZOOM_LEVELS
            .iter()
            .position(|&d| d == default_dist)
            .unwrap_or(4);

        Self {
            target: (center_x, 0.0, center_z),
            distance: default_dist,
            target_distance: default_dist,
            angle: 0.0,
            zoom: 1.0,
            zoom_index: index,
        }
    }

    /// Create camera with custom zoom levels
    pub fn with_distance(center_x: f32, center_z: f32, distance: f32) -> Self {
        let index = Self::ZOOM_LEVELS
            .iter()
            .position(|&d| d >= distance)
            .unwrap_or(Self::ZOOM_LEVELS.len() - 1);

        Self {
            target: (center_x, 0.0, center_z),
            distance,
            target_distance: distance,
            angle: 0.0,
            zoom: 1.0,
            zoom_index: index,
        }
    }

    /// Zoom in to the next closer zoom level
    pub fn zoom_in(&mut self) {
        if self.zoom_index > 0 {
            self.zoom_index -= 1;
            self.target_distance = Self::ZOOM_LEVELS[self.zoom_index];
        }
    }

    /// Zoom out to the next further zoom level
    pub fn zoom_out(&mut self) {
        if self.zoom_index < Self::ZOOM_LEVELS.len() - 1 {
            self.zoom_index += 1;
            self.target_distance = Self::ZOOM_LEVELS[self.zoom_index];
        }
    }

    /// Set zoom to a specific level index
    pub fn set_zoom_level(&mut self, index: usize) {
        if index < Self::ZOOM_LEVELS.len() {
            self.zoom_index = index;
            self.target_distance = Self::ZOOM_LEVELS[index];
        }
    }

    /// Get the current zoom level index
    pub fn get_zoom_level(&self) -> usize {
        self.zoom_index
    }

    /// Move camera target to a new position
    pub fn set_target(&mut self, x: f32, y: f32, z: f32) {
        self.target = (x, y, z);
    }

    /// Pan the camera by a delta amount
    pub fn pan(&mut self, dx: f32, dz: f32) {
        self.target.0 += dx;
        self.target.2 += dz;
    }

    /// Rotate camera around Y axis
    pub fn rotate(&mut self, delta_angle: f32) {
        self.angle += delta_angle;
    }

    /// Update camera (call every frame for smooth zoom)
    pub fn update(&mut self, dt: f32) {
        // Smooth zoom interpolation
        let lerp_factor = 5.0 * dt;
        self.distance += (self.target_distance - self.distance) * lerp_factor.clamp(0.0, 1.0);
    }

    /// Get the camera's world position
    pub fn get_position(&self) -> Vec3 {
        vec3(
            self.target.0
                + (self.distance * 0.5) * (self.angle + std::f32::consts::FRAC_PI_2).cos(),
            self.distance,
            self.target.2
                + (self.distance * 0.5) * (self.angle + std::f32::consts::FRAC_PI_2).sin(),
        )
    }

    /// Convert to macroquad's Camera3D for rendering
    pub fn get_camera3d(&self) -> Camera3D {
        Camera3D {
            position: self.get_position(),
            target: vec3(self.target.0, 0.0, self.target.2),
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0f32.to_radians(),
            projection: Projection::Perspective,
            aspect: Some(screen_width() / screen_height()),
            ..Default::default()
        }
    }
}

impl Default for IsometricCamera {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

/// Simple 3D orbit camera for general purpose use
#[derive(Debug, Clone)]
pub struct OrbitCamera {
    /// Target position the camera looks at
    pub target: Vec3,
    /// Distance from target
    pub distance: f32,
    /// Horizontal rotation (yaw) in radians
    pub yaw: f32,
    /// Vertical rotation (pitch) in radians
    pub pitch: f32,
    /// Field of view in degrees
    pub fov: f32,
}

impl OrbitCamera {
    /// Create a new orbit camera
    pub fn new(target: Vec3, distance: f32) -> Self {
        Self {
            target,
            distance,
            yaw: 0.0,
            pitch: 0.5, // About 30 degrees
            fov: 45.0,
        }
    }

    /// Get the camera's world position
    pub fn get_position(&self) -> Vec3 {
        let x = self.target.x + self.distance * self.pitch.cos() * self.yaw.sin();
        let y = self.target.y + self.distance * self.pitch.sin();
        let z = self.target.z + self.distance * self.pitch.cos() * self.yaw.cos();
        vec3(x, y, z)
    }

    /// Convert to macroquad's Camera3D
    pub fn get_camera3d(&self) -> Camera3D {
        Camera3D {
            position: self.get_position(),
            target: self.target,
            up: vec3(0.0, 1.0, 0.0),
            fovy: self.fov.to_radians(),
            projection: Projection::Perspective,
            aspect: Some(screen_width() / screen_height()),
            ..Default::default()
        }
    }

    /// Rotate the camera
    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch = (self.pitch + delta_pitch).clamp(0.1, std::f32::consts::FRAC_PI_2 - 0.1);
    }

    /// Zoom the camera
    pub fn zoom(&mut self, factor: f32) {
        self.distance = (self.distance * factor).clamp(1.0, 100.0);
    }
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self::new(Vec3::ZERO, 10.0)
    }
}
