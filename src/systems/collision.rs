use bevy::prelude::*;
use crate::components::player::Player;
use crate::components::enemy::Enemy;
use crate::components::obstacle::Obstacle;
use crate::components::debug::CollisionDebug;

// Constants for collision handling
const COLLISION_BUFFER: f32 = 10.0; // Increased buffer distance to prevent entities from getting too close to obstacles

pub fn handle_player_obstacle_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform, &Player)>,
    obstacle_query: Query<(Entity, &Transform, &Obstacle), Without<Player>>,
    time: Res<Time>,
) {
    for (player_entity, mut player_transform, _player) in player_query.iter_mut() {
        let player_pos = player_transform.translation.truncate();
        let player_radius = 16.0; // Assuming player sprite is roughly 32x32
        
        for (obstacle_entity, obstacle_transform, obstacle) in obstacle_query.iter() {
            let obstacle_pos = obstacle_transform.translation.truncate();
            let obstacle_size = obstacle.size;
            
            // Calculate the distance between player and obstacle
            let distance = player_pos.distance(obstacle_pos);
            let min_distance = player_radius + obstacle_size.x / 2.0 + COLLISION_BUFFER;
            
            // If collision detected
            if distance < min_distance {
                // Calculate direction from obstacle to player
                let direction = (player_pos - obstacle_pos).normalize_or_zero();
                
                // Push player away from obstacle with stronger force
                let push_strength = (min_distance - distance) * 15.0 * time.delta_seconds();
                player_transform.translation.x += direction.x * push_strength;
                player_transform.translation.y += direction.y * push_strength;
                
                // Add debug component to player for visual feedback
                commands.entity(player_entity).insert(CollisionDebug::default());
                
                // Add debug component to obstacle for visual feedback
                commands.entity(obstacle_entity).insert(CollisionDebug::default());
                
                // Print debug info
                info!("Player collision with obstacle: distance={}, min_distance={}", distance, min_distance);
            }
        }
    }
}

pub fn handle_enemy_obstacle_collision(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Transform, &Enemy)>,
    obstacle_query: Query<(Entity, &Transform, &Obstacle), Without<Enemy>>,
    time: Res<Time>,
) {
    for (enemy_entity, mut enemy_transform, _enemy) in enemy_query.iter_mut() {
        let enemy_pos = enemy_transform.translation.truncate();
        let enemy_radius = 16.0; // Assuming enemy sprite is roughly 32x32
        
        for (obstacle_entity, obstacle_transform, obstacle) in obstacle_query.iter() {
            let obstacle_pos = obstacle_transform.translation.truncate();
            let obstacle_size = obstacle.size;
            
            // Calculate the distance between enemy and obstacle
            let distance = enemy_pos.distance(obstacle_pos);
            let min_distance = enemy_radius + obstacle_size.x / 2.0 + COLLISION_BUFFER;
            
            // If collision detected
            if distance < min_distance {
                // Calculate direction from obstacle to enemy
                let direction = (enemy_pos - obstacle_pos).normalize_or_zero();
                
                // Push enemy away from obstacle with stronger force
                let push_strength = (min_distance - distance) * 15.0 * time.delta_seconds();
                enemy_transform.translation.x += direction.x * push_strength;
                enemy_transform.translation.y += direction.y * push_strength;
                
                // Add debug component to enemy for visual feedback
                commands.entity(enemy_entity).insert(CollisionDebug::default());
                
                // Add debug component to obstacle for visual feedback
                commands.entity(obstacle_entity).insert(CollisionDebug::default());
                
                // Print debug info
                info!("Enemy collision with obstacle: distance={}, min_distance={}", distance, min_distance);
            }
        }
    }
}

// Optional: Add a system to handle projectile collisions with obstacles
pub fn handle_projectile_obstacle_collision(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<crate::components::weapon::Projectile>>,
    obstacle_query: Query<(Entity, &Transform, &Obstacle)>,
) {
    for (projectile_entity, projectile_transform) in projectile_query.iter() {
        let projectile_pos = projectile_transform.translation.truncate();
        let projectile_radius = 4.0; // Assuming projectile is small
        
        for (obstacle_entity, obstacle_transform, obstacle) in obstacle_query.iter() {
            let obstacle_pos = obstacle_transform.translation.truncate();
            let obstacle_size = obstacle.size;
            
            // Calculate the distance between projectile and obstacle
            let distance = projectile_pos.distance(obstacle_pos);
            let min_distance = projectile_radius + obstacle_size.x / 2.0;
            
            // If collision detected, despawn the projectile
            if distance < min_distance {
                // For bushes, allow projectiles to pass through
                if matches!(obstacle.obstacle_type, crate::components::obstacle::ObstacleType::Bush) {
                    continue;
                }
                
                // Add debug component to obstacle for visual feedback
                commands.entity(obstacle_entity).insert(CollisionDebug::default());
                
                // Print debug info
                info!("Projectile collision with obstacle: distance={}, min_distance={}", distance, min_distance);
                
                commands.entity(projectile_entity).despawn();
                break; // No need to check other obstacles for this projectile
            }
        }
    }
}

// System to update collision debug visuals
pub fn update_collision_debug(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut CollisionDebug)>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut debug) in query.iter_mut() {
        // Update the timer
        debug.timer.tick(time.delta());
        
        // Make the sprite flash red
        sprite.color = Color::RED;
        
        // If the timer is finished, remove the debug component
        if debug.timer.finished() {
            commands.entity(entity).remove::<CollisionDebug>();
            sprite.color = Color::WHITE; // Reset color
        }
    }
} 