use bevy::prelude::*;

/// Component to mark entities that have recently collided
#[derive(Component)]
pub struct CollisionDebug {
    pub timer: Timer,
}

impl Default for CollisionDebug {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
} 