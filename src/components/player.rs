// src/components/player.rs
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub speed: f32,
    pub rotation: f32,
}

// src/systems/player.rs
use bevy::prelude::*;
use crate::components::player::Player;
use crate::resources::game_state::GameState;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    game_state: Res<GameState>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        let mut direction = Vec2::ZERO;
        
        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        
        if direction != Vec2::ZERO {
            direction = direction.normalize();
            transform.translation.x += direction.x * player.speed * time.delta_seconds();
            transform.translation.y += direction.y * player.speed * time.delta_seconds();
            
            // Update player rotation based on movement direction
            player.rotation = direction.y.atan2(direction.x);
            transform.rotation = Quat::from_rotation_z(player.rotation);
        }
    }
}