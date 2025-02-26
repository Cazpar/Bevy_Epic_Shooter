use bevy::prelude::*;
use crate::components::player::Player;

pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
) {
    // Get the player's position
    if let Ok(player_transform) = player_query.get_single() {
        // Get the camera
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Smoothly move the camera to follow the player
            let target_position = Vec3::new(
                player_transform.translation.x,
                player_transform.translation.y,
                camera_transform.translation.z, // Keep the camera's z position
            );
            
            // Smooth follow with lerp
            camera_transform.translation = camera_transform.translation.lerp(
                target_position,
                time.delta_seconds() * 5.0, // Adjust the 5.0 value to change follow speed
            );
        }
    }
} 