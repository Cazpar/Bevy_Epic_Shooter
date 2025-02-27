use bevy::prelude::*;
use crate::systems::enemy::*;
use crate::resources::game_state::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add enemy movement system that only runs in the Playing state
            .add_systems(
                Update, 
                (
                    enemy_movement,
                    update_enemy_animation
                ).run_if(in_state(GameState::Playing))
            )
            // Spawn enemies every 2 seconds, but only in the Playing state
            .add_systems(
                Update, 
                spawn_enemy
                    .run_if(on_timer(std::time::Duration::from_secs_f32(2.0)))
                    .run_if(in_state(GameState::Playing))
            );
    }
}

// Helper function to run a system on a timer
fn on_timer(duration: std::time::Duration) -> impl FnMut() -> bool {
    let mut timer = Timer::new(duration, TimerMode::Repeating);
    move || {
        timer.tick(std::time::Duration::from_secs_f32(1.0 / 60.0)); // Assuming 60 FPS
        timer.just_finished()
    }
} 