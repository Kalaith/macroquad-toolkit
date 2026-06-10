//! Random Number Generation utilities
//!
//! Wraps macroquad::rand to provide a consistent interface and helper functions.
//! Replaces direct usage of the `rand` crate to ensure WebGL compatibility.

use macroquad::rand;

/// Small deterministic RNG for reproducible generation.
#[derive(Debug, Clone, Copy)]
pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        let init = seed ^ 0x9E3779B97F4A7C15;
        Self { state: init }
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    pub fn next_f32(&mut self) -> f32 {
        let value = self.next_u64() >> 40;
        (value as f32) / ((1u64 << 24) as f32)
    }
}

/// Generate a random float between 0.0 and 1.0 (exclusive)
pub fn rand() -> f32 {
    rand::gen_range(0.0, 1.0)
}

/// Seed Macroquad's shared random generator.
pub fn srand(seed: u64) {
    rand::srand(seed);
}

/// Generate a random `u64`, useful for visual seeds and IDs.
pub fn random_u64() -> u64 {
    rand::gen_range(0u64, u64::MAX)
}

/// Generate a random `u32`, useful for compact IDs and legacy helper APIs.
pub fn random_u32() -> u32 {
    rand::gen_range(0u32, u32::MAX)
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
    rand::gen_range(0.0, 1.0) < probability.clamp(0.0, 1.0)
}

/// Return true with a whole-number percentage chance from 0 to 100.
pub fn chance_percent(percent: i32) -> bool {
    rand::gen_range(0, 100) < percent.clamp(0, 100)
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

/// Pick up to `count` random unique elements from a slice.
pub fn choose_multiple<T>(slice: &[T], count: usize) -> Vec<&T> {
    if slice.is_empty() || count == 0 {
        return Vec::new();
    }

    let mut indices: Vec<usize> = (0..slice.len()).collect();
    shuffle(&mut indices);

    indices
        .into_iter()
        .take(count.min(slice.len()))
        .map(|index| &slice[index])
        .collect()
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

#[cfg(test)]
mod tests {
    use super::SeededRng;

    #[test]
    fn seeded_rng_is_repeatable() {
        let mut a = SeededRng::new(42);
        let mut b = SeededRng::new(42);

        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn seeded_rng_float_is_unit_interval() {
        let mut rng = SeededRng::new(7);
        for _ in 0..16 {
            let value = rng.next_f32();
            assert!((0.0..=1.0).contains(&value));
        }
    }
}
