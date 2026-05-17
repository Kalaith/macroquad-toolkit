//! Sprite utilities for rendering textures with transformations

use macroquad::prelude::*;

/// Sprite struct for robust scaling, rotation, and tinting of textures
///
/// # Example
/// ```no_run
/// use macroquad_toolkit::sprite::Sprite;
/// use macroquad::prelude::*;
///
/// # async fn example() {
/// # let texture = load_texture("player.png").await.unwrap();
/// let sprite = Sprite::new()
///     .with_texture(texture)
///     .at(100.0, 100.0)
///     .scaled(2.0, 2.0)
///     .rotated(0.5);
///
/// sprite.draw();
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Sprite {
    pub texture: Option<Texture2D>,
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32, // Radians
    pub origin: Vec2,  // Pivot point (0.5, 0.5 for center)
    pub color: Color,
}

impl Sprite {
    /// Create a new sprite with default values
    pub fn new() -> Self {
        Self {
            texture: None,
            position: Vec2::ZERO,
            scale: Vec2::ONE,
            rotation: 0.0,
            origin: vec2(0.5, 0.5),
            color: WHITE,
        }
    }

    /// Set the texture for this sprite (builder pattern)
    pub fn with_texture(mut self, texture: Texture2D) -> Self {
        self.texture = Some(texture);
        self
    }

    /// Set the position (builder pattern)
    pub fn at(mut self, x: f32, y: f32) -> Self {
        self.position = vec2(x, y);
        self
    }

    /// Set the scale (builder pattern)
    pub fn scaled(mut self, sx: f32, sy: f32) -> Self {
        self.scale = vec2(sx, sy);
        self
    }

    /// Set the rotation in radians (builder pattern)
    pub fn rotated(mut self, angle: f32) -> Self {
        self.rotation = angle;
        self
    }

    /// Set the tint color (builder pattern)
    pub fn colored(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Alias for colored
    pub fn tinted(self, color: Color) -> Self {
        self.colored(color)
    }

    /// Set the origin/pivot point (builder pattern)
    pub fn with_origin(mut self, ox: f32, oy: f32) -> Self {
        self.origin = vec2(ox, oy);
        self
    }

    /// Draw the sprite
    pub fn draw(&self) {
        if let Some(tex) = &self.texture {
            let w = tex.width() * self.scale.x;
            let h = tex.height() * self.scale.y;
            let ox = w * self.origin.x;
            let oy = h * self.origin.y;

            draw_texture_ex(
                tex,
                self.position.x - ox,
                self.position.y - oy,
                self.color,
                DrawTextureParams {
                    dest_size: Some(vec2(w, h)),
                    rotation: self.rotation,
                    pivot: Some(self.position),
                    ..Default::default()
                },
            );
        }
    }

    /// Draw a colored rectangle as a placeholder when no texture is available
    pub fn draw_placeholder(&self, width: f32, height: f32, color: Color) {
        let w = width * self.scale.x;
        let h = height * self.scale.y;
        let ox = w * self.origin.x;
        let oy = h * self.origin.y;

        draw_rectangle(self.position.x - ox, self.position.y - oy, w, h, color);
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Self::new()
    }
}
