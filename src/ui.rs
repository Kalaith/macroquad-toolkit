//! UI rendering utilities for buttons, panels, and progress bars

use macroquad::prelude::*;
use crate::input::*;
use crate::colors::dark;

#[derive(Debug, Clone)]
pub struct TextLayoutResult {
    pub lines: Vec<String>,
    pub font_size: f32,
    pub truncated: bool,
}

/// Style configuration for buttons
#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
    pub border: Color,
    pub text_color: Color,
    pub disabled: Color,
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
            disabled: Color::new(0.1, 0.1, 0.1, 1.0),
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

    draw_text_centered_in_box(
        text,
        x + 8.0,
        y + y_offset,
        w - 16.0,
        h,
        20.0,
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

    draw_text_centered_in_box(
        text,
        x + 8.0,
        y + y_offset,
        w - 16.0,
        h,
        20.0,
        style.text_color,
    );

    clicked
}

/// Draw a button with explicit colors (simplified wrapper)
pub fn colored_button(x: f32, y: f32, w: f32, h: f32, text: &str, color: Color) -> bool {
    let style = ButtonStyle {
        normal: color,
        hovered: Color::new(color.r * 1.2, color.g * 1.2, color.b * 1.2, color.a),
        pressed: Color::new(color.r * 0.8, color.g * 0.8, color.b * 0.8, color.a),
        border: dark::TEXT_DIM,
        text_color: dark::TEXT_BRIGHT,
        ..ButtonStyle::default()
    };
    button_on_release(x, y, w, h, text, &style)
}

/// Draw a simple window/modal frame
pub fn window(x: f32, y: f32, w: f32, h: f32, title: Option<&str>, close_button: bool) -> bool {
    // Shadow
    draw_rectangle(x + 4.0, y + 4.0, w, h, Color::new(0.0, 0.0, 0.0, 0.5));
    
    // Main body
    draw_rectangle(x, y, w, h, dark::PANEL);
    draw_rectangle_lines(x, y, w, h, 2.0, dark::ACCENT);
    
    // Header
    if let Some(t) = title {
        draw_rectangle(x, y, w, 30.0, dark::PANEL_HEADER);
        draw_text_centered_in_box(t, x + 10.0, y + 2.0, w - 20.0, 26.0, 20.0, dark::TEXT);
    }
    
    // Close button
    if close_button {
        let btn_size = 24.0;
        let btn_x = x + w - btn_size - 3.0;
        let btn_y = y + 3.0;
        
        let style = ButtonStyle {
            normal: dark::NEGATIVE,
            hovered: Color::new(0.9, 0.4, 0.4, 1.0),
            pressed: Color::new(0.7, 0.2, 0.2, 1.0),
            border: dark::TEXT,
            text_color: dark::TEXT_BRIGHT,
            ..ButtonStyle::default()
        };
        
        if button_on_release(btn_x, btn_y, btn_size, btn_size, "X", &style) {
            return true;
        }
    }
    
    false
}

/// Draw a panel with optional title
pub fn panel(x: f32, y: f32, w: f32, h: f32, title: Option<&str>) {
    // Background
    draw_rectangle(x, y, w, h, dark::PANEL);

    // Header (if title provided)
    if let Some(title) = title {
        draw_rectangle(x, y, w, 30.0, dark::PANEL_HEADER);
        draw_text_centered_in_box(title, x + 10.0, y + 2.0, w - 20.0, 26.0, 20.0, dark::TEXT);
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

#[allow(clippy::too_many_arguments)]
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

    draw_text_centered_in_box(label, x + 6.0, y, w - 12.0, h, 16.0, dark::TEXT);
}

/// Draw a section panel with title header - common for UI sections
pub fn section_panel(x: f32, y: f32, w: f32, h: f32, title: &str) {
    // Background
    draw_rectangle(x, y, w, h, Color::new(0.1, 0.1, 0.15, 0.85));
    draw_rectangle_lines(x, y, w, h, 1.0, Color::new(0.4, 0.4, 0.6, 0.5));
    
    // Title
    draw_text_centered_in_box(title, x + 10.0, y + 2.0, w - 20.0, 24.0, 18.0, dark::ACCENT);
}

/// Draw a clickable card component. Returns true if clicked.
pub fn card(x: f32, y: f32, w: f32, h: f32, selected: bool) -> bool {
    let hovered = is_hovered(x, y, w, h);
    let clicked = hovered && is_mouse_button_released(MouseButton::Left);
    
    let bg_color = if selected {
        Color::new(0.2, 0.25, 0.35, 0.9)
    } else if hovered {
        Color::new(0.18, 0.18, 0.25, 0.9)
    } else {
        Color::new(0.12, 0.12, 0.18, 0.9)
    };
    
    let border_color = if selected {
        dark::ACCENT
    } else {
        Color::new(0.5, 0.5, 0.5, 0.4)
    };
    
    draw_rectangle(x, y, w, h, bg_color);
    draw_rectangle_lines(x, y, w, h, 1.0, border_color);
    
    clicked
}

/// Draw a full-screen semi-transparent overlay (for modals/screens)
pub fn full_screen_overlay(alpha: f32) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.05, 0.05, 0.1, alpha),
    );
}

/// Capitalize the first character of a string
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    if let Some(c) = chars.get_mut(0) {
        *c = c.to_ascii_uppercase();
    }
    chars.into_iter().collect()
}

