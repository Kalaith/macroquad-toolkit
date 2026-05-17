//! Generic State Machine for managing game screens
//!
//! Provides a `GameState` trait and `StateManager` for handling transitions.

use std::any::Any;

/// Trait representing a single game state (screen)
///
/// T is the data context passed to update/draw (e.g., your generic Game struct)
pub trait GameState<T> {
    /// Update the state logic. Returns an optional transition.
    fn update(&mut self, context: &mut T) -> Option<Box<dyn Any>>;

    /// Draw the state
    fn draw(&self, context: &T);

    /// Called when directly entering this state
    fn on_enter(&mut self, _context: &mut T) {}

    /// Called when leaving this state
    fn on_exit(&mut self, _context: &mut T) {}
}

/// Simple enum for basic state transitions if you don't need complex data passing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transition {
    None,
    Push,   // Push new state (if using stack)
    Pop,    // Pop current state (if using stack)
    Switch, // Replace current state
    Quit,
}

// Helper for state machine management could be added here,
// but often games prefer their own explicit Enum matching (like generic `Game` struct).
//
// This module primarily provides the Trait definition to standardize what a "Screen" looks like.
