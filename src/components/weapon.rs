use bevy::prelude::*;

#[derive(Component)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage: f32,
    pub fire_rate: f32, // Shots per second
    pub projectile_speed: f32,
    pub last_shot: f32, // Time since last shot
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    Pistol,
    Shotgun,
    MachineGun,
    RocketLauncher,
}

impl Weapon {
    pub fn new(weapon_type: WeaponType) -> Self {
        match weapon_type {
            WeaponType::Pistol => Self {
                weapon_type,
                damage: 10.0,
                fire_rate: 2.0,
                projectile_speed: 500.0,
                last_shot: 0.0,
            },
            WeaponType::Shotgun => Self {
                weapon_type,
                damage: 5.0,
                fire_rate: 1.0,
                projectile_speed: 400.0,
                last_shot: 0.0,
            },
            WeaponType::MachineGun => Self {
                weapon_type,
                damage: 5.0,
                fire_rate: 8.0,
                projectile_speed: 600.0,
                last_shot: 0.0,
            },
            WeaponType::RocketLauncher => Self {
                weapon_type,
                damage: 30.0,
                fire_rate: 0.5,
                projectile_speed: 300.0,
                last_shot: 0.0,
            },
        }
    }
    
    pub fn can_shoot(&self, time: f32) -> bool {
        time - self.last_shot >= 1.0 / self.fire_rate
    }
}

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub lifetime: f32, // How long the projectile lives in seconds
    pub weapon_type: WeaponType,
} 