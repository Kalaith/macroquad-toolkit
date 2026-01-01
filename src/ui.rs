//! UI rendering utilities for buttons, panels, and progress bars

use macroquad::prelude::*;
use crate::input::*;
use crate::colors::dark;

/// Style configuration for buttons
#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
    pub border: Color,
    pub text_color: Color,
}

impl ButtonStyle {
    /// Default dark theme button style
    pub fn default_dark() -> Self {
        Self {
            normal: dark::PANEL,
            hovered: dark::HOVERED,
            pressed: Color::new(0.25, 0.35, 0.5, 1.0),
            border: dark::ACCENT,
            text_color: dark::TEXT,
        }
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self::default_dark()
    }
}

/// Draw a button with default styling. Returns true if clicked (on mouse release).
pub fn button(x: f32, y: f32, w: f32, h: f32, text: &str) -> bool {
    button_styled(x, y, w, h, text, &ButtonStyle::default())
}

/// Draw a button with custom styling. Returns true if clicked (on mouse release).
pub fn button_styled(x: f32, y: f32, w: f32, h: f32, text: &str, style: &ButtonStyle) -> bool {
    button_on_release(x, y, w, h, text, style)
}

/// Draw a button that triggers on mouse press (button down).
/// Returns true when mouse button is pressed down over the button.
pub fn button_on_press(x: f32, y: f32, w: f32, h: f32, text: &str, style: &ButtonStyle) -> bool {
    let hovered = is_hovered(x, y, w, h);
    let is_pressed = hovered && is_mouse_button_down(MouseButton::Left);
    let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);

    // Determine button color
    let bg_color = if is_pressed {
        style.pressed
    } else if hovered {
        style.hovered
    } else {
        style.normal
    };

    // Draw background
    draw_rectangle(x, y, w, h, bg_color);
    draw_rectangle_lines(x, y, w, h, 2.0, style.border);

    // Text offset for press effect
    let y_offset = if is_pressed { 2.0 } else { 0.0 };

    // Center text
    let text_size = 20.0;
    let text_width = measure_text(text, None, text_size as u16, 1.0).width;
    draw_text(
        text,
        x + (w - text_width) / 2.0,
        y + h / 2.0 + 6.0 + y_offset,
        text_size,
        style.text_color,
    );

    clicked
}

/// Draw a button that triggers on mouse release (button up).
/// Returns true when mouse button is released over the button.
/// This is the safer default as it prevents accidental double-clicks.
pub fn button_on_release(x: f32, y: f32, w: f32, h: f32, text: &str, style: &ButtonStyle) -> bool {
    let hovered = is_hovered(x, y, w, h);
    let is_pressed = hovered && is_mouse_button_down(MouseButton::Left);
    let clicked = hovered && is_mouse_button_released(MouseButton::Left);

    // Determine button color
    let bg_color = if is_pressed {
        style.pressed
    } else if hovered {
        style.hovered
    } else {
        style.normal
    };

    // Draw background
    draw_rectangle(x, y, w, h, bg_color);
    draw_rectangle_lines(x, y, w, h, 2.0, style.border);

    // Text offset for press effect
    let y_offset = if is_pressed { 2.0 } else { 0.0 };

    // Center text
    let text_size = 20.0;
    let text_width = measure_text(text, None, text_size as u16, 1.0).width;
    draw_text(
        text,
        x + (w - text_width) / 2.0,
        y + h / 2.0 + 6.0 + y_offset,
        text_size,
        style.text_color,
    );

    clicked
}

/// Draw a panel with optional title
pub fn panel(x: f32, y: f32, w: f32, h: f32, title: Option<&str>) {
    // Background
    draw_rectangle(x, y, w, h, dark::PANEL);

    // Header (if title provided)
    if let Some(title) = title {
        draw_rectangle(x, y, w, 30.0, dark::PANEL_HEADER);
        draw_text(title, x + 10.0, y + 22.0, 20.0, dark::TEXT);
    }

    // Border
    draw_rectangle_lines(x, y, w, h, 1.0, dark::TEXT_DIM);
}

/// Draw a panel with shadow effect
pub fn panel_with_shadow(x: f32, y: f32, w: f32, h: f32) {
    // Shadow
    draw_rectangle(x + 4.0, y + 4.0, w, h, Color::new(0.0, 0.0, 0.0, 0.5));

    // Panel background
    draw_rectangle(x, y, w, h, dark::PANEL);

    // Borders
    draw_rectangle_lines(x, y, w, h, 2.0, dark::TEXT_DIM);
    draw_rectangle_lines(
        x + 2.0,
        y + 2.0,
        w - 4.0,
        h - 4.0,
        1.0,
        Color::new(0.2, 0.2, 0.22, 0.5),
    );
}

/// Draw a progress bar
pub fn progress_bar(x: f32, y: f32, w: f32, h: f32, value: f32, max: f32, color: Color) {
    let fill_width = (value / max).clamp(0.0, 1.0) * w;

    // Background
    draw_rectangle(x, y, w, h, Color::new(0.15, 0.15, 0.15, 1.0));

    // Fill
    draw_rectangle(x, y, fill_width, h, color);

    // Border
    draw_rectangle_lines(x, y, w, h, 1.0, dark::TEXT_DIM);
}

/// Draw a progress bar with centered label
pub fn progress_bar_labeled(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    value: f32,
    max: f32,
    label: &str,
    color: Color,
) {
    progress_bar(x, y, w, h, value, max, color);

    // Draw label centered
    let text_size = 16.0;
    let text_width = measure_text(label, None, text_size as u16, 1.0).width;
    draw_text(
        label,
        x + (w - text_width) / 2.0,
        y + h / 2.0 + 5.0,
        text_size,
        dark::TEXT,
    );
}
