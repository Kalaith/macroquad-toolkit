//! Transient visual effects: screen shake, screen fades, particles,
//! floating text, and traveling projectiles.
//!
//! Extracted from per-game implementations in scrapyard, nightmare_shift,
//! alchemy_tower, kaiju_sim, nanite_swarm, carriage_run, apartment,
//! dungeon_core, and feast_frenzy.

mod crt;
mod fade;
mod floating_text;
mod particles;
mod shake;
mod travel;

pub use crt::*;
pub use fade::*;
pub use floating_text::*;
pub use particles::*;
pub use shake::*;
pub use travel::*;
