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
                    player_attacking,
                    update_attacking_state,
                    handle_melee_attack_collisions
                ).run_if(in_state(GameState::Playing))
            );
    }
} 