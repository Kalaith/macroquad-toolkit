//! UI rendering utilities for buttons, panels, and progress bars

use crate::colors::dark;
use crate::input::*;
use macroquad::prelude::*;
use std::cell::RefCell;

thread_local! {
    static DEFAULT_UI_FONT: RefCell<Option<&'static Font>> = const { RefCell::new(None) };
    static UI_TEXT_SCALE: RefCell<f32> = const { RefCell::new(1.0) };
    static MIN_UI_FONT_SIZE: RefCell<f32> = const { RefCell::new(1.0) };
}

fn font_size_u16(font_size: f32) -> u16 {
    font_size.round().clamp(1.0, u16::MAX as f32) as u16
}

fn ui_text_scale() -> f32 {
    UI_TEXT_SCALE.with(|stored| stored.borrow().clamp(0.25, 4.0))
}

fn min_ui_font_size() -> f32 {
    MIN_UI_FONT_SIZE.with(|stored| stored.borrow().clamp(1.0, 96.0))
}

fn effective_font_size(font_size: f32) -> f32 {
    font_size.max(min_ui_font_size()) * ui_text_scale()
}

fn effective_line_gap(line_gap: f32) -> f32 {
    line_gap * ui_text_scale()
}

/// Register a default font used by toolkit text helpers when no explicit font is supplied.
///
/// This is intended to be called once during game startup. The font is retained for the
/// process lifetime so `TextStyle::params()` can safely return Macroquad's borrowed font params.
pub fn set_default_ui_font(font: Font) {
    let font = Box::leak(Box::new(font));
    DEFAULT_UI_FONT.with(|stored| {
        *stored.borrow_mut() = Some(font);
    });
}

/// Decode and register a default font from embedded TTF/OTF bytes.
pub fn set_default_ui_font_from_bytes(bytes: &'static [u8]) -> Result<(), String> {
    let font = load_ttf_font_from_bytes(bytes)
        .map_err(|err| format!("failed to load default UI font: {err:?}"))?;
    set_default_ui_font(font);
    Ok(())
}

/// Return the registered default UI font, if one has been set.
pub fn default_ui_font() -> Option<&'static Font> {
    DEFAULT_UI_FONT.with(|stored| *stored.borrow())
}

/// Set a global multiplier used by toolkit text helpers.
///
/// This is useful for dense fixed-resolution UIs when the canvas is being displayed below
/// its logical resolution. The scale affects drawing and text measurement consistently.
pub fn set_ui_text_scale(scale: f32) {
    UI_TEXT_SCALE.with(|stored| {
        *stored.borrow_mut() = scale.clamp(0.25, 4.0);
    });
}

/// Set the minimum logical font size used by toolkit text helpers.
pub fn set_min_ui_font_size(font_size: f32) {
    MIN_UI_FONT_SIZE.with(|stored| {
        *stored.borrow_mut() = font_size.clamp(1.0, 96.0);
    });
}

/// Scale text up when a fixed logical UI is displayed below its design resolution.
pub fn set_ui_text_scale_for_screen(
    logical_width: f32,
    logical_height: f32,
    max_scale: f32,
) -> f32 {
    let pixel_scale = (screen_width() / logical_width.max(1.0))
        .min(screen_height() / logical_height.max(1.0))
        .max(0.01);
    let scale = (1.0 / pixel_scale).clamp(1.0, max_scale.max(1.0));
    set_ui_text_scale(scale);
    scale
}

#[derive(Debug, Clone)]
pub struct TextLayoutResult {
    pub lines: Vec<String>,
    pub font_size: f32,
    pub truncated: bool,
}

/// Font-aware text drawing configuration.
#[derive(Debug, Clone, Copy)]
pub struct TextStyle<'a> {
    pub font: Option<&'a Font>,
    pub font_size: f32,
    pub color: Color,
    pub line_gap: f32,
}

impl<'a> TextStyle<'a> {
    pub fn new(font_size: f32, color: Color) -> Self {
        Self {
            font: None,
            font_size,
            color,
            line_gap: 4.0,
        }
    }

    pub fn with_font(mut self, font: &'a Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn with_line_gap(mut self, line_gap: f32) -> Self {
        self.line_gap = line_gap;
        self
    }

