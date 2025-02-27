use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::player::Player;
use crate::components::weapon::WeaponType;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    // Get the window and camera
    let Ok(window) = q_window.get_single() else {
        return; // No window found
    };
    
    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        return; // No camera found
    };
    
    // Get the cursor position in the window
    if let Some(cursor_position) = window.cursor_position() {
        // Convert cursor position to world coordinates
        if let Some(cursor_world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            for (mut transform, mut player) in query.iter_mut() {
                // Calculate direction to mouse
                let player_position = transform.translation.truncate();
                let direction_to_mouse = (cursor_world_position - player_position).normalize_or_zero();
                
                // Update player rotation to face the mouse
                if direction_to_mouse != Vec2::ZERO {
                    player.rotation = direction_to_mouse.y.atan2(direction_to_mouse.x);
                    transform.rotation = Quat::from_rotation_z(player.rotation);
                }
                
                // Handle forward movement with W key
                if keyboard_input.pressed(KeyCode::W) {
                    // Move in the direction the player is facing
                    let forward_direction = transform.rotation.mul_vec3(Vec3::X).truncate();
                    transform.translation.x += forward_direction.x * player.speed * time.delta_seconds();
                    transform.translation.y += forward_direction.y * player.speed * time.delta_seconds();
                }
                
                // Optional: Handle backward movement with S key
                if keyboard_input.pressed(KeyCode::S) {
                    // Move backward from the direction the player is facing
                    let forward_direction = transform.rotation.mul_vec3(Vec3::X).truncate();
                    transform.translation.x -= forward_direction.x * player.speed * time.delta_seconds();
                    transform.translation.y -= forward_direction.y * player.speed * time.delta_seconds();
                }
                
                // Optional: Handle strafing with A and D keys
                if keyboard_input.pressed(KeyCode::A) {
                    // Strafe left (perpendicular to forward direction)
                    let right_direction = transform.rotation.mul_vec3(Vec3::Y).truncate();
                    transform.translation.x -= right_direction.x * player.speed * time.delta_seconds();
                    transform.translation.y -= right_direction.y * player.speed * time.delta_seconds();
                }
                
                if keyboard_input.pressed(KeyCode::D) {
                    // Strafe right (perpendicular to forward direction)
                    let right_direction = transform.rotation.mul_vec3(Vec3::Y).truncate();
                    transform.translation.x += right_direction.x * player.speed * time.delta_seconds();
                    transform.translation.y += right_direction.y * player.speed * time.delta_seconds();
                }
            }
        }
    }
}

// System to update player appearance based on current weapon
pub fn update_player_appearance(
    mut query: Query<(&Player, &mut Sprite), Changed<Player>>,
) {
    for (player, mut sprite) in query.iter_mut() {
        // Change player color based on weapon type for visual feedback
        match player.current_weapon {
            WeaponType::Pistol => {
                sprite.color = Color::YELLOW;
                info!("Player appearance updated to Pistol (Yellow)");
            },
            WeaponType::Shotgun => {
                sprite.color = Color::ORANGE;
                info!("Player appearance updated to Shotgun (Orange)");
            },
            WeaponType::MachineGun => {
                sprite.color = Color::CYAN;
                info!("Player appearance updated to Machine Gun (Cyan)");
            },
            WeaponType::RocketLauncher => {
                sprite.color = Color::RED;
                info!("Player appearance updated to Rocket Launcher (Red)");
            },
        }
    }
}