//! Rectangular UI surfaces: styled panels with borders, shadows, and headers,
//! plus ragged, brush-stroke, and chamfered decorative variants.

use crate::colors::dark;
use macroquad::prelude::*;

use super::{draw_text_centered_in_box_ex, TextStyle};

/// Border configuration for rectangular UI surfaces.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceBorder {
    pub inset: f32,
    pub width: f32,
    pub color: Color,
}

impl SurfaceBorder {
    pub fn new(width: f32, color: Color) -> Self {
        Self {
            inset: 0.0,
            width,
            color,
        }
    }

    pub fn inset(mut self, inset: f32) -> Self {
        self.inset = inset;
        self
    }
}

/// Drop-shadow configuration for UI surfaces.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceShadow {
    pub offset: Vec2,
    pub color: Color,
}

impl SurfaceShadow {
    pub fn new(offset: Vec2, color: Color) -> Self {
        Self { offset, color }
    }
}

/// Optional header strip for a panel-like surface.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceHeader {
    pub height: f32,
    pub fill: Color,
    pub divider: Option<SurfaceBorder>,
}

impl SurfaceHeader {
    pub fn new(height: f32, fill: Color) -> Self {
        Self {
            height,
            fill,
            divider: None,
        }
    }

    pub fn with_divider(mut self, width: f32, color: Color) -> Self {
        self.divider = Some(SurfaceBorder::new(width, color));
        self
    }
}

/// Configurable rectangular surface style used for panels, cards, buttons, and rows.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceStyle {
    pub fill: Color,
    pub border: Option<SurfaceBorder>,
    pub shadow: Option<SurfaceShadow>,
    pub inner_border: Option<SurfaceBorder>,
    pub header: Option<SurfaceHeader>,
    pub top_highlight: Option<SurfaceBorder>,
    pub left_accent: Option<SurfaceBorder>,
}

impl SurfaceStyle {
    pub fn new(fill: Color) -> Self {
        Self {
            fill,
            border: None,
            shadow: None,
            inner_border: None,
            header: None,
            top_highlight: None,
            left_accent: None,
        }
    }

    pub fn dark_panel() -> Self {
        Self::new(dark::PANEL).with_border(1.0, dark::TEXT_DIM)
    }

    pub fn with_border(mut self, width: f32, color: Color) -> Self {
        self.border = Some(SurfaceBorder::new(width, color));
        self
    }

    pub fn with_shadow(mut self, offset: Vec2, color: Color) -> Self {
        self.shadow = Some(SurfaceShadow::new(offset, color));
        self
    }

    pub fn with_inner_border(mut self, inset: f32, width: f32, color: Color) -> Self {
        self.inner_border = Some(SurfaceBorder::new(width, color).inset(inset));
        self
    }

    pub fn with_header(mut self, height: f32, fill: Color) -> Self {
        self.header = Some(SurfaceHeader::new(height, fill));
        self
    }

    pub fn with_header_divider(mut self, width: f32, color: Color) -> Self {
        if let Some(header) = self.header.as_mut() {
            header.divider = Some(SurfaceBorder::new(width, color));
        }
        self
    }

    pub fn with_top_highlight(mut self, height: f32, color: Color) -> Self {
        self.top_highlight = Some(SurfaceBorder {
            inset: 0.0,
            width: height,
            color,
        });
        self
    }

    pub fn with_left_accent(mut self, width: f32, color: Color) -> Self {
        self.left_accent = Some(SurfaceBorder {
            inset: 0.0,
            width,
            color,
        });
        self
    }
}

impl Default for SurfaceStyle {
    fn default() -> Self {
        Self::dark_panel()
    }
}

