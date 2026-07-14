//! Transient visual effects: screen shake, screen fades, particles, and
//! floating text.
//!
//! Extracted from per-game implementations in scrapyard, nightmare_shift,
//! alchemy_tower, kaiju_sim, nanite_swarm, carriage_run, apartment,
//! dungeon_core, and feast_frenzy.

mod fade;
mod shake;

pub use fade::*;
pub use shake::*;
