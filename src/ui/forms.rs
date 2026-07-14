//! Form-style widgets: toggles, checkboxes, sliders, steppers, segmented
//! bars, and keycaps.
//!
//! Extracted from settings screens across the games: ai_defense's
//! toggle/slider rows, monstron's animated switch, scrapyard's checkbox,
//! keycap, and segmented bar, and nanite_swarm's volume steppers.

use std::cell::Cell;

use macroquad::prelude::*;

use crate::colors::{dark, with_alpha};
use crate::input::{is_hovered_rect, was_clicked_rect};
use crate::ui::font::{draw_text_centered_in_box, draw_ui_text, measure_ui_text};
use crate::ui::widgets::button_rect;

/// Draws an on/off switch (track + knob). Returns true when toggled this frame.
pub fn toggle(rect: Rect, value: &mut bool) -> bool {
    let hovered = is_hovered_rect(rect);
    let clicked = was_clicked_rect(rect);
    if clicked {
        *value = !*value;
    }

    let track_color = if *value {
        dark::ACCENT
    } else {
        Color::new(0.3, 0.3, 0.35, 1.0)
    };
    let track_color = if hovered {
        with_alpha(track_color, 0.9)
    } else {
        track_color
    };

    let radius = rect.h * 0.5;
    draw_rectangle(
        rect.x + radius,
        rect.y,
        rect.w - rect.h,
        rect.h,
        track_color,
    );
    draw_circle(rect.x + radius, rect.y + radius, radius, track_color);
    draw_circle(rect.right() - radius, rect.y + radius, radius, track_color);

    let knob_x = if *value {
        rect.right() - radius
    } else {
        rect.x + radius
    };
    draw_circle(knob_x, rect.y + radius, radius - 2.0, dark::TEXT_BRIGHT);

    clicked
}

/// Draws a labeled toggle row: label on the left, switch on the right.
/// Returns true when toggled this frame.
pub fn toggle_row(rect: Rect, label: &str, value: &mut bool) -> bool {
    let switch_w = rect.h * 1.9;
    let switch = Rect::new(
        rect.right() - switch_w,
        rect.y + rect.h * 0.15,
        switch_w,
        rect.h * 0.7,
    );
    draw_text_centered_in_box(
        label,
        rect.x,
        rect.y,
        rect.w - switch_w - 12.0,
        rect.h,
        18.0,
        dark::TEXT,
    );
    // The whole row is clickable, not just the switch.
    let row_clicked = was_clicked_rect(rect) && !is_hovered_rect(switch);
    if row_clicked {
        *value = !*value;
    }
    toggle(switch, value) || row_clicked
}

/// Draws a checkbox with an optional label to its right.
/// Returns true when toggled this frame.
pub fn checkbox(box_rect: Rect, label: &str, value: &mut bool) -> bool {
    let label_width = if label.is_empty() {
        0.0
    } else {
        measure_ui_text(label, None, 18, 1.0).width + 10.0
    };
    let hit = Rect::new(box_rect.x, box_rect.y, box_rect.w + label_width, box_rect.h);
    let clicked = was_clicked_rect(hit);
    if clicked {
        *value = !*value;
    }

    let border = if is_hovered_rect(hit) {
        dark::ACCENT
    } else {
        dark::TEXT_DIM
    };
    draw_rectangle(box_rect.x, box_rect.y, box_rect.w, box_rect.h, dark::PANEL);
    draw_rectangle_lines(box_rect.x, box_rect.y, box_rect.w, box_rect.h, 2.0, border);
    if *value {
        let inset = box_rect.w * 0.25;
        draw_line(
            box_rect.x + inset,
            box_rect.y + box_rect.h * 0.55,
            box_rect.x + box_rect.w * 0.45,
            box_rect.bottom() - inset,
            2.0,
            dark::POSITIVE,
        );
        draw_line(
            box_rect.x + box_rect.w * 0.45,
            box_rect.bottom() - inset,
            box_rect.right() - inset * 0.75,
            box_rect.y + inset,
            2.0,
            dark::POSITIVE,
        );
    }
    if !label.is_empty() {
        draw_text_centered_in_box(
            label,
            box_rect.right() + 10.0,
            box_rect.y,
            label_width,
            box_rect.h,
            18.0,
            dark::TEXT,
        );
    }
    clicked
}

thread_local! {
    static ACTIVE_SLIDER: Cell<Option<u64>> = const { Cell::new(None) };
}

fn slider_id(rect: Rect) -> u64 {
    ((rect.x.to_bits() as u64) << 32) ^ (rect.y.to_bits() as u64)
}