    pub fn resolved_font(&self) -> Option<&'a Font> {
        self.font.or(default_ui_font())
    }

    pub fn effective_font_size(&self) -> f32 {
        effective_font_size(self.font_size)
    }

    pub fn effective_line_gap(&self) -> f32 {
        effective_line_gap(self.line_gap)
    }

    pub fn params(&self) -> TextParams<'a> {
        TextParams {
            font: self.resolved_font(),
            font_size: font_size_u16(self.effective_font_size()),
            color: self.color,
            ..Default::default()
        }
    }
}

impl Default for TextStyle<'_> {
    fn default() -> Self {
        Self::new(20.0, dark::TEXT)
    }
}

/// Measure text using a [`TextStyle`].
pub fn measure_text_size(text: &str, style: TextStyle<'_>) -> TextDimensions {
    measure_text(
        text,
        style.resolved_font(),
        font_size_u16(style.effective_font_size()),
        1.0,
    )
}

/// Format an integer currency value with comma separators.
pub fn format_money(value: i64) -> String {
    let sign = if value < 0 { "-" } else { "" };
    let mut digits = value.abs().to_string();
    let mut result = String::new();

    while digits.len() > 3 {
        let tail = digits.split_off(digits.len() - 3);
        if result.is_empty() {
            result = tail;
        } else {
            result = format!("{tail},{result}");
        }
    }

    if result.is_empty() {
        format!("{sign}${digits}")
    } else {
        format!("{sign}${digits},{result}")
    }
}

/// Format an integer currency value compactly for dense UI.
pub fn format_compact_money(value: i64) -> String {
    let sign = if value < 0 { "-" } else { "" };
    let abs = value.abs();
    if abs >= 1_000_000 {
        format!("{sign}${:.1}m", abs as f32 / 1_000_000.0)
    } else if abs >= 1_000 {
        format!("{sign}${}k", abs / 1_000)
    } else {
        format!("{sign}${abs}")
    }
}

/// Mouse event used by a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonTrigger {
    Press,
    Release,
}

/// Semantic button tone for common UI actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonTone {
    Primary,
    Secondary,
    Positive,
    Warning,
    Danger,
    Muted,
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

    /// Style from a semantic button tone.
    pub fn from_tone(tone: ButtonTone) -> Self {
        match tone {
            ButtonTone::Primary => Self {
                normal: dark::ACCENT,
                hovered: Color::new(0.35, 0.55, 0.9, 1.0),
                pressed: Color::new(0.18, 0.32, 0.58, 1.0),
                border: dark::TEXT_BRIGHT,
                text_color: dark::TEXT_BRIGHT,
                disabled: Color::new(0.14, 0.16, 0.2, 1.0),
            },
            ButtonTone::Secondary => Self::default_dark(),
            ButtonTone::Positive => Self {
                normal: dark::POSITIVE,
                hovered: Color::new(0.35, 0.75, 0.45, 1.0),
                pressed: Color::new(0.12, 0.42, 0.22, 1.0),
                border: dark::TEXT_BRIGHT,
                text_color: dark::TEXT_BRIGHT,
                disabled: Color::new(0.12, 0.18, 0.14, 1.0),
            },
            ButtonTone::Warning => Self {
                normal: dark::WARNING,
                hovered: Color::new(0.95, 0.72, 0.25, 1.0),
                pressed: Color::new(0.55, 0.34, 0.08, 1.0),
                border: dark::TEXT_BRIGHT,
                text_color: dark::TEXT_BRIGHT,
                disabled: Color::new(0.2, 0.16, 0.08, 1.0),
            },
            ButtonTone::Danger => Self {
                normal: dark::NEGATIVE,
                hovered: Color::new(0.9, 0.32, 0.32, 1.0),
                pressed: Color::new(0.55, 0.12, 0.12, 1.0),
                border: dark::TEXT_BRIGHT,
                text_color: dark::TEXT_BRIGHT,
                disabled: Color::new(0.18, 0.1, 0.1, 1.0),
            },
            ButtonTone::Muted => Self {
                normal: Color::new(0.12, 0.12, 0.14, 1.0),
                hovered: Color::new(0.18, 0.18, 0.22, 1.0),
                pressed: Color::new(0.08, 0.08, 0.1, 1.0),
                border: dark::TEXT_DIM,
                text_color: dark::TEXT_DIM,
                disabled: Color::new(0.08, 0.08, 0.09, 1.0),
            },
        }
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self::default_dark()
    }
}

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

