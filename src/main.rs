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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Epic Shooter".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(plugins::GamePlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Setup camera and initial game state
    commands.spawn(Camera2dBundle::default());
}