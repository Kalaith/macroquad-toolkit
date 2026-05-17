//! Generic event bus for decoupling game logic

/// Generic event bus for managing and processing events
///
/// # Example
/// ```
/// use macroquad_toolkit::events::EventBus;
///
/// enum GameEvent {
///     PlayerDied,
///     EnemySpawned,
/// }
///
/// let mut bus = EventBus::new();
/// bus.push(GameEvent::PlayerDied);
///
/// for event in bus.drain() {
///     // Process event
/// }
/// ```
pub struct EventBus<T> {
    events: Vec<T>,
}

impl<T> EventBus<T> {
    /// Create a new empty event bus
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Push an event to the bus
    pub fn push(&mut self, event: T) {
        self.events.push(event);
    }

    /// Drain all events from the bus for processing
    /// Returns an iterator over all events, clearing the bus
    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.events.drain(..)
    }

    /// Clear all events without processing
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Check if there are any pending events
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Get the number of pending events
    pub fn len(&self) -> usize {
        self.events.len()
    }
}

impl<T> Default for EventBus<T> {
    fn default() -> Self {
        Self::new()
    }
}