/// Convenience rectangle wrapper used by several games for UI layout.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl UiRect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn from_rect(rect: Rect) -> Self {
        rect.into()
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.w, self.h)
    }

    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }

    pub fn center(&self) -> Vec2 {
        vec2(self.x + self.w * 0.5, self.y + self.h * 0.5)
    }

    pub fn centered_x(width: f32, y: f32, w: f32, h: f32) -> Self {
        Self::new((width - w) * 0.5, y, w, h)
    }

    pub fn centered_on_screen(w: f32, h: f32) -> Self {
        Self::new(
            (screen_width() - w) * 0.5,
            (screen_height() - h) * 0.5,
            w,
            h,
        )
    }

    pub fn inset(&self, amount: f32) -> Self {
        Self::new(
            self.x + amount,
            self.y + amount,
            (self.w - amount * 2.0).max(0.0),
            (self.h - amount * 2.0).max(0.0),
        )
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        rect_contains(self.x, self.y, self.w, self.h, point.x, point.y)
    }

    pub fn contains_mouse(&self) -> bool {
        let (mx, my) = mouse_position();
        self.contains_point(vec2(mx, my))
    }
}

impl From<Rect> for UiRect {
    fn from(rect: Rect) -> Self {
        Self::new(rect.x, rect.y, rect.w, rect.h)
    }
}

impl From<UiRect> for Rect {
    fn from(rect: UiRect) -> Self {
        rect.rect()
    }
}

/// Convenience methods for macroquad `Rect` values.
pub trait RectExt {
    fn right(&self) -> f32;
    fn bottom(&self) -> f32;
    fn center(&self) -> Vec2;
    fn inset(&self, amount: f32) -> Rect;
    fn contains_point(&self, point: Vec2) -> bool;
    fn contains_mouse(&self) -> bool;
}

impl RectExt for Rect {
    fn right(&self) -> f32 {
        self.x + self.w
    }

    fn bottom(&self) -> f32 {
        self.y + self.h
    }

    fn center(&self) -> Vec2 {
        vec2(self.x + self.w * 0.5, self.y + self.h * 0.5)
    }

    fn inset(&self, amount: f32) -> Rect {
        Rect::new(
            self.x + amount,
            self.y + amount,
            (self.w - amount * 2.0).max(0.0),
            (self.h - amount * 2.0).max(0.0),
        )
    }

    fn contains_point(&self, point: Vec2) -> bool {
        rect_contains(self.x, self.y, self.w, self.h, point.x, point.y)
    }

