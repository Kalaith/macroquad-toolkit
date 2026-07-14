//! Color palettes and color manipulation helpers for consistent game UI theming

use macroquad::prelude::Color;

/// Returns the color with its alpha replaced by `alpha`.
pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color::new(color.r, color.g, color.b, alpha)
}

/// Returns the color with its alpha multiplied by `factor` (for fading an already-translucent color).
pub fn multiply_alpha(mut color: Color, factor: f32) -> Color {
    color.a *= factor;
    color
}

/// Additively brightens each RGB channel by `amount` (clamped to 1.0). Alpha is preserved.
pub fn lighten(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r + amount).clamp(0.0, 1.0),
        (color.g + amount).clamp(0.0, 1.0),
        (color.b + amount).clamp(0.0, 1.0),
        color.a,
    )
}

/// Additively darkens each RGB channel by `amount` (clamped to 0.0). Alpha is preserved.
pub fn darken(color: Color, amount: f32) -> Color {
    lighten(color, -amount)
}

/// Multiplies each RGB channel by `factor` (clamped to `[0, 1]`). Alpha is preserved.
pub fn scale_rgb(color: Color, factor: f32) -> Color {
    Color::new(
        (color.r * factor).clamp(0.0, 1.0),
        (color.g * factor).clamp(0.0, 1.0),
        (color.b * factor).clamp(0.0, 1.0),
        color.a,
    )
}

/// Component-wise linear interpolation between two colors (including alpha).
/// `t` is clamped to `[0, 1]`.
pub fn mix(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}

/// Alias for [`mix`], matching the common `lerp_color(a, b, t)` naming.
pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    mix(a, b, t)
}

/// Converts RGB (each `[0, 1]`) to HSV: hue in degrees `[0, 360)`, saturation and value `[0, 1]`.
pub fn rgb_to_hsv(color: Color) -> (f32, f32, f32) {
    let max = color.r.max(color.g).max(color.b);
    let min = color.r.min(color.g).min(color.b);
    let delta = max - min;

    let hue = if delta <= f32::EPSILON {
        0.0
    } else if max == color.r {
        60.0 * (((color.g - color.b) / delta).rem_euclid(6.0))
    } else if max == color.g {
        60.0 * ((color.b - color.r) / delta + 2.0)
    } else {
        60.0 * ((color.r - color.g) / delta + 4.0)
    };

    let saturation = if max <= f32::EPSILON {
        0.0
    } else {
        delta / max
    };
    (hue, saturation, max)
}

/// Converts HSV (hue in degrees, saturation/value `[0, 1]`) to an opaque RGB color.
pub fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> Color {
    let hue = hue.rem_euclid(360.0);
    let saturation = saturation.clamp(0.0, 1.0);
    let value = value.clamp(0.0, 1.0);

    let c = value * saturation;
    let x = c * (1.0 - ((hue / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = value - c;

    let (r, g, b) = match (hue / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    Color::new(r + m, g + m, b + m, 1.0)
}

/// Rotates the hue of a color by `degrees`, preserving saturation, value, and alpha.
pub fn shift_hue(color: Color, degrees: f32) -> Color {
    let (h, s, v) = rgb_to_hsv(color);
    let mut shifted = hsv_to_rgb(h + degrees, s, v);
    shifted.a = color.a;
    shifted
}

/// Dark theme color palette - suitable for most game UIs
pub mod dark {
    use macroquad::prelude::Color;

    pub const BACKGROUND: Color = Color::new(0.12, 0.12, 0.14, 1.0);
    pub const PANEL: Color = Color::new(0.18, 0.18, 0.22, 1.0);
    pub const PANEL_HEADER: Color = Color::new(0.22, 0.22, 0.28, 1.0);

    pub const TEXT: Color = Color::new(0.9, 0.9, 0.9, 1.0);
    pub const TEXT_BRIGHT: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    pub const TEXT_DIM: Color = Color::new(0.6, 0.6, 0.6, 1.0);

    pub const ACCENT: Color = Color::new(0.3, 0.6, 0.9, 1.0);
    pub const POSITIVE: Color = Color::new(0.3, 0.8, 0.4, 1.0);
    pub const WARNING: Color = Color::new(0.9, 0.7, 0.2, 1.0);
    pub const NEGATIVE: Color = Color::new(0.9, 0.3, 0.3, 1.0);

    pub const HOVERED: Color = Color::new(0.3, 0.4, 0.55, 1.0);
}

/// Rarity color palette - for items, equipment, loot in RPG-style games
pub mod rarity {
    use macroquad::prelude::Color;

    pub const COMMON: Color = Color::new(0.6, 0.6, 0.6, 1.0);
    pub const UNCOMMON: Color = Color::new(0.3, 0.7, 0.3, 1.0);
    pub const RARE: Color = Color::new(0.3, 0.5, 0.9, 1.0);
    pub const EPIC: Color = Color::new(0.6, 0.3, 0.9, 1.0);
    pub const LEGENDARY: Color = Color::new(0.9, 0.6, 0.2, 1.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f32, b: f32) {
        assert!((a - b).abs() < 1e-4, "expected {b}, got {a}");
    }

    #[test]
    fn with_alpha_replaces_only_alpha() {
        let c = with_alpha(Color::new(0.2, 0.4, 0.6, 1.0), 0.25);
        assert_close(c.r, 0.2);
        assert_close(c.g, 0.4);
        assert_close(c.b, 0.6);
        assert_close(c.a, 0.25);
    }

    #[test]
    fn lighten_and_darken_clamp() {
        let c = lighten(Color::new(0.9, 0.5, 0.1, 0.8), 0.2);
        assert_close(c.r, 1.0);
        assert_close(c.g, 0.7);
        assert_close(c.a, 0.8);
        let d = darken(Color::new(0.1, 0.5, 0.9, 0.8), 0.2);
        assert_close(d.r, 0.0);
        assert_close(d.g, 0.3);
    }

    #[test]
    fn mix_interpolates_and_clamps_t() {
        let a = Color::new(0.0, 0.0, 0.0, 0.0);
        let b = Color::new(1.0, 1.0, 1.0, 1.0);
        let half = mix(a, b, 0.5);
        assert_close(half.r, 0.5);
        assert_close(half.a, 0.5);
        let over = mix(a, b, 2.0);
        assert_close(over.r, 1.0);
    }

    #[test]
    fn hsv_round_trip() {
        for &(r, g, b) in &[
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
            (0.3, 0.6, 0.9),
            (0.5, 0.5, 0.5),
        ] {
            let original = Color::new(r, g, b, 1.0);
            let (h, s, v) = rgb_to_hsv(original);
            let back = hsv_to_rgb(h, s, v);
            assert_close(back.r, r);
            assert_close(back.g, g);
            assert_close(back.b, b);
        }
    }

    #[test]
    fn shift_hue_rotates_primary() {
        let red = Color::new(1.0, 0.0, 0.0, 0.5);
        let green = shift_hue(red, 120.0);
        assert_close(green.g, 1.0);
        assert_close(green.r, 0.0);
        assert_close(green.a, 0.5);
    }
}
