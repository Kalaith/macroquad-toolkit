//! Random Number Generation utilities
//!
//! Wraps macroquad::rand to provide a consistent interface and helper functions.
//! Replaces direct usage of the `rand` crate to ensure WebGL compatibility.

use macroquad::rand;

/// Generate a random float between 0.0 and 1.0 (exclusive)
pub fn rand() -> f32 {
    rand::gen_range(0.0, 1.0)
}

/// Generate a random value within a range
/// Supports floats (0.0, 1.0) and integers (0, 10)
pub fn gen_range<T>(low: T, high: T) -> T
where
    T: macroquad::rand::RandomRange,
{
    rand::gen_range(low, high)
}

/// Return true with a given probability (0.0 to 1.0)
pub fn chance(probability: f32) -> bool {
    rand::gen_range(0.0, 1.0) < probability
}

/// Shuffle a slice in place using Fisher-Yates algorithm
pub fn shuffle<T>(slice: &mut [T]) {
    for i in (1..slice.len()).rev() {
        // gen_range(0, i + 1) generates int in [0, i] because high is exclusive?
        // macroquad::rand::gen_range for integers is [low, high) i.e. exclusive.
        // So gen_range(0, i + 1) gives 0..=i
        let j = rand::gen_range(0, i + 1);
        slice.swap(i, j);
    }
}

/// Pick a random element from a slice
pub fn choose<T>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        None
    } else {
        Some(&slice[rand::gen_range(0, slice.len())])
    }
}

/// Pick a random mutable element from a slice
pub fn choose_mut<T>(slice: &mut [T]) -> Option<&mut T> {
    if slice.is_empty() {
        None
    } else {
        let len = slice.len();
        Some(&mut slice[rand::gen_range(0, len)])
    }
}
