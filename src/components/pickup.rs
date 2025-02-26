use bevy::prelude::*;
use crate::components::weapon::WeaponType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickupType {
    Weapon(WeaponType),
    DoubleShot,
    TripleShot,
    RapidFire,
    IncreasedDamage,
    HealthPack,
}

#[derive(Component)]
pub struct Pickup {
    pub pickup_type: PickupType,
    pub lifetime: f32, // How long the pickup stays in the world
    pub rotation_speed: f32, // For visual effect
}

impl Pickup {
    pub fn new(pickup_type: PickupType) -> Self {
        Self {
            pickup_type,
            lifetime: 15.0, // 15 seconds by default
            rotation_speed: 2.0,
        }
    }
}

// Component to track weapon upgrades
#[derive(Component, Default)]
pub struct WeaponUpgrades {
    pub double_shot: bool,
    pub triple_shot: bool,
    pub rapid_fire_multiplier: f32, // 1.0 = normal, 2.0 = twice as fast
    pub damage_multiplier: f32,     // 1.0 = normal, 2.0 = twice the damage
}

impl WeaponUpgrades {
    pub fn new() -> Self {
        Self {
            double_shot: false,
            triple_shot: false,
            rapid_fire_multiplier: 1.0,
            damage_multiplier: 1.0,
        }
    }
} 