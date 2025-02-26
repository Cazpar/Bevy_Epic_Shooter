use bevy::prelude::*;
use crate::systems::pickup::{spawn_enemy_drops, update_pickups, handle_pickup_collection};

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_enemy_drops)
            .add_systems(Update, update_pickups)
            .add_systems(Update, handle_pickup_collection);
    }
} 