/// Draws a horizontal draggable slider. Returns true while the value is
/// being changed. Dragging may continue outside the track once started.
pub fn slider(rect: Rect, value: &mut f32, min: f32, max: f32) -> bool {
    let id = slider_id(rect);
    let mouse = Vec2::from(mouse_position());
    let hovered = rect.contains(mouse);

    if is_mouse_button_pressed(MouseButton::Left) && hovered {
        ACTIVE_SLIDER.with(|active| active.set(Some(id)));
    }
    if !is_mouse_button_down(MouseButton::Left) {
        ACTIVE_SLIDER.with(|active| {
            if active.get() == Some(id) {
                active.set(None);
            }
        });
    }
    let dragging = ACTIVE_SLIDER.with(|active| active.get() == Some(id));

    let mut changed = false;
    if dragging && max > min {
        let t = ((mouse.x - rect.x) / rect.w).clamp(0.0, 1.0);
        let new_value = min + t * (max - min);
        if (new_value - *value).abs() > f32::EPSILON {
            *value = new_value;
            changed = true;
        }
    }

    let t = if max > min {
        ((*value - min) / (max - min)).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let track_h = (rect.h * 0.3).max(4.0);
    let track_y = rect.y + (rect.h - track_h) * 0.5;
    draw_rectangle(
        rect.x,
        track_y,
        rect.w,
        track_h,
        Color::new(0.15, 0.15, 0.18, 1.0),
    );
    draw_rectangle(rect.x, track_y, rect.w * t, track_h, dark::ACCENT);
    draw_rectangle_lines(rect.x, track_y, rect.w, track_h, 1.0, dark::TEXT_DIM);

    let knob_x = rect.x + rect.w * t;
    let knob_r = (rect.h * 0.5).min(10.0);
    let knob_color = if dragging || hovered {
        dark::TEXT_BRIGHT
    } else {
        dark::TEXT
    };
    draw_circle(knob_x, rect.y + rect.h * 0.5, knob_r, knob_color);

    changed
}

/// Draws a labeled slider row: label left, slider center, current value
/// rendered as a percentage on the right. Returns true while changing.
pub fn slider_row(rect: Rect, label: &str, value: &mut f32, min: f32, max: f32) -> bool {
    let label_w = rect.w * 0.35;
    let value_w = 56.0;
    draw_text_centered_in_box(label, rect.x, rect.y, label_w, rect.h, 18.0, dark::TEXT);

    let track = Rect::new(rect.x + label_w, rect.y, rect.w - label_w - value_w, rect.h);
    let changed = slider(track, value, min, max);

    let percent = if max > min {
        ((*value - min) / (max - min) * 100.0).round()
    } else {
        0.0
    };
    draw_text_centered_in_box(
        &format!("{percent:.0}%"),
        rect.right() - value_w,
        rect.y,
        value_w,
        rect.h,
        18.0,
        dark::TEXT_DIM,
    );
    changed
}

/// Draws a `-` / value / `+` stepper row. Returns -1, 0, or +1 for the
/// direction clicked this frame; the caller applies stepping and clamping.
pub fn stepper_row(rect: Rect, label: &str, value_text: &str) -> i32 {
    let button_w = rect.h;
    let label_w = rect.w * 0.4;
    draw_text_centered_in_box(label, rect.x, rect.y, label_w, rect.h, 18.0, dark::TEXT);

    let minus = Rect::new(rect.x + label_w, rect.y, button_w, rect.h);
    let plus = Rect::new(rect.right() - button_w, rect.y, button_w, rect.h);
    let value_box_x = minus.right();

    draw_text_centered_in_box(
        value_text,
        value_box_x,
        rect.y,
        plus.x - value_box_x,
        rect.h,
        18.0,
        dark::TEXT_BRIGHT,
    );

    let mut direction = 0;
    if button_rect(minus, "-") {
        direction -= 1;
    }
    if button_rect(plus, "+") {
        direction += 1;
    }
    direction
}

/// Draws a bar of discrete segments, filling them left to right by
/// `fraction` (0..=1). Good for ammo, charges, and chunked health.
pub fn segmented_bar(rect: Rect, fraction: f32, segments: usize, fill: Color) {
    let segments = segments.max(1);
    let gap = 2.0;
    let seg_w = (rect.w - gap * (segments as f32 - 1.0)) / segments as f32;
    let filled = (fraction.clamp(0.0, 1.0) * segments as f32).round() as usize;
    for i in 0..segments {
        let x = rect.x + i as f32 * (seg_w + gap);
        let color = if i < filled {
            fill
        } else {
            Color::new(0.15, 0.15, 0.18, 1.0)
        };
        draw_rectangle(x, rect.y, seg_w, rect.h, color);
        draw_rectangle_lines(x, rect.y, seg_w, rect.h, 1.0, dark::TEXT_DIM);
    }
}

/// Draws a keyboard-key cap with a centered label, for control hints.
pub fn keycap(rect: Rect, label: &str) {
    draw_rectangle(
        rect.x,
        rect.y + 2.0,
        rect.w,
        rect.h,
        Color::new(0.1, 0.1, 0.12, 1.0),
    );
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h - 2.0,
        Color::new(0.25, 0.25, 0.3, 1.0),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, dark::TEXT_DIM);
    draw_text_centered_in_box(
        label,
        rect.x,
        rect.y,
        rect.w,
        rect.h - 2.0,
        14.0,
        dark::TEXT_BRIGHT,
    );
}

/// Draws a small hint like `[F3] Debug` — a keycap followed by a label.
pub fn keycap_hint(x: f32, y: f32, key: &str, label: &str) -> f32 {
    let key_w = (measure_ui_text(key, None, 14, 1.0).width + 12.0).max(22.0);
    let cap = Rect::new(x, y, key_w, 20.0);
    keycap(cap, key);
    let dimensions = draw_ui_text(label, cap.right() + 6.0, y + 15.0, 16.0, dark::TEXT_DIM);
    cap.right() + 6.0 + dimensions.width
}
