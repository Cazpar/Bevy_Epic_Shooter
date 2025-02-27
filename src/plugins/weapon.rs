use bevy::prelude::*;
use crate::systems::weapon::*;
use crate::resources::game_state::GameState;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add weapon systems that only run in the Playing state
            .add_systems(
                Update, 
                (
                    player_shooting,
                    projectile_movement,
                    handle_projectile_enemy_collision,
                    handle_projectile_obstacle_damage
                ).run_if(in_state(GameState::Playing))
            );
    }
} 