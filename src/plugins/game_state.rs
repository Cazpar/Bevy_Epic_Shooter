use bevy::prelude::*;
use crate::resources::game_state::{GameState, GameData};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            // Register the GameState type as a Bevy State
            .add_state::<GameState>()
            // Insert the GameData resource
            .init_resource::<GameData>()
            // Add systems for handling state transitions
            .add_systems(Update, handle_pause_input.run_if(in_state(GameState::Playing)))
            .add_systems(Update, handle_resume_input.run_if(in_state(GameState::Paused)))
            .add_systems(Update, handle_restart_input.run_if(in_state(GameState::GameOver)))
            .add_systems(Update, handle_start_game_input.run_if(in_state(GameState::MainMenu)))
            .add_systems(Update, handle_next_level.run_if(in_state(GameState::LevelComplete)));
    }
}

// System to handle pausing the game
fn handle_pause_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

// System to handle resuming the game
fn handle_resume_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) || keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

// System to handle restarting the game after game over
fn handle_restart_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<GameData>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Return) {
        game_data.reset();
        next_state.set(GameState::Playing);
    }
}

// System to handle starting the game from the main menu
fn handle_start_game_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Return) {
        next_state.set(GameState::Playing);
    }
}

// System to handle proceeding to the next level
fn handle_next_level(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<GameData>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Return) {
        game_data.next_level();
        next_state.set(GameState::Playing);
    }
} 