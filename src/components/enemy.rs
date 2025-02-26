use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
    pub rotation: f32,
    pub enemy_type: EnemyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Basic,
    Fast,
    Tank,
    Shooter,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        match enemy_type {
            EnemyType::Basic => Self {
                health: 50.0,
                speed: 100.0,
                damage: 10.0,
                rotation: 0.0,
                enemy_type,
            },
            EnemyType::Fast => Self {
                health: 30.0,
                speed: 150.0,
                damage: 5.0,
                rotation: 0.0,
                enemy_type,
            },
            EnemyType::Tank => Self {
                health: 150.0,
                speed: 50.0,
                damage: 20.0,
                rotation: 0.0,
                enemy_type,
            },
            EnemyType::Shooter => Self {
                health: 40.0,
                speed: 80.0,
                damage: 15.0,
                rotation: 0.0,
                enemy_type,
            },
        }
    }
} 