/// Draw a configurable rectangular UI surface.
pub fn draw_surface(rect: Rect, style: &SurfaceStyle) {
    if let Some(shadow) = style.shadow {
        draw_rectangle(
            rect.x + shadow.offset.x,
            rect.y + shadow.offset.y,
            rect.w,
            rect.h,
            shadow.color,
        );
    }

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, style.fill);

    if let Some(header) = style.header {
        let header_h = header.height.min(rect.h.max(0.0));
        draw_rectangle(rect.x, rect.y, rect.w, header_h, header.fill);
        if let Some(divider) = header.divider {
            draw_line(
                rect.x,
                rect.y + header_h,
                rect.x + rect.w,
                rect.y + header_h,
                divider.width,
                divider.color,
            );
        }
    }

    if let Some(highlight) = style.top_highlight {
        let inset = highlight.inset;
        draw_rectangle(
            rect.x + inset,
            rect.y + inset,
            (rect.w - inset * 2.0).max(0.0),
            highlight.width,
            highlight.color,
        );
    }

    if let Some(accent) = style.left_accent {
        let inset = accent.inset;
        draw_rectangle(
            rect.x + inset,
            rect.y + inset,
            accent.width,
            (rect.h - inset * 2.0).max(0.0),
            accent.color,
        );
    }

    if let Some(border) = style.border {
        let inset = border.inset;
        draw_rectangle_lines(
            rect.x + inset,
            rect.y + inset,
            (rect.w - inset * 2.0).max(0.0),
            (rect.h - inset * 2.0).max(0.0),
            border.width,
            border.color,
        );
    }

    if let Some(border) = style.inner_border {
        let inset = border.inset;
        draw_rectangle_lines(
            rect.x + inset,
            rect.y + inset,
            (rect.w - inset * 2.0).max(0.0),
            (rect.h - inset * 2.0).max(0.0),
            border.width,
            border.color,
        );
    }
}

/// Draw a configurable surface with optional centered title text in its header.
pub fn draw_surface_with_title(
    rect: Rect,
    title: Option<&str>,
    style: &SurfaceStyle,
    title_style: TextStyle<'_>,
) {
    draw_surface(rect, style);

    let Some(title) = title else {
        return;
    };

    let header_h = style.header.map(|header| header.height).unwrap_or(30.0);
    draw_text_centered_in_box_ex(
        title,
        rect.x + 10.0,
        rect.y,
        rect.w - 20.0,
        header_h,
        title_style,
    );
}

/// Style for a cut-corner/ragged surface used by more expressive game UIs.
#[derive(Debug, Clone, Copy)]
pub struct RaggedSurfaceStyle {
    pub fill: Color,
    pub accent: Color,
    pub shadow: Color,
    pub emphasis: bool,
}

impl RaggedSurfaceStyle {
    pub fn new(fill: Color, accent: Color) -> Self {
        Self {
            fill,
            accent,
            shadow: Color::new(0.0, 0.0, 0.0, 0.35),
            emphasis: false,
        }
    }

    pub fn with_shadow(mut self, shadow: Color) -> Self {
        self.shadow = shadow;
        self
    }

    pub fn with_emphasis(mut self, emphasis: bool) -> Self {
        self.emphasis = emphasis;
        self
    }
}

/// Draw a cut-corner/ragged surface while preserving a rectangular layout box.
pub fn draw_ragged_surface(rect: Rect, style: &RaggedSurfaceStyle) {
    let x = rect.x;
    let y = rect.y;
    let w = rect.w;
    let h = rect.h;
    let fill = style.fill;
    let edge_alpha = if style.emphasis { 0.88 } else { 0.60 };
    let edge = Color::new(style.accent.r, style.accent.g, style.accent.b, edge_alpha);

    draw_rectangle(x + 8.0, y + 10.0, w - 6.0, h - 8.0, style.shadow);
    draw_rectangle(x + 10.0, y + 3.0, w - 20.0, h - 6.0, fill);
    draw_rectangle(x + 4.0, y + 15.0, w - 8.0, h - 30.0, fill);
    draw_triangle(
        vec2(x + 8.0, y + 5.0),
        vec2(x + 44.0, y),
        vec2(x + 18.0, y + 26.0),
        fill,
    );
    draw_triangle(
        vec2(x + w - 9.0, y + 6.0),
        vec2(x + w - 48.0, y + 1.0),
        vec2(x + w - 18.0, y + 28.0),
        fill,
    );
    draw_triangle(
        vec2(x + 8.0, y + h - 7.0),
        vec2(x + 40.0, y + h - 1.0),
        vec2(x + 18.0, y + h - 30.0),
        fill,
    );
    draw_triangle(
        vec2(x + w - 10.0, y + h - 7.0),
        vec2(x + w - 46.0, y + h),
        vec2(x + w - 18.0, y + h - 30.0),
        fill,
    );

    draw_line(
        x + 18.0,
        y + 4.0,
        x + w - 30.0,
        y + 2.0,
        if style.emphasis { 2.0 } else { 1.0 },
        edge,
    );
    draw_line(x + w - 7.0, y + 18.0, x + w - 13.0, y + h - 26.0, 1.0, edge);
    draw_line(x + 12.0, y + h - 5.0, x + w - 34.0, y + h - 2.0, 1.0, edge);
    draw_line(x + 4.0, y + 25.0, x + 10.0, y + h - 28.0, 1.0, edge);
    draw_line(
        x + 25.0,
        y + 12.0,
        x + w - 46.0,
        y + 12.0,
        1.0,
        Color::new(
            style.accent.r,
            style.accent.g,
            style.accent.b,
            edge_alpha * 0.28,
        ),
    );
}

