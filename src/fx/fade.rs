//! Full-screen fade in/out overlay for scene transitions.

use macroquad::prelude::{draw_rectangle, screen_height, screen_width, Color, BLACK};

use crate::colors::with_alpha;

/// Direction a [`ScreenFade`] is currently animating.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FadeDirection {
    /// Overlay alpha rising toward opaque (screen fading to the fade color).
    Out,
    /// Overlay alpha falling toward transparent (screen becoming visible).
    In,
}

/// A full-screen fade overlay for scene transitions.
///
/// Call [`update`](ScreenFade::update) each frame and
/// [`draw`](ScreenFade::draw) after all other rendering. A typical scene
/// switch: `fade_out()`, wait for `update` to return true, swap scenes,
/// then `fade_in()`.
///
/// Extracted from nightmare_shift's `ScreenTransition`.
#[derive(Debug, Clone, Copy)]
pub struct ScreenFade {
    alpha: f32,
    direction: Option<FadeDirection>,
    /// Seconds for a full fade.
    pub duration: f32,
    /// Overlay color (alpha channel is ignored).
    pub color: Color,
}

impl ScreenFade {
    /// Creates a transparent black fade with the given duration in seconds.
    pub fn new(duration: f32) -> Self {
        Self {
            alpha: 0.0,
            direction: None,
            duration: duration.max(0.001),
            color: BLACK,
        }
    }

    /// Starts fading the screen out (overlay becomes opaque).
    pub fn fade_out(&mut self) {
        self.direction = Some(FadeDirection::Out);
    }

    /// Starts fading the screen in (overlay becomes transparent).
    pub fn fade_in(&mut self) {
        self.direction = Some(FadeDirection::In);
    }

    /// Starts fully opaque and fading in — for the first frame of a new scene.
    pub fn begin_scene(&mut self) {
        self.alpha = 1.0;
        self.fade_in();
    }

    /// Advances the fade. Returns true on the frame the active fade completes.
    pub fn update(&mut self, dt: f32) -> bool {
        let Some(direction) = self.direction else {
            return false;
        };
        let step = dt / self.duration;
        match direction {
            FadeDirection::Out => {
                self.alpha = (self.alpha + step).min(1.0);
                if self.alpha >= 1.0 {
                    self.direction = None;
                    return true;
                }
            }
            FadeDirection::In => {
                self.alpha = (self.alpha - step).max(0.0);
                if self.alpha <= 0.0 {
                    self.direction = None;
                    return true;
                }
            }
        }
        false
    }

    /// Current overlay alpha in `[0, 1]`.
    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    /// True while a fade is animating.
    pub fn is_fading(&self) -> bool {
        self.direction.is_some()
    }

    /// True while the overlay hides the screen at all (alpha > 0).
    pub fn is_visible(&self) -> bool {
        self.alpha > 0.0
    }

    /// The active direction, if any.
    pub fn direction(&self) -> Option<FadeDirection> {
        self.direction
    }

    /// Draws the overlay across the whole screen. No-op when fully transparent.
    pub fn draw(&self) {
        if self.alpha <= 0.0 {
            return;
        }
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            with_alpha(self.color, self.alpha),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fade_out_then_in_completes() {
        let mut fade = ScreenFade::new(0.5);
        assert!(!fade.update(0.1), "idle fade must not complete");

        fade.fade_out();
        assert!(fade.is_fading());
        assert!(!fade.update(0.25));
        assert!((fade.alpha() - 0.5).abs() < 1e-4);
        assert!(fade.update(0.3), "should complete when alpha reaches 1");
        assert!(!fade.is_fading());
        assert!(fade.is_visible());

        fade.fade_in();
        assert!(!fade.update(0.25));
        assert!(fade.update(0.3));
        assert!(!fade.is_visible());
    }

    #[test]
    fn begin_scene_starts_opaque() {
        let mut fade = ScreenFade::new(1.0);
        fade.begin_scene();
        assert!((fade.alpha() - 1.0).abs() < 1e-6);
        assert_eq!(fade.direction(), Some(FadeDirection::In));
    }
}
