//! Input utilities for mouse and keyboard handling

use macroquad::prelude::*;

/// Current mouse position as a `Vec2`.
pub fn mouse_position_vec2() -> Vec2 {
    let (mx, my) = mouse_position();
    vec2(mx, my)
}

/// Check if a point is inside a rectangle
pub fn rect_contains(x: f32, y: f32, w: f32, h: f32, px: f32, py: f32) -> bool {
    px >= x && px <= x + w && py >= y && py <= y + h
}

/// Check if mouse is hovering over a rectangle
pub fn is_hovered(x: f32, y: f32, w: f32, h: f32) -> bool {
    let (mx, my) = mouse_position();
    rect_contains(x, y, w, h, mx, my)
}

/// Check if a point is inside a macroquad `Rect`.
pub fn rect_contains_point(rect: Rect, point: Vec2) -> bool {
    rect_contains(rect.x, rect.y, rect.w, rect.h, point.x, point.y)
}

/// Check if mouse is hovering over a macroquad `Rect`.
pub fn is_hovered_rect(rect: Rect) -> bool {
    let (mx, my) = mouse_position();
    rect_contains_point(rect, vec2(mx, my))
}

/// Alias for is_hovered - check if mouse is over a rectangle
pub fn is_mouse_over(x: f32, y: f32, w: f32, h: f32) -> bool {
    is_hovered(x, y, w, h)
}

/// Check if a rectangle was clicked (mouse button released over it)
pub fn was_clicked(x: f32, y: f32, w: f32, h: f32) -> bool {
    is_hovered(x, y, w, h) && is_mouse_button_released(MouseButton::Left)
}

/// Check if a rectangle was clicked (mouse button released over it).
pub fn was_clicked_rect(rect: Rect) -> bool {
    is_hovered_rect(rect) && is_mouse_button_released(MouseButton::Left)
}

/// Check if a rectangle was pressed (mouse button pressed down over it)
pub fn was_pressed(x: f32, y: f32, w: f32, h: f32) -> bool {
    is_hovered(x, y, w, h) && is_mouse_button_pressed(MouseButton::Left)
}

/// Check if a rectangle was pressed (mouse button pressed down over it).
pub fn was_pressed_rect(rect: Rect) -> bool {
    is_hovered_rect(rect) && is_mouse_button_pressed(MouseButton::Left)
}

/// Check if a rectangle was right-clicked (right mouse button released over it)
pub fn was_right_clicked(x: f32, y: f32, w: f32, h: f32) -> bool {
    is_hovered(x, y, w, h) && is_mouse_button_released(MouseButton::Right)
}

/// Check if a rectangle was right-clicked (right mouse button released over it).
pub fn was_right_clicked_rect(rect: Rect) -> bool {
    is_hovered_rect(rect) && is_mouse_button_released(MouseButton::Right)
}

/// Captures current input state for processing
#[derive(Debug, Clone)]
pub struct InputState {
    pub mouse_pos: Vec2,
    /// Compatibility alias for `left_pressed`.
    pub left_click: bool,
    /// Compatibility alias for `right_pressed`.
    pub right_click: bool,
    pub left_pressed: bool,
    pub left_released: bool,
    pub left_down: bool,
    pub right_pressed: bool,
    pub right_released: bool,
    pub right_down: bool,
    pub escape_pressed: bool,
    pub enter_pressed: bool,
    pub space_pressed: bool,
}

impl InputState {
    /// Capture current frame's input state
    pub fn capture() -> Self {
        let mouse_pos = mouse_position_vec2();
        let left_pressed = is_mouse_button_pressed(MouseButton::Left);
        let right_pressed = is_mouse_button_pressed(MouseButton::Right);
        Self {
            mouse_pos,
            left_click: left_pressed,
            right_click: right_pressed,
            left_pressed,
            left_released: is_mouse_button_released(MouseButton::Left),
            left_down: is_mouse_button_down(MouseButton::Left),
            right_pressed,
            right_released: is_mouse_button_released(MouseButton::Right),
            right_down: is_mouse_button_down(MouseButton::Right),
            escape_pressed: is_key_pressed(KeyCode::Escape),
            enter_pressed: is_key_pressed(KeyCode::Enter),
            space_pressed: is_key_pressed(KeyCode::Space),
        }
    }

    /// Check whether the captured mouse position is inside a `Rect`.
    pub fn hovered_rect(&self, rect: Rect) -> bool {
        rect_contains_point(rect, self.mouse_pos)
    }

    /// Check whether the left mouse button was pressed over a `Rect`.
    pub fn left_pressed_rect(&self, rect: Rect) -> bool {
        self.left_pressed && self.hovered_rect(rect)
    }

    /// Check whether the left mouse button was released over a `Rect`.
    pub fn left_released_rect(&self, rect: Rect) -> bool {
        self.left_released && self.hovered_rect(rect)
    }

    /// Check whether the right mouse button was pressed over a `Rect`.
    pub fn right_pressed_rect(&self, rect: Rect) -> bool {
        self.right_pressed && self.hovered_rect(rect)
    }

    /// Check whether the right mouse button was released over a `Rect`.
    pub fn right_released_rect(&self, rect: Rect) -> bool {
        self.right_released && self.hovered_rect(rect)
    }
}
