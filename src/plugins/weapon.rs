use bevy::prelude::*;
use crate::systems::weapon::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, player_shooting)
            .add_systems(Update, projectile_movement)
            .add_systems(Update, handle_projectile_enemy_collision)
            .add_systems(Update, handle_projectile_obstacle_damage);
    }
} 