/// Format a type_key (snake_case) into a display name (Title Case)
/// e.g., "health_potion" -> "Health Potion"
pub fn display_name(type_key: &str) -> String {
    type_key
        .split('_')
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn truncate_text_to_width(text: &str, max_width: f32, font_size: f32) -> String {
    if measure_text(text, None, font_size as u16, 1.0).width <= max_width {
        return text.to_owned();
    }

    let ellipsis = "...";
    let mut result = String::new();
    for ch in text.chars() {
        let candidate = format!("{result}{ch}{ellipsis}");
        if measure_text(&candidate, None, font_size as u16, 1.0).width > max_width {
            break;
        }
        result.push(ch);
    }

    if result.is_empty() {
        ellipsis.to_owned()
    } else {
        format!("{result}{ellipsis}")
    }
}

pub fn wrap_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let mut wrapped = Vec::new();

    for paragraph in text.split('\n') {
        if paragraph.trim().is_empty() {
            wrapped.push(String::new());
            continue;
        }

        let mut current = String::new();
        for word in paragraph.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_owned()
            } else {
                format!("{current} {word}")
            };

            if measure_text(&candidate, None, font_size as u16, 1.0).width <= max_width {
                current = candidate;
                continue;
            }

            if !current.is_empty() {
                wrapped.push(std::mem::take(&mut current));
            }

            if measure_text(word, None, font_size as u16, 1.0).width <= max_width {
                current = word.to_owned();
                continue;
            }

            let mut chunk = String::new();
            for ch in word.chars() {
                let candidate = format!("{chunk}{ch}");
                if measure_text(&candidate, None, font_size as u16, 1.0).width > max_width
                    && !chunk.is_empty()
                {
                    wrapped.push(chunk);
                    chunk = ch.to_string();
                } else {
                    chunk.push(ch);
                }
            }
            current = chunk;
        }

        if !current.is_empty() {
            wrapped.push(current);
        }
    }

    if wrapped.is_empty() {
        vec![String::new()]
    } else {
        wrapped
    }
}

pub fn fit_text_to_box(
    text: &str,
    max_width: f32,
    max_height: f32,
    starting_font_size: f32,
    line_gap: f32,
    min_font_size: f32,
) -> TextLayoutResult {
    let mut font_size = starting_font_size;

    while font_size >= min_font_size {
        let lines = wrap_text(text, max_width, font_size);
        let total_height = lines.len() as f32 * font_size
            + (lines.len().saturating_sub(1) as f32 * line_gap);
        if total_height <= max_height {
            return TextLayoutResult {
                lines,
                font_size,
                truncated: false,
            };
        }
        font_size -= 1.0;
    }

    let font_size = min_font_size.max(1.0);
    let max_lines = ((max_height + line_gap) / (font_size + line_gap)).floor().max(1.0) as usize;
    let mut lines = wrap_text(text, max_width, font_size);
    let truncated = lines.len() > max_lines;
    lines.truncate(max_lines);
    if let Some(last_line) = lines.last_mut() {
        *last_line = truncate_text_to_width(last_line, max_width, font_size);
    }

    TextLayoutResult {
        lines,
        font_size,
        truncated,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn draw_text_block(
    text: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    starting_font_size: f32,
    line_gap: f32,
    color: Color,
) -> TextLayoutResult {
    let layout = fit_text_to_box(text, w, h, starting_font_size, line_gap, 12.0);
    let mut line_y = y + layout.font_size;
    for line in &layout.lines {
        draw_text(line, x, line_y, layout.font_size, color);
        line_y += layout.font_size + line_gap;
    }
    layout
}

pub fn draw_text_centered_in_box(
    text: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    starting_font_size: f32,
    color: Color,
) -> TextLayoutResult {
    let layout = fit_text_to_box(text, w, h, starting_font_size, 4.0, 10.0);
    let total_height = layout.lines.len() as f32 * layout.font_size
        + (layout.lines.len().saturating_sub(1) as f32 * 4.0);
    let mut line_y = y + ((h - total_height) * 0.5) + layout.font_size;

    for line in &layout.lines {
        let line_width = measure_text(line, None, layout.font_size as u16, 1.0).width;
        let line_x = x + (w - line_width) * 0.5;
        draw_text(line, line_x, line_y, layout.font_size, color);
        line_y += layout.font_size + 4.0;
    }

    layout
}


/// Helper for grid layouts
pub struct GridLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub padding: f32,
    pub cols: usize,
    pub card_height: f32,
}

impl GridLayout {
    pub fn new(x: f32, y: f32, width: f32, padding: f32, cols: usize, card_height: f32) -> Self {
        Self { x, y, width, padding, cols, card_height }
    }

    /// Multiply card height by number of rows to get total content height
    pub fn content_height(&self, item_count: usize) -> f32 {
        let rows = item_count.div_ceil(self.cols);
        (rows as f32) * (self.card_height + self.padding)
    }

    /// Get position and size for an item at index
    pub fn get_item_rect(&self, index: usize, scroll_y: f32) -> (f32, f32, f32, f32) {
        let col = (index % self.cols) as f32;
        let row = (index / self.cols) as f32;
        
        // Distribute width
        let total_padding = (self.cols - 1) as f32 * self.padding;
        let card_width = (self.width - total_padding) / self.cols as f32;

        let item_x = self.x + col * (card_width + self.padding);
        let item_y = self.y + row * (self.card_height + self.padding) - scroll_y;

        (item_x, item_y, card_width, self.card_height)
    }
}

/// Helper to handle scrolling logic
/// Returns the new scroll value clamped to 0..max_scroll
pub fn handle_scroll(current_scroll: f32, total_height: f32, view_height: f32, scroll_speed: f32) -> f32 {
    let (_, wheel_y) = mouse_wheel();
    let mut scroll = current_scroll;
    
    if wheel_y != 0.0 {
        scroll -= wheel_y * scroll_speed;
    }
    
    let max_scroll = (total_height - view_height).max(0.0);
    scroll.clamp(0.0, max_scroll)
}
