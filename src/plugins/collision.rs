use bevy::prelude::*;
use crate::systems::collision::{
    handle_player_obstacle_collision,
    handle_enemy_obstacle_collision,
    handle_projectile_obstacle_collision,
    update_collision_debug,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                handle_player_obstacle_collision,
                handle_enemy_obstacle_collision,
                handle_projectile_obstacle_collision,
            ))
            // Run the debug update system at the end of the frame to avoid issues with despawned entities
            .add_systems(PostUpdate, update_collision_debug);
    }
} 