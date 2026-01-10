//! Color palettes for consistent game UI theming

/// Dark theme color palette - suitable for most game UIs
pub mod dark {
    use macroquad::prelude::Color;

    pub const BACKGROUND: Color = Color::new(0.12, 0.12, 0.14, 1.0);
    pub const PANEL: Color = Color::new(0.18, 0.18, 0.22, 1.0);
    pub const PANEL_HEADER: Color = Color::new(0.22, 0.22, 0.28, 1.0);

    pub const TEXT: Color = Color::new(0.9, 0.9, 0.9, 1.0);
    pub const TEXT_BRIGHT: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    pub const TEXT_DIM: Color = Color::new(0.6, 0.6, 0.6, 1.0);

    pub const ACCENT: Color = Color::new(0.3, 0.6, 0.9, 1.0);
    pub const POSITIVE: Color = Color::new(0.3, 0.8, 0.4, 1.0);
    pub const WARNING: Color = Color::new(0.9, 0.7, 0.2, 1.0);
    pub const NEGATIVE: Color = Color::new(0.9, 0.3, 0.3, 1.0);

    pub const HOVERED: Color = Color::new(0.3, 0.4, 0.55, 1.0);
}

/// Rarity color palette - for items, equipment, loot in RPG-style games
pub mod rarity {
    use macroquad::prelude::Color;

    pub const COMMON: Color = Color::new(0.6, 0.6, 0.6, 1.0);
    pub const UNCOMMON: Color = Color::new(0.3, 0.7, 0.3, 1.0);
    pub const RARE: Color = Color::new(0.3, 0.5, 0.9, 1.0);
    pub const EPIC: Color = Color::new(0.6, 0.3, 0.9, 1.0);
    pub const LEGENDARY: Color = Color::new(0.9, 0.6, 0.2, 1.0);
}
