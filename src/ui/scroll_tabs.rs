//! Scrollable regions with rendered scrollbars, and tab/nav bars.
//!
//! Extracted from nanite_swarm's build-palette scrollbar, nightmare_shift's
//! skill-tree/almanac scroll offsets, kaiju_sim's sidebar nav,
//! iron_fauna's codex tab bar, and finallanding's bottom toolbar.

use macroquad::prelude::*;

use crate::colors::{dark, with_alpha};
use crate::input::{is_hovered_rect, was_clicked_rect};
use crate::ui::font::draw_text_centered_in_box;

/// Persistent scroll state for a list/panel region: wheel scrolling while
/// hovered, a proportional draggable scrollbar, and offset clamping.
///
/// Keep one `ScrollArea` per scrollable region in your state, call
/// [`update`](Self::update) each frame with the region's rect and total
/// content height, offset your row drawing by [`offset`](Self::offset),
/// then call [`draw_scrollbar`](Self::draw_scrollbar) after the rows.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScrollArea {
    offset: f32,
    dragging: bool,
    /// Pixels scrolled per wheel notch.
    pub wheel_speed: f32,
    /// Width of the scrollbar drawn at the region's right edge.
    pub bar_width: f32,
}

impl ScrollArea {
    /// Creates a scroll area with default wheel speed (40px) and bar width (8px).
    pub fn new() -> Self {
        Self {
            offset: 0.0,
            dragging: false,
            wheel_speed: 40.0,
            bar_width: 8.0,
        }
    }

    /// Current scroll offset in pixels (subtract from your content's y).
    pub fn offset(&self) -> f32 {
        self.offset
    }

    /// Jumps to a specific offset (clamped on next update).
    pub fn set_offset(&mut self, offset: f32) {
        self.offset = offset.max(0.0);
    }

    /// The largest valid offset for the given view/content heights.
    pub fn max_offset(view: Rect, content_height: f32) -> f32 {
        (content_height - view.h).max(0.0)
    }

    /// Handles wheel scrolling while hovered and scrollbar dragging, then
    /// clamps the offset. Call once per frame before drawing content.
    pub fn update(&mut self, view: Rect, content_height: f32) {
        let max_offset = Self::max_offset(view, content_height);

        if is_hovered_rect(view) {
            let (_, wheel_y) = mouse_wheel();
            if wheel_y != 0.0 {
                self.offset -= wheel_y.signum() * self.wheel_speed;
            }
        }

        if max_offset > 0.0 {
            let track = self.track_rect(view);
            let mouse = Vec2::from(mouse_position());
            if is_mouse_button_pressed(MouseButton::Left) && track.contains(mouse) {
                self.dragging = true;
            }
            if !is_mouse_button_down(MouseButton::Left) {
                self.dragging = false;
            }
            if self.dragging {
                let handle_h = self.handle_height(view, content_height);
                let t = ((mouse.y - view.y - handle_h * 0.5) / (view.h - handle_h)).clamp(0.0, 1.0);
                self.offset = t * max_offset;
            }
        } else {
            self.dragging = false;
        }

        self.offset = self.offset.clamp(0.0, max_offset);
    }

    /// Draws the scrollbar track and proportional handle. No-op when the
    /// content fits inside the view.
    pub fn draw_scrollbar(&self, view: Rect, content_height: f32) {
        let max_offset = Self::max_offset(view, content_height);
        if max_offset <= 0.0 {
            return;
        }
        let track = self.track_rect(view);
        draw_rectangle(
            track.x,
            track.y,
            track.w,
            track.h,
            Color::new(0.1, 0.1, 0.12, 0.8),
        );

        let handle_h = self.handle_height(view, content_height);
        let t = self.offset / max_offset;
        let handle_y = view.y + t * (view.h - handle_h);
        let color = if self.dragging {
            dark::ACCENT
        } else {
            with_alpha(dark::TEXT_DIM, 0.8)
        };
        draw_rectangle(track.x + 1.0, handle_y, track.w - 2.0, handle_h, color);
    }

