// src/main.rs
use bevy::prelude::*;

mod plugins;
mod components;
mod resources;
mod systems;
// mod events;

use resources::game_state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugins::GamePlugins)
        .insert_resource(GameState::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Setup camera and initial game state
    commands.spawn(Camera2dBundle::default());
}