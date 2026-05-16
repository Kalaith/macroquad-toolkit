//! Notification/Toast system for game events
//!
//! Provides a queue-based notification system with fade-out animations
//! and multiple notification types for styling.
//!
//! # Example
//! ```
//! use macroquad_toolkit::notifications::{NotificationManager, NotificationType};
//!
//! let mut notifications = NotificationManager::new();
//!
//! notifications.success("Level completed!");
//! notifications.warning("Low health!");
//! notifications.danger("Enemy approaching!");
//!
//! // In game loop:
//! // notifications.update(delta_time);
//! // for notif in notifications.get_notifications() {
//! //     // render notification with notif.opacity() for fade effect
//! // }
//! ```

use serde::{Deserialize, Serialize};

/// Type of notification for styling purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum NotificationType {
    /// Positive event (achievement, level up, victory)
    Success,
    /// Neutral information (hint, status update)
    #[default]
    Info,
    /// Warning (low resources, approaching danger)
    Warning,
    /// Negative event (damage taken, item lost, defeat)
    Danger,
}

/// A single notification message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// The message text
    pub message: String,
    /// Type of notification for styling
    pub notification_type: NotificationType,
    /// Time remaining before this notification disappears (seconds)
    pub time_remaining: f32,
    /// Total duration for fade calculations
    pub total_duration: f32,
}

impl Notification {
    /// Create a new notification
    pub fn new(message: String, notification_type: NotificationType, duration: f32) -> Self {
        Self {
            message,
            notification_type,
            time_remaining: duration,
            total_duration: duration,
        }
    }

    /// Get opacity for fade-out effect (1.0 = fully visible, 0.0 = invisible)
    ///
    /// Starts fading when 1 second remains
    pub fn opacity(&self) -> f32 {
        let fade_start = 1.0;
        if self.time_remaining > fade_start {
            1.0
        } else {
            (self.time_remaining / fade_start).max(0.0)
        }
    }

    /// Check if this notification has expired
    pub fn is_expired(&self) -> bool {
        self.time_remaining <= 0.0
    }

    /// Get progress (0.0 = just started, 1.0 = expired)
    pub fn progress(&self) -> f32 {
        1.0 - (self.time_remaining / self.total_duration).clamp(0.0, 1.0)
    }
}

/// Default notification duration in seconds
pub const DEFAULT_DURATION: f32 = 4.0;
/// Maximum number of notifications to display at once
pub const MAX_NOTIFICATIONS: usize = 5;

/// Manages the notification queue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationManager {
    notifications: Vec<Notification>,
    #[serde(skip, default)]
    max_notifications: usize,
    #[serde(skip, default)]
    default_duration: f32,
}

impl NotificationManager {
    /// Create a new notification manager with default settings
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
            max_notifications: MAX_NOTIFICATIONS,
            default_duration: DEFAULT_DURATION,
        }
    }

    /// Create with custom settings
    pub fn with_settings(max_notifications: usize, default_duration: f32) -> Self {
        Self {
            notifications: Vec::new(),
            max_notifications,
            default_duration,
        }
    }

    /// Add a notification with default duration
    pub fn push(&mut self, message: impl Into<String>, notification_type: NotificationType) {
        self.push_with_duration(message, notification_type, self.default_duration);
    }

    /// Add a notification with custom duration
    pub fn push_with_duration(
        &mut self,
        message: impl Into<String>,
        notification_type: NotificationType,
        duration: f32,
    ) {
        let notification = Notification::new(message.into(), notification_type, duration);
        self.notifications.push(notification);

        // Trim oldest if over limit
        while self.notifications.len() > self.max_notifications {
            self.notifications.remove(0);
        }
    }

    /// Add a success notification
    pub fn success(&mut self, message: impl Into<String>) {
        self.push(message, NotificationType::Success);
    }

    /// Add an info notification
    pub fn info(&mut self, message: impl Into<String>) {
        self.push(message, NotificationType::Info);
    }

    /// Add a warning notification
    pub fn warning(&mut self, message: impl Into<String>) {
        self.push(message, NotificationType::Warning);
    }

    /// Add a danger notification
    pub fn danger(&mut self, message: impl Into<String>) {
        self.push(message, NotificationType::Danger);
    }

    /// Update all notifications (call every frame)
    pub fn update(&mut self, dt: f32) {
        for notification in &mut self.notifications {
            notification.time_remaining -= dt;
        }

        // Remove expired notifications
        self.notifications.retain(|n| n.time_remaining > 0.0);
    }

    /// Get all active notifications for rendering
    pub fn get_notifications(&self) -> &[Notification] {
        &self.notifications
    }

    /// Get notifications as mutable slice
    pub fn get_notifications_mut(&mut self) -> &mut [Notification] {
        &mut self.notifications
    }

    /// Get the number of active notifications
    pub fn count(&self) -> usize {
        self.notifications.len()
    }

    /// Check if there are any notifications
    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }

    /// Clear all notifications
    pub fn clear(&mut self) {
        self.notifications.clear();
    }

    /// Remove the oldest notification
    pub fn pop_oldest(&mut self) -> Option<Notification> {
        if !self.notifications.is_empty() {
            Some(self.notifications.remove(0))
        } else {
            None
        }
    }

    /// Remove the newest notification
    pub fn pop_newest(&mut self) -> Option<Notification> {
        self.notifications.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_opacity() {
        let notif = Notification::new("Test".to_string(), NotificationType::Info, 4.0);
        assert!((notif.opacity() - 1.0).abs() < 0.001);

        let mut notif2 = Notification::new("Test".to_string(), NotificationType::Info, 4.0);
        notif2.time_remaining = 0.5;
        assert!((notif2.opacity() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_notification_manager() {
        let mut manager = NotificationManager::new();

        manager.success("Test 1");
        manager.warning("Test 2");

        assert_eq!(manager.count(), 2);

        // Update to expire first notification
        manager.update(5.0);
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_max_notifications() {
        let mut manager = NotificationManager::with_settings(3, 4.0);

        for i in 0..5 {
            manager.info(format!("Test {}", i));
        }

        assert_eq!(manager.count(), 3);
        // Should have Test 2, 3, 4 (oldest removed)
        assert_eq!(manager.get_notifications()[0].message, "Test 2");
    }
}
