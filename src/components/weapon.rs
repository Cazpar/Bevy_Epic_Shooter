use bevy::prelude::*;

#[derive(Component)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage: f32,
    pub attack_speed: f32, // Attacks per second
    pub attack_range: f32, // Range of the melee attack
    pub last_attack: f32,  // Time since last attack
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    Dagger,
    Sword,
    Axe,
    Hammer,
}

impl Weapon {
    pub fn new(weapon_type: WeaponType) -> Self {
        match weapon_type {
            WeaponType::Dagger => Self {
                weapon_type,
                damage: 8.0,
                attack_speed: 2.5,
                attack_range: 40.0,
                last_attack: 0.0,
            },
            WeaponType::Sword => Self {
                weapon_type,
                damage: 15.0,
                attack_speed: 1.5,
                attack_range: 60.0,
                last_attack: 0.0,
            },
            WeaponType::Axe => Self {
                weapon_type,
                damage: 20.0,
                attack_speed: 1.0,
                attack_range: 50.0,
                last_attack: 0.0,
            },
            WeaponType::Hammer => Self {
                weapon_type,
                damage: 30.0,
                attack_speed: 0.7,
                attack_range: 45.0,
                last_attack: 0.0,
            },
        }
    }
    
    pub fn can_attack(&self, time: f32) -> bool {
        time - self.last_attack >= 1.0 / self.attack_speed
    }
}

#[derive(Component)]
pub struct MeleeAttack {
    pub damage: f32,
    pub weapon_type: WeaponType,
    pub lifetime: f32, // How long the attack hitbox exists in seconds
    pub angle: f32,    // Attack angle in radians
    pub width: f32,    // Width of the attack arc
}

// Component to mark entities that are currently attacking
#[derive(Component)]
pub struct Attacking {
    pub timer: Timer,
}

impl Attacking {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
} 