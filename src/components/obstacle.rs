use bevy::prelude::*;

#[derive(Component)]
pub struct Obstacle {
    pub obstacle_type: ObstacleType,
    pub size: Vec2,
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
        
        Self {
            obstacle_type,
            size,
        }
    }
} 