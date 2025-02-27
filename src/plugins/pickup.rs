use bevy::prelude::*;
use crate::systems::pickup::{spawn_enemy_drops, update_pickups, handle_pickup_collection};
use crate::resources::game_state::GameState;

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add pickup systems that only run in the Playing state
            .add_systems(Update, spawn_enemy_drops.run_if(in_state(GameState::Playing)))
            .add_systems(Update, update_pickups.run_if(in_state(GameState::Playing)))
            .add_systems(Update, handle_pickup_collection.run_if(in_state(GameState::Playing)));
    }
} 