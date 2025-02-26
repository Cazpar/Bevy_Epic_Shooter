use bevy::prelude::*;

#[derive(Component)]
pub struct Obstacle {
    pub obstacle_type: ObstacleType,
    pub size: Vec2,
    pub health: f32,
    pub max_health: f32,
    pub is_destructible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObstacleType {
    Rock,
    Crate,
    Bush,
}

impl Obstacle {
    pub fn new(obstacle_type: ObstacleType) -> Self {
        let size = match obstacle_type {
            ObstacleType::Rock => Vec2::new(32.0, 32.0),
            ObstacleType::Crate => Vec2::new(32.0, 32.0),
            ObstacleType::Bush => Vec2::new(32.0, 32.0),
        };
        
        let (health, is_destructible) = match obstacle_type {
            ObstacleType::Rock => (100.0, false),  // Rocks are indestructible
            ObstacleType::Crate => (50.0, true),   // Crates can be destroyed
            ObstacleType::Bush => (20.0, true),    // Bushes can be destroyed but projectiles pass through
        };
        
        Self {
            obstacle_type,
            size,
            health,
            max_health: health,
            is_destructible,
        }
    }
    
    pub fn take_damage(&mut self, damage: f32) -> bool {
        if !self.is_destructible {
            return false;
        }
        
        self.health -= damage;
        self.health = self.health.max(0.0);
        
        self.health <= 0.0 // Returns true if the obstacle is destroyed
    }
} 