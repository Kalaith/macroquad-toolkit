//! Terminal-style character-by-character text reveal.
//!
//! Pure, GL-free helpers: given the full text, seconds elapsed since the reveal
//! began, and a characters-per-second rate, they return how much of the string
//! should be visible. Callers draw the returned prefix however they like (word
//! wrap, cursor, etc.). Counting is by `char`, so multi-byte UTF-8 never splits.
//!
//! A non-positive rate reveals everything immediately, which is handy for
//! disabling the effect (accessibility, screenshot capture) without branching at
//! every call site.

/// Number of `char`s of `text` visible after `elapsed` seconds at
/// `chars_per_sec`. Saturates at the full length; a non-positive rate returns
/// the full length immediately.
pub fn typed_char_count(text: &str, elapsed: f32, chars_per_sec: f32) -> usize {
    let total = text.chars().count();
    if chars_per_sec <= 0.0 {
        return total;
    }
    let shown = elapsed.max(0.0) * chars_per_sec;
    if !shown.is_finite() || shown >= total as f32 {
        return total;
    }
    shown.floor() as usize
}

/// The visible prefix of `text` — always a valid slice on a `char` boundary.
pub fn typed_prefix(text: &str, elapsed: f32, chars_per_sec: f32) -> &str {
    let n = typed_char_count(text, elapsed, chars_per_sec);
    match text.char_indices().nth(n) {
        Some((byte_idx, _)) => &text[..byte_idx],
        None => text,
    }
}

/// True once every `char` of `text` is visible.
pub fn is_fully_typed(text: &str, elapsed: f32, chars_per_sec: f32) -> bool {
    typed_char_count(text, elapsed, chars_per_sec) >= text.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reveals_progressively_then_saturates() {
        let text = "hello world";
        assert_eq!(typed_char_count(text, 0.0, 10.0), 0);
        assert_eq!(typed_char_count(text, 0.5, 10.0), 5);
        assert_eq!(typed_char_count(text, 100.0, 10.0), text.len());
        assert!(is_fully_typed(text, 100.0, 10.0));
    }

    #[test]
    fn non_positive_rate_reveals_all_immediately() {
        let text = "instant";
        assert_eq!(typed_char_count(text, 0.0, 0.0), 7);
        assert_eq!(typed_prefix(text, 0.0, -1.0), "instant");
        assert!(is_fully_typed(text, 0.0, 0.0));
    }

    #[test]
    fn prefix_respects_char_boundaries() {
        // Each accented glyph is multi-byte; the prefix must stay valid UTF-8.
        let text = "café⚙ok";
        for e in 0..20 {
            let p = typed_prefix(text, e as f32 * 0.1, 5.0);
            assert!(text.starts_with(p), "prefix {p:?} not a prefix of {text:?}");
        }
        assert_eq!(typed_prefix(text, 0.6, 5.0), "caf");
    }

    #[test]
    fn empty_text_is_always_done() {
        assert_eq!(typed_char_count("", 0.0, 10.0), 0);
        assert!(is_fully_typed("", 0.0, 10.0));
        assert_eq!(typed_prefix("", 5.0, 10.0), "");
    }
}
