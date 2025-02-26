use bevy::prelude::*;
use crate::components::weapon::WeaponType;

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub speed: f32,
    pub rotation: f32,
    pub current_weapon: WeaponType,
}
