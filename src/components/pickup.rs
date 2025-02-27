use bevy::prelude::*;
use crate::components::weapon::WeaponType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickupType {
    Weapon(WeaponType),
    AreaAttack,     // Increases attack area
    CriticalHit,    // Chance for critical hits
    AttackSpeed,    // Faster attack speed
    IncreasedDamage, // More damage
    HealthPack,     // Restore health
    StaminaBoost,   // Increase stamina/energy
    ArmorBoost,     // Damage reduction
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
    pub area_attack: bool,       // Wider attack area
    pub critical_hit_chance: f32, // Chance to deal critical damage (0.0 to 1.0)
    pub attack_speed_multiplier: f32, // 1.0 = normal, 2.0 = twice as fast
    pub damage_multiplier: f32,     // 1.0 = normal, 2.0 = twice the damage
}

impl WeaponUpgrades {
    pub fn new() -> Self {
        Self {
            area_attack: false,
            critical_hit_chance: 0.0,
            attack_speed_multiplier: 1.0,
            damage_multiplier: 1.0,
        }
    }
} 