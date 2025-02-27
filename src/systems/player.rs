use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::player::Player;
use crate::components::weapon::WeaponType;
use crate::components::animation::AnimationState;

pub fn player_movement(
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(&mut Transform, &mut Player, &mut Sprite)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok((mut transform, mut player, mut sprite)) = query.get_single_mut() {
        // Get the primary window
        let window = window_query.get_single().unwrap();

        // Get the camera
        let (camera, camera_transform) = camera_query.get_single().unwrap();

        // Calculate movement based on keyboard input
        let mut movement = Vec2::ZERO;

        // Forward/backward movement
        if keyboard_input.pressed(KeyCode::W) {
            movement.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            movement.y -= 1.0;
        }

        // Strafe movement
        if keyboard_input.pressed(KeyCode::A) {
            movement.x -= 1.0;
            // Flip sprite when moving left
            sprite.flip_x = true;
        }
        if keyboard_input.pressed(KeyCode::D) {
            movement.x += 1.0;
            // Don't flip sprite when moving right
            sprite.flip_x = false;
        }

        // If not moving horizontally, use mouse position for sprite flipping
        if movement.x == 0.0 {
            // Get the cursor position in the window
            if let Some(cursor_position) = window.cursor_position() {
                // Convert cursor position to world coordinates
                let cursor_world_position = camera
                    .viewport_to_world(camera_transform, cursor_position)
                    .map(|ray| ray.origin.truncate())
                    .unwrap_or_default();

                // Calculate direction to mouse (for determining sprite flip)
                let direction_to_mouse = cursor_world_position - transform.translation.truncate();
                
                // Flip sprite based on mouse position (for aiming)
                if direction_to_mouse.x < 0.0 {
                    sprite.flip_x = true;
                } else {
                    sprite.flip_x = false;
                }
            }
        }

        // Normalize movement vector to prevent diagonal movement from being faster
        if movement != Vec2::ZERO {
            movement = movement.normalize();
        }

        // Apply movement
        let move_speed = player.speed * time.delta_seconds();
        transform.translation.x += movement.x * move_speed;
        transform.translation.y += movement.y * move_speed;
    }
}

// System to update player appearance based on current weapon
pub fn update_player_appearance(
    mut query: Query<(&Player, &mut Sprite), Changed<Player>>,
) {
    for (player, mut sprite) in query.iter_mut() {
        // Change player color based on weapon type for visual feedback
        match player.current_weapon {
            WeaponType::Dagger => {
                sprite.color = Color::YELLOW;
                info!("Player appearance updated to Dagger (Yellow)");
            },
            WeaponType::Sword => {
                sprite.color = Color::rgb(0.0, 0.5, 1.0); // Blue
                info!("Player appearance updated to Sword (Blue)");
            },
            WeaponType::Axe => {
                sprite.color = Color::ORANGE;
                info!("Player appearance updated to Axe (Orange)");
            },
            WeaponType::Hammer => {
                sprite.color = Color::RED;
                info!("Player appearance updated to Hammer (Red)");
            },
        }
    }
}