/// Draw a layered brush-stroke surface with deterministic texture from its position.
pub fn draw_brush_stroke_surface(rect: Rect, color: Color, intensity: f32) {
    let seed = (rect.x * 100.0 + rect.y * 50.0) as i32;
    let stroke_height = rect.h * 0.7;
    let stroke_y = rect.y + (rect.h - stroke_height) * 0.5;

    for i in 0..5 {
        let offset_y = ((seed + i * 17) % 7) as f32 - 3.0;
        let offset_x = ((seed + i * 23) % 5) as f32 - 2.0;
        let thickness = stroke_height * (0.6 + i as f32 * 0.1);
        let alpha = intensity * (0.3 + i as f32 * 0.15);
        let stroke_color = Color::new(color.r, color.g, color.b, alpha.min(color.a));

        let taper_left = rect.w * 0.05;
        let taper_right = rect.w * 0.05;

        draw_rectangle(
            rect.x + taper_left + offset_x,
            stroke_y + offset_y,
            rect.w - taper_left - taper_right,
            thickness,
            stroke_color,
        );

        for t in 0..3 {
            let t_ratio = t as f32 / 3.0;
            let t_width = taper_left * (1.0 - t_ratio);
            let t_height = thickness * (0.3 + t_ratio * 0.7);
            draw_rectangle(
                rect.x + t_ratio * taper_left + offset_x,
                stroke_y + (thickness - t_height) * 0.5 + offset_y,
                t_width,
                t_height,
                stroke_color,
            );
        }

        for t in 0..3 {
            let t_ratio = t as f32 / 3.0;
            let t_width = taper_right * (1.0 - t_ratio);
            let t_height = thickness * (0.3 + t_ratio * 0.7);
            draw_rectangle(
                rect.x + rect.w - taper_right + t_ratio * taper_right + offset_x,
                stroke_y + (thickness - t_height) * 0.5 + offset_y,
                t_width,
                t_height,
                stroke_color,
            );
        }
    }
}

/// Style for a compact cut-corner surface such as badges, metric tiles, and footer buttons.
#[derive(Debug, Clone, Copy)]
pub struct ChamferedSurfaceStyle {
    pub fill: Color,
    pub accent: Color,
    pub corner: f32,
    pub border_width: f32,
    pub lower_alpha: f32,
}

impl ChamferedSurfaceStyle {
    pub fn new(fill: Color, accent: Color) -> Self {
        Self {
            fill,
            accent,
            corner: 8.0,
            border_width: 1.0,
            lower_alpha: 0.48,
        }
    }

    pub fn with_corner(mut self, corner: f32) -> Self {
        self.corner = corner;
        self
    }

    pub fn with_border_width(mut self, width: f32) -> Self {
        self.border_width = width;
        self
    }

    pub fn with_lower_alpha(mut self, alpha: f32) -> Self {
        self.lower_alpha = alpha;
        self
    }
}

/// Draw a compact cut-corner surface while preserving the supplied layout rectangle.
pub fn draw_chamfered_surface(rect: Rect, style: &ChamferedSurfaceStyle) {
    let c = style.corner.min(rect.w * 0.25).min(rect.h * 0.5).max(0.0);
    draw_rectangle(
        rect.x + c,
        rect.y,
        (rect.w - c * 2.0).max(0.0),
        rect.h,
        style.fill,
    );
    draw_rectangle(
        rect.x,
        rect.y + c,
        rect.w,
        (rect.h - c * 2.0).max(0.0),
        style.fill,
    );
    draw_triangle(
        vec2(rect.x, rect.y + c),
        vec2(rect.x + c, rect.y),
        vec2(rect.x + c, rect.y + rect.h),
        style.fill,
    );
    draw_triangle(
        vec2(rect.x + rect.w, rect.y + c),
        vec2(rect.x + rect.w - c, rect.y),
        vec2(rect.x + rect.w - c, rect.y + rect.h),
        style.fill,
    );

    draw_line(
        rect.x + c,
        rect.y,
        rect.x + rect.w - c,
        rect.y,
        style.border_width,
        style.accent,
    );
    draw_line(
        rect.x + c,
        rect.y + rect.h,
        rect.x + rect.w - c,
        rect.y + rect.h,
        style.border_width,
        Color::new(
            style.accent.r,
            style.accent.g,
            style.accent.b,
            style.accent.a * style.lower_alpha,
        ),
    );
}