    fn track_rect(&self, view: Rect) -> Rect {
        Rect::new(
            view.right() - self.bar_width,
            view.y,
            self.bar_width,
            view.h,
        )
    }

    fn handle_height(&self, view: Rect, content_height: f32) -> f32 {
        (view.h * (view.h / content_height.max(1.0))).clamp(24.0_f32.min(view.h), view.h)
    }
}

/// Orientation for [`tab_bar_ex`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabOrientation {
    /// Tabs side by side; the active tab is underlined.
    Horizontal,
    /// Tabs stacked; the active tab gets a left accent bar.
    Vertical,
}

/// Draws a horizontal tab bar with equal-width tabs and an accent underline
/// on the active tab. Returns the index clicked this frame, if any.
pub fn tab_bar(rect: Rect, labels: &[&str], active: usize) -> Option<usize> {
    tab_bar_ex(rect, labels, active, TabOrientation::Horizontal)
}

/// Draws a vertical nav column with an accent side bar on the active item.
/// Returns the index clicked this frame, if any.
pub fn nav_column(rect: Rect, labels: &[&str], active: usize) -> Option<usize> {
    tab_bar_ex(rect, labels, active, TabOrientation::Vertical)
}

/// Draws a one-of-N tab/nav bar. Returns the index clicked this frame, if any.
pub fn tab_bar_ex(
    rect: Rect,
    labels: &[&str],
    active: usize,
    orientation: TabOrientation,
) -> Option<usize> {
    if labels.is_empty() {
        return None;
    }
    let count = labels.len() as f32;
    let mut clicked = None;

    for (index, label) in labels.iter().enumerate() {
        let tab = match orientation {
            TabOrientation::Horizontal => {
                let w = rect.w / count;
                Rect::new(rect.x + index as f32 * w, rect.y, w, rect.h)
            }
            TabOrientation::Vertical => {
                let h = rect.h / count;
                Rect::new(rect.x, rect.y + index as f32 * h, rect.w, h)
            }
        };

        let is_active = index == active;
        let hovered = is_hovered_rect(tab);
        let fill = if is_active {
            Color::new(0.22, 0.22, 0.28, 1.0)
        } else if hovered {
            Color::new(0.18, 0.18, 0.22, 1.0)
        } else {
            Color::new(0.14, 0.14, 0.17, 1.0)
        };
        draw_rectangle(tab.x, tab.y, tab.w, tab.h, fill);

        if is_active {
            match orientation {
                TabOrientation::Horizontal => {
                    draw_rectangle(tab.x, tab.bottom() - 3.0, tab.w, 3.0, dark::ACCENT)
                }
                TabOrientation::Vertical => draw_rectangle(tab.x, tab.y, 3.0, tab.h, dark::ACCENT),
            }
        }

        let text_color = if is_active {
            dark::TEXT_BRIGHT
        } else {
            dark::TEXT_DIM
        };
        draw_text_centered_in_box(
            label,
            tab.x + 4.0,
            tab.y,
            tab.w - 8.0,
            tab.h,
            17.0,
            text_color,
        );

        if was_clicked_rect(tab) {
            clicked = Some(index);
        }
    }

    clicked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_offset_clamps_to_zero_when_content_fits() {
        let view = Rect::new(0.0, 0.0, 100.0, 200.0);
        assert_eq!(ScrollArea::max_offset(view, 150.0), 0.0);
        assert_eq!(ScrollArea::max_offset(view, 500.0), 300.0);
    }

    #[test]
    fn set_offset_never_negative() {
        let mut area = ScrollArea::new();
        area.set_offset(-50.0);
        assert_eq!(area.offset(), 0.0);
        area.set_offset(120.0);
        assert_eq!(area.offset(), 120.0);
    }
}