    fn contains_mouse(&self) -> bool {
        self.contains_point(mouse_position_vec2())
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

/// Draw a button from a `Rect` using default styling.
pub fn button_rect(rect: Rect, text: &str) -> bool {
    button_rect_styled(rect, text, &ButtonStyle::default())
}

/// Draw a button from a `Rect` using custom styling.
pub fn button_rect_styled(rect: Rect, text: &str, style: &ButtonStyle) -> bool {
    button_rect_enabled_styled(rect, text, true, style)
}

/// Draw an enabled/disabled button with default styling.
pub fn button_enabled(x: f32, y: f32, w: f32, h: f32, text: &str, enabled: bool) -> bool {
    button_rect_enabled(Rect::new(x, y, w, h), text, enabled)
}

/// Draw an enabled/disabled button from a `Rect`.
pub fn button_rect_enabled(rect: Rect, text: &str, enabled: bool) -> bool {
    button_rect_enabled_styled(rect, text, enabled, &ButtonStyle::default())
}

/// Draw an enabled/disabled button with a semantic tone.
pub fn button_rect_tone(rect: Rect, text: &str, enabled: bool, tone: ButtonTone) -> bool {
    let style = ButtonStyle::from_tone(tone);
    button_rect_enabled_styled(rect, text, enabled, &style)
}

/// Draw an enabled/disabled button from a `Rect` using custom styling.
pub fn button_rect_enabled_styled(
    rect: Rect,
    text: &str,
    enabled: bool,
    style: &ButtonStyle,
) -> bool {
    button_rect_enabled_styled_ex(
        rect,
        text,
        enabled,
        style,
        TextStyle::new(20.0, style.text_color),
        ButtonTrigger::Release,
    )
}

/// Font-aware, `Rect`-based button renderer.
pub fn button_rect_enabled_styled_ex(
    rect: Rect,
    text: &str,
    enabled: bool,
    style: &ButtonStyle,
    text_style: TextStyle<'_>,
    trigger: ButtonTrigger,
) -> bool {
    let hovered = enabled && rect.contains_mouse();
    let is_pressed = hovered && is_mouse_button_down(MouseButton::Left);
    let activated = match trigger {
        ButtonTrigger::Press => hovered && is_mouse_button_pressed(MouseButton::Left),
        ButtonTrigger::Release => hovered && is_mouse_button_released(MouseButton::Left),
    };

    let bg_color = if !enabled {
        style.disabled
    } else if is_pressed {
        style.pressed
    } else if hovered {
        style.hovered
    } else {
        style.normal
    };

    let text_color = if enabled {
        text_style.color
    } else {
        Color::new(
            text_style.color.r,
            text_style.color.g,
            text_style.color.b,
            0.45,
        )
    };

    let surface = SurfaceStyle::new(bg_color).with_border(2.0, style.border);
    draw_surface(rect, &surface);

    let y_offset = if is_pressed { 2.0 } else { 0.0 };
    draw_text_centered_in_box_ex(
        text,
        rect.x + 8.0,
        rect.y + y_offset,
        rect.w - 16.0,
        rect.h,
        TextStyle {
            color: text_color,
            ..text_style
        },
    );

    activated
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

    let surface = SurfaceStyle::new(bg_color).with_border(2.0, style.border);
    draw_surface(Rect::new(x, y, w, h), &surface);

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

    let surface = SurfaceStyle::new(bg_color).with_border(2.0, style.border);
    draw_surface(Rect::new(x, y, w, h), &surface);

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
    let mut surface = SurfaceStyle::new(dark::PANEL)
        .with_shadow(vec2(4.0, 4.0), Color::new(0.0, 0.0, 0.0, 0.5))
        .with_border(2.0, dark::ACCENT);
    if title.is_some() {
        surface = surface.with_header(30.0, dark::PANEL_HEADER);
    }
    draw_surface_with_title(
        Rect::new(x, y, w, h),
        title,
        &surface,
        TextStyle::new(20.0, dark::TEXT),
    );

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
    let mut style = SurfaceStyle::dark_panel();
    if title.is_some() {
        style = style.with_header(30.0, dark::PANEL_HEADER);
    }
    draw_surface_with_title(
        Rect::new(x, y, w, h),
        title,
        &style,
        TextStyle::new(20.0, dark::TEXT),
    );
}

/// Draw a panel with shadow effect
pub fn panel_with_shadow(x: f32, y: f32, w: f32, h: f32) {
    let style = SurfaceStyle::new(dark::PANEL)
        .with_shadow(vec2(4.0, 4.0), Color::new(0.0, 0.0, 0.0, 0.5))
        .with_border(2.0, dark::TEXT_DIM)
        .with_inner_border(2.0, 1.0, Color::new(0.2, 0.2, 0.22, 0.5));
    draw_surface(Rect::new(x, y, w, h), &style);
}

/// Draw a progress bar
pub fn progress_bar(x: f32, y: f32, w: f32, h: f32, value: f32, max: f32, color: Color) {
    let ratio = if max <= 0.0 {
        0.0
    } else {
        (value / max).clamp(0.0, 1.0)
    };
    let fill_width = ratio * w;

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
    let style = SurfaceStyle::new(Color::new(0.1, 0.1, 0.15, 0.85))
        .with_border(1.0, Color::new(0.4, 0.4, 0.6, 0.5));
    draw_surface_with_title(
        Rect::new(x, y, w, h),
        Some(title),
        &style,
        TextStyle::new(18.0, dark::ACCENT),
    );
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

    let style = SurfaceStyle::new(bg_color).with_border(1.0, border_color);
    draw_surface(Rect::new(x, y, w, h), &style);

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
    truncate_text_to_width_ex(text, max_width, None, font_size)
}

pub fn truncate_text_to_width_ex(
    text: &str,
    max_width: f32,
    font: Option<&Font>,
    font_size: f32,
) -> String {
    let font = font.or(default_ui_font());
    let font_size = effective_font_size(font_size);
    if measure_text(text, font, font_size_u16(font_size), 1.0).width <= max_width {
        return text.to_owned();
    }

    let ellipsis = "...";
    let mut result = String::new();
    for ch in text.chars() {
        let candidate = format!("{result}{ch}{ellipsis}");
        if measure_text(&candidate, font, font_size_u16(font_size), 1.0).width > max_width {
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
    wrap_text_ex(text, max_width, None, font_size)
}

pub fn wrap_text_ex(
    text: &str,
    max_width: f32,
    font: Option<&Font>,
    font_size: f32,
) -> Vec<String> {
    let font = font.or(default_ui_font());
    let font_size = effective_font_size(font_size);
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

            if measure_text(&candidate, font, font_size_u16(font_size), 1.0).width <= max_width {
                current = candidate;
                continue;
            }

            if !current.is_empty() {
                wrapped.push(std::mem::take(&mut current));
            }

            if measure_text(word, font, font_size_u16(font_size), 1.0).width <= max_width {
                current = word.to_owned();
                continue;
            }

            let mut chunk = String::new();
            for ch in word.chars() {
                let candidate = format!("{chunk}{ch}");
                if measure_text(&candidate, font, font_size_u16(font_size), 1.0).width > max_width
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
    fit_text_to_box_ex(
        text,
        max_width,
        max_height,
        TextStyle::new(starting_font_size, dark::TEXT).with_line_gap(line_gap),
        min_font_size,
    )
}

pub fn fit_text_to_box_ex(
    text: &str,
    max_width: f32,
    max_height: f32,
    style: TextStyle<'_>,
    min_font_size: f32,
) -> TextLayoutResult {
    let mut font_size = style.font_size;
    let line_gap = style.effective_line_gap();

    while font_size >= min_font_size {
        let lines = wrap_text_ex(text, max_width, style.font, font_size);
        let draw_font_size = effective_font_size(font_size);
        let total_height =
            lines.len() as f32 * draw_font_size + (lines.len().saturating_sub(1) as f32 * line_gap);
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
    let draw_font_size = effective_font_size(font_size);
    let max_lines = ((max_height + line_gap) / (draw_font_size + line_gap))
        .floor()
        .max(1.0) as usize;
    let mut lines = wrap_text_ex(text, max_width, style.font, font_size);
    let truncated = lines.len() > max_lines;
    lines.truncate(max_lines);
    if let Some(last_line) = lines.last_mut() {
        *last_line = truncate_text_to_width_ex(last_line, max_width, style.font, font_size);
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
    draw_text_block_ex(
        text,
        x,
        y,
        w,
        h,
        TextStyle::new(starting_font_size, color).with_line_gap(line_gap),
        12.0,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn draw_text_block_ex(
    text: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    style: TextStyle<'_>,
    min_font_size: f32,
) -> TextLayoutResult {
    let layout = fit_text_to_box_ex(text, w, h, style, min_font_size);
    let draw_font_size = effective_font_size(layout.font_size);
    let line_gap = style.effective_line_gap();
    let mut line_y = y + draw_font_size;
    for line in &layout.lines {
        draw_text_ex(
            line,
            x,
            line_y,
            TextStyle {
                font_size: layout.font_size,
                ..style
            }
            .params(),
        );
        line_y += draw_font_size + line_gap;
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
    draw_text_centered_in_box_ex(text, x, y, w, h, TextStyle::new(starting_font_size, color))
}

pub fn draw_text_centered_in_box_ex(
    text: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    style: TextStyle<'_>,
) -> TextLayoutResult {
    let layout = fit_text_to_box_ex(text, w, h, style, 10.0);
    let draw_font_size = effective_font_size(layout.font_size);
    let line_gap = style.effective_line_gap();
    let total_height = layout.lines.len() as f32 * draw_font_size
        + (layout.lines.len().saturating_sub(1) as f32 * line_gap);
    let mut line_y = y + ((h - total_height) * 0.5) + draw_font_size;

    for line in &layout.lines {
        let line_width = measure_text(
            line,
            style.resolved_font(),
            font_size_u16(draw_font_size),
            1.0,
        )
        .width;
        let line_x = x + (w - line_width) * 0.5;
        draw_text_ex(
            line,
            line_x,
            line_y,
            TextStyle {
                font_size: layout.font_size,
                ..style
            }
            .params(),
        );
        line_y += draw_font_size + line_gap;
    }

    layout
}

/// Draw text centered around `center_x` at the supplied baseline.
pub fn draw_text_centered(text: &str, center_x: f32, baseline_y: f32, style: TextStyle<'_>) {
    let dimensions = measure_text_size(text, style);
    draw_text_ex(
        text,
        center_x - dimensions.width * 0.5,
        baseline_y,
        style.params(),
    );
}

pub fn draw_text_right(text: &str, right_x: f32, baseline_y: f32, style: TextStyle<'_>) {
    let width = measure_text(
        text,
        style.resolved_font(),
        font_size_u16(style.effective_font_size()),
        1.0,
    )
    .width;
    draw_text_ex(text, right_x - width, baseline_y, style.params());
}

pub fn draw_text_shadow(
    text: &str,
    x: f32,
    y: f32,
    style: TextStyle<'_>,
    shadow_offset: Vec2,
    shadow_color: Color,
) {
    draw_text_ex(
        text,
        x + shadow_offset.x,
        y + shadow_offset.y,
        TextStyle {
            color: shadow_color,
            ..style
        }
        .params(),
    );
    draw_text_ex(text, x, y, style.params());
}

/// Fixed-resolution UI mapper with optional letterboxing.
#[derive(Debug, Clone, Copy)]
pub struct VirtualUi {
    pub logical_width: f32,
    pub logical_height: f32,
    pub scale: f32,
    pub offset: Vec2,
}

impl VirtualUi {
    pub fn new(logical_width: f32, logical_height: f32) -> Self {
        let scale_x = screen_width() / logical_width;
        let scale_y = screen_height() / logical_height;
        let scale = scale_x.min(scale_y);
        let viewport_width = logical_width * scale;
        let viewport_height = logical_height * scale;
        let offset = vec2(
            (screen_width() - viewport_width) * 0.5,
            (screen_height() - viewport_height) * 0.5,
        );

        Self {
            logical_width,
            logical_height,
            scale,
            offset,
        }
    }

    pub fn viewport(&self) -> (i32, i32, i32, i32) {
        (
            self.offset.x.round() as i32,
            self.offset.y.round() as i32,
            (self.logical_width * self.scale).round() as i32,
            (self.logical_height * self.scale).round() as i32,
        )
    }

    pub fn camera(&self) -> macroquad::camera::Camera2D {
        macroquad::camera::Camera2D {
            target: vec2(self.logical_width * 0.5, self.logical_height * 0.5),
            zoom: vec2(2.0 / self.logical_width, -2.0 / self.logical_height),
            viewport: Some(self.viewport()),
            ..Default::default()
        }
    }

    pub fn begin(&self) {
        set_camera(&self.camera());
    }

    pub fn screen_to_ui(&self, point: Vec2) -> Vec2 {
        (point - self.offset) / self.scale
    }

    pub fn ui_to_screen(&self, point: Vec2) -> Vec2 {
        self.offset + point * self.scale
    }

    pub fn mouse_position(&self) -> Vec2 {
        let (mx, my) = mouse_position();
        self.screen_to_ui(vec2(mx, my))
    }

    pub fn is_mouse_inside(&self) -> bool {
        let pos = self.mouse_position();
        pos.x >= 0.0 && pos.y >= 0.0 && pos.x <= self.logical_width && pos.y <= self.logical_height
    }
}

/// Begin drawing in a fixed logical UI resolution.
///
/// Call `end_virtual_ui_frame()` after drawing.
pub fn begin_virtual_ui_frame(logical_width: f32, logical_height: f32) -> VirtualUi {
    let ui = VirtualUi::new(logical_width, logical_height);
    ui.begin();
    ui
}

/// Restore the default macroquad camera after `begin_virtual_ui_frame`.
pub fn end_virtual_ui_frame() {
    set_default_camera();
}

/// Convert current mouse position into a fixed logical UI resolution.
pub fn virtual_mouse_position(logical_width: f32, logical_height: f32) -> Vec2 {
    VirtualUi::new(logical_width, logical_height).mouse_position()
}

/// Style configuration for tooltip rendering.
#[derive(Debug, Clone)]
pub struct TooltipStyle {
    pub background: Color,
    pub border: Color,
    pub text: Color,
    pub padding: f32,
    pub max_width: f32,
    pub font_size: f32,
    pub line_gap: f32,
}

impl Default for TooltipStyle {
    fn default() -> Self {
        Self {
            background: Color::new(0.04, 0.04, 0.06, 0.94),
            border: dark::ACCENT,
            text: dark::TEXT,
            padding: 8.0,
            max_width: 320.0,
            font_size: 16.0,
            line_gap: 3.0,
        }
    }
}

/// Draw a tooltip near an anchor point, clamped inside the current screen.
pub fn draw_tooltip(text: &str, anchor: Vec2) -> Rect {
    draw_tooltip_styled(text, anchor, &TooltipStyle::default(), None)
}

/// Draw a font-aware tooltip near an anchor point, clamped inside the current screen.
pub fn draw_tooltip_styled(
    text: &str,
    anchor: Vec2,
    style: &TooltipStyle,
    font: Option<&Font>,
) -> Rect {
    let font = font.or(default_ui_font());
    let font_size = effective_font_size(style.font_size);
    let line_gap = effective_line_gap(style.line_gap);
    let lines = wrap_text_ex(
        text,
        (style.max_width - style.padding * 2.0).max(1.0),
        font,
        style.font_size,
    );
    let content_width = lines
        .iter()
        .map(|line| measure_text(line, font, font_size_u16(font_size), 1.0).width)
        .fold(0.0, f32::max);
    let content_height =
        lines.len() as f32 * font_size + lines.len().saturating_sub(1) as f32 * line_gap;
    let width = (content_width + style.padding * 2.0).min(style.max_width);
    let height = content_height + style.padding * 2.0;

    let mut rect = Rect::new(anchor.x + 14.0, anchor.y + 14.0, width, height);
    if rect.x + rect.w > screen_width() {
        rect.x = (screen_width() - rect.w - 6.0).max(6.0);
    }
    if rect.y + rect.h > screen_height() {
        rect.y = (screen_height() - rect.h - 6.0).max(6.0);
    }

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, style.background);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, style.border);

    let mut y = rect.y + style.padding + font_size;
    let text_style = TextStyle {
        font,
        font_size: style.font_size,
        color: style.text,
        line_gap: style.line_gap,
    };
    for line in &lines {
        draw_text_ex(line, rect.x + style.padding, y, text_style.params());
        y += font_size + line_gap;
    }

    rect
}

/// Draw a compact badge/chip.
pub fn draw_badge(rect: Rect, label: &str, fill: Color, text_color: Color) {
    let style = SurfaceStyle::new(fill).with_border(1.0, Color::new(1.0, 1.0, 1.0, 0.2));
    draw_surface(rect, &style);
    draw_text_centered_in_box(
        label,
        rect.x + 4.0,
        rect.y,
        rect.w - 8.0,
        rect.h,
        14.0,
        text_color,
    );
}

/// Draw a meter with optional centered label.
pub fn meter(rect: Rect, value: f32, max: f32, fill: Color, label: Option<&str>) {
    progress_bar(rect.x, rect.y, rect.w, rect.h, value, max, fill);
    if let Some(label) = label {
        draw_text_centered_in_box(
            label,
            rect.x + 4.0,
            rect.y,
            rect.w - 8.0,
            rect.h,
            14.0,
            dark::TEXT,
        );
    }
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
        Self {
            x,
            y,
            width,
            padding,
            cols,
            card_height,
        }
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
pub fn handle_scroll(
    current_scroll: f32,
    total_height: f32,
    view_height: f32,
    scroll_speed: f32,
) -> f32 {
    let (_, wheel_y) = mouse_wheel();
    let mut scroll = current_scroll;

    if wheel_y != 0.0 {
        scroll -= wheel_y * scroll_speed;
    }

    let max_scroll = (total_height - view_height).max(0.0);
    scroll.clamp(0.0, max_scroll)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_money_with_commas() {
        assert_eq!(format_money(0), "$0");
        assert_eq!(format_money(1_234), "$1,234");
        assert_eq!(format_money(-1_234_567), "-$1,234,567");
    }

    #[test]
    fn formats_compact_money() {
        assert_eq!(format_compact_money(999), "$999");
        assert_eq!(format_compact_money(12_000), "$12k");
        assert_eq!(format_compact_money(1_240_000), "$1.2m");
    }
}
