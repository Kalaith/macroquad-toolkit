//! 2D camera with pan and zoom functionality

use macroquad::prelude::*;

/// 2D camera for panning and zooming in world space
///
/// # Example
/// ```no_run
/// use macroquad_toolkit::camera::Camera2D;
/// use macroquad::prelude::*;
///
/// let mut camera = Camera2D::new(vec2(0.0, 0.0), 1.0);
///
/// // In game loop:
/// camera.update(get_frame_time(), false);
///
/// // Convert between screen and world coordinates
/// let world_pos = camera.screen_to_world(vec2(100.0, 100.0));
/// let screen_pos = camera.world_to_screen(vec2(50.0, 50.0));
/// ```
#[derive(Debug, Clone)]
pub struct Camera2D {
    pub target: Vec2,
    pub zoom: f32,

    // Drag state
    drag_start: Option<Vec2>,
    cam_start: Vec2,
}

impl Camera2D {
    /// Create a new camera with the given target position and zoom level
    pub fn new(target: Vec2, zoom: f32) -> Self {
        Self {
            target,
            zoom,
            drag_start: None,
            cam_start: Vec2::ZERO,
        }
    }

    /// Convert screen coordinates to world coordinates
    pub fn screen_to_world(&self, point: Vec2) -> Vec2 {
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let local = point - center;
        (local / self.zoom) + self.target
    }

    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, point: Vec2) -> Vec2 {
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let local = point - self.target;
        (local * self.zoom) + center
    }

    /// Pan the camera by the given delta in world space
    pub fn pan(&mut self, delta: Vec2) {
        self.target += delta;
    }

    /// Zoom the camera at the given screen point
    pub fn zoom_at(&mut self, factor: f32, screen_point: Vec2) {
        let world_before = self.screen_to_world(screen_point);
        self.zoom *= factor;
        self.zoom = self.zoom.clamp(0.1, 10.0);
        let world_after = self.screen_to_world(screen_point);
        self.target += world_before - world_after;
    }

    /// Update camera with input handling
    ///
    /// # Parameters
    /// - `delta`: Frame time in seconds
    /// - `input_captured`: If true, disables mouse input (e.g., when UI is being interacted with)
    pub fn update(&mut self, delta: f32, input_captured: bool) {
        let speed = 500.0 / self.zoom;

        // Keyboard Pan (WASD + Arrow keys)
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.target.y -= speed * delta;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.target.y += speed * delta;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.target.x -= speed * delta;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.target.x += speed * delta;
        }

        // Mouse Input (only if not interacting with UI)
        if !input_captured {
            // Mouse Drag
            if is_mouse_button_pressed(MouseButton::Left) {
                self.drag_start = Some(mouse_position().into());
                self.cam_start = self.target;
            }

            if is_mouse_button_down(MouseButton::Left) {
                if let Some(start) = self.drag_start {
                    let current: Vec2 = mouse_position().into();
                    let screen_delta = current - start;
                    let world_delta = screen_delta / self.zoom;
                    self.target = self.cam_start - world_delta;
                }
            }

            // Mouse Wheel Zoom
            let (_, wheel_y) = mouse_wheel();
            if wheel_y != 0.0 {
                let zoom_factor = if wheel_y > 0.0 { 1.1 } else { 0.9 };
                self.zoom *= zoom_factor;
            }
        }

        // Reset drag when button released
        if is_mouse_button_released(MouseButton::Left) {
            self.drag_start = None;
        }

        // Keyboard Zoom (+/- keys)
        if is_key_down(KeyCode::Equal) {
            self.zoom *= 1.0 + delta;
        }
        if is_key_down(KeyCode::Minus) {
            self.zoom *= 1.0 - delta;
        }

        // Clamp zoom
        self.zoom = self.zoom.clamp(0.1, 10.0);
    }
}

impl Default for Camera2D {
    fn default() -> Self {
        Self::new(Vec2::ZERO, 1.0)
    }
}
