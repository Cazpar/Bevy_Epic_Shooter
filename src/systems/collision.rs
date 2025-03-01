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
    // Create a list of collision debug entities to add after all collision checks
    let mut collision_debug_entities = Vec::new();
    
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
                
                // Store entities for collision debug instead of immediately inserting components
                collision_debug_entities.push(player_entity);
                collision_debug_entities.push(obstacle_entity);
            }
        }
    }
    
    // Add collision debug components after all collision checks
    for entity in collision_debug_entities {
        // Only add if the entity still exists
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.insert(CollisionDebug::default());
        }
    }
}

pub fn handle_enemy_obstacle_collision(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Transform, &Enemy)>,
    obstacle_query: Query<(Entity, &Transform, &Obstacle), Without<Enemy>>,
    time: Res<Time>,
) {
    // Create a list of collision debug entities to add after all collision checks
    let mut collision_debug_entities = Vec::new();
    
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
                
                // Only add entities that still exist to our list
                if commands.get_entity(enemy_entity).is_some() {
                    collision_debug_entities.push(enemy_entity);
                }
                
                if commands.get_entity(obstacle_entity).is_some() {
                    collision_debug_entities.push(obstacle_entity);
                }
            }
        }
    }
    
    // Add collision debug components after all collision checks
    for entity in collision_debug_entities {
        // Double-check that the entity still exists
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.insert(CollisionDebug::default());
        }
    }
}

// Optional: Add a system to handle projectile collisions with obstacles
// This system is no longer needed with the melee combat system, but keeping it commented for reference
/*
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
                
                // Add debug component to obstacle for visual feedback - only if it still exists
                if let Some(mut entity_commands) = commands.get_entity(obstacle_entity) {
                    entity_commands.insert(CollisionDebug::default());
                }
                
                // Print debug info
                info!("Projectile collision with obstacle: distance={}, min_distance={}", distance, min_distance);
                
                commands.entity(projectile_entity).despawn();
                break; // No need to check other obstacles for this projectile
            }
        }
    }
}
*/

// System to update collision debug visuals
pub fn update_collision_debug(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut CollisionDebug, Option<&Children>)>,
    mut child_sprites: Query<&mut Sprite, Without<CollisionDebug>>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut debug, children) in query.iter_mut() {
        // Update the timer
        debug.timer.tick(time.delta());
        
        // Make the sprite flash red
        sprite.color = Color::RED;
        
        // Also make any child sprites flash red
        if let Some(children) = children {
            for child in children.iter() {
                // Check if the child entity still exists before accessing it
                if child_sprites.contains(*child) {
                    if let Ok(mut child_sprite) = child_sprites.get_mut(*child) {
                        child_sprite.color = Color::RED;
                    }
                }
            }
        }
        
        // If the timer is finished, remove the debug component
        if debug.timer.finished() {
            // Use a safe approach to remove the component
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.remove::<CollisionDebug>();
                sprite.color = Color::WHITE; // Reset color
                
                // Also reset any child sprites
                if let Some(children) = children {
                    for child in children.iter() {
                        // Check if the child entity still exists before accessing it
                        if child_sprites.contains(*child) {
                            if let Ok(mut child_sprite) = child_sprites.get_mut(*child) {
                                child_sprite.color = Color::WHITE;
                            }
                        }
                    }
                }
            }
        }
    }
} 