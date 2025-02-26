use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use crate::components::player::Player;
use crate::components::weapon::{Weapon, Projectile, WeaponType};
use crate::components::enemy::Enemy;
use crate::components::obstacle::{Obstacle, ObstacleType};
use crate::components::debug::CollisionDebug;
use crate::resources::game_state::GameState;

// Handle player shooting
pub fn player_shooting(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    game_state: Res<GameState>,
    mut query: Query<(&Transform, &mut Weapon, &Player)>,
) {
    // Skip if game is paused or over
    if game_state.paused || game_state.game_over {
        return;
    }
    
    let current_time = time.elapsed_seconds();
    
    for (transform, mut weapon, _player) in query.iter_mut() {
        // Check if player is shooting (either spacebar or left mouse button)
        if (keyboard_input.pressed(KeyCode::Space) || mouse_input.pressed(MouseButton::Left)) && weapon.can_shoot(current_time) {
            // Update last shot time
            weapon.last_shot = current_time;
            
            // Get player position and rotation
            let position = transform.translation;
            let rotation = transform.rotation;
            
            // Calculate projectile direction based on player rotation
            let forward = rotation * Vec3::X;
            
            // Calculate gun position based on the player sprite
            // Based on the screenshot, the gun appears to be on the right side of the player
            // These values are adjusted to match the position of the gun in the sprite
            let gun_offset = rotation * Vec3::new(15.0, 0.0, 0.0); // Offset from player center to gun position
            
            // Spawn projectile from the gun position
            spawn_projectile(
                &mut commands,
                position + gun_offset, // Position at the gun
                forward.truncate(),
                weapon.as_ref(),
            );
        }
    }
}

// Move projectiles
pub fn projectile_movement(
    mut commands: Commands,
    time: Res<Time>,
    game_state: Res<GameState>,
    mut query: Query<(Entity, &mut Transform, &mut Projectile)>,
) {
    // Skip if game is paused or over
    if game_state.paused || game_state.game_over {
        return;
    }
    
    for (entity, mut transform, mut projectile) in query.iter_mut() {
        // Update projectile lifetime
        projectile.lifetime -= time.delta_seconds();
        
        // Despawn if lifetime is over
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        
        // Move projectile forward
        transform.translation.x += transform.rotation.mul_vec3(Vec3::X).x * projectile.speed * time.delta_seconds();
        transform.translation.y += transform.rotation.mul_vec3(Vec3::X).y * projectile.speed * time.delta_seconds();
    }
}

// Helper function to spawn a projectile
fn spawn_projectile(
    commands: &mut Commands,
    position: Vec3,
    direction: Vec2,
    weapon: &Weapon,
) {
    // Calculate rotation from direction
    let rotation = Quat::from_rotation_z(direction.y.atan2(direction.x));
    
    // Determine projectile color and size based on weapon type
    let (color, size) = match weapon.weapon_type {
        WeaponType::Pistol => (Color::rgb(1.0, 1.0, 0.0), Vec2::new(10.0, 4.0)), // Yellow
        WeaponType::Shotgun => (Color::rgb(1.0, 0.5, 0.0), Vec2::new(8.0, 4.0)), // Orange
        WeaponType::MachineGun => (Color::rgb(0.0, 1.0, 1.0), Vec2::new(6.0, 3.0)), // Cyan
        WeaponType::RocketLauncher => (Color::rgb(1.0, 0.0, 0.0), Vec2::new(15.0, 6.0)), // Red
    };
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(position)
                .with_rotation(rotation),
            ..default()
        },
        Projectile {
            damage: weapon.damage,
            speed: weapon.projectile_speed,
            lifetime: 2.0, // 2 seconds lifetime
            weapon_type: weapon.weapon_type,
        },
    ));
}

// Handle projectile damage to enemies
pub fn handle_projectile_enemy_collision(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    projectile_query: Query<(Entity, &Transform, &Projectile)>,
    time: Res<Time>,
) {
    for (projectile_entity, projectile_transform, projectile) in projectile_query.iter() {
        let projectile_pos = projectile_transform.translation.truncate();
        let projectile_radius = match projectile.weapon_type {
            WeaponType::Pistol => 5.0,
            WeaponType::Shotgun => 4.0,
            WeaponType::MachineGun => 3.0,
            WeaponType::RocketLauncher => 7.5,
        };
        
        for (enemy_entity, enemy_transform, mut enemy) in enemy_query.iter_mut() {
            let enemy_pos = enemy_transform.translation.truncate();
            let enemy_radius = 16.0; // Assuming enemy sprite is roughly 32x32
            
            // Calculate the distance between projectile and enemy
            let distance = projectile_pos.distance(enemy_pos);
            let min_distance = projectile_radius + enemy_radius;
            
            // If collision detected
            if distance < min_distance {
                // Apply damage to enemy
                enemy.health -= projectile.damage;
                
                // Add debug component to enemy for visual feedback
                commands.entity(enemy_entity).insert(CollisionDebug::default());
                
                // Print debug info
                info!("Enemy hit! Health: {}, Damage: {}", enemy.health, projectile.damage);
                
                // Despawn projectile
                commands.entity(projectile_entity).despawn();
                
                // If enemy health <= 0, despawn enemy
                if enemy.health <= 0.0 {
                    info!("Enemy defeated!");
                    commands.entity(enemy_entity).despawn();
                }
                
                break; // No need to check other enemies for this projectile
            }
        }
    }
}

// Handle projectile damage to obstacles
pub fn handle_projectile_obstacle_damage(
    mut commands: Commands,
    mut obstacle_query: Query<(Entity, &Transform, &mut Obstacle)>,
    projectile_query: Query<(Entity, &Transform, &Projectile)>,
) {
    for (projectile_entity, projectile_transform, projectile) in projectile_query.iter() {
        let projectile_pos = projectile_transform.translation.truncate();
        let projectile_radius = match projectile.weapon_type {
            WeaponType::Pistol => 5.0,
            WeaponType::Shotgun => 4.0,
            WeaponType::MachineGun => 3.0,
            WeaponType::RocketLauncher => 7.5,
        };
        
        for (obstacle_entity, obstacle_transform, mut obstacle) in obstacle_query.iter_mut() {
            let obstacle_pos = obstacle_transform.translation.truncate();
            let obstacle_size = obstacle.size;
            
            // Calculate the distance between projectile and obstacle
            let distance = projectile_pos.distance(obstacle_pos);
            let min_distance = projectile_radius + obstacle_size.x / 2.0;
            
            // If collision detected
            if distance < min_distance {
                // For bushes, allow projectiles to pass through but still take damage
                let should_despawn_projectile = !matches!(obstacle.obstacle_type, ObstacleType::Bush);
                
                // Apply damage to obstacle if it's destructible
                if obstacle.is_destructible {
                    let is_destroyed = obstacle.take_damage(projectile.damage);
                    
                    // Add debug component to obstacle for visual feedback
                    commands.entity(obstacle_entity).insert(CollisionDebug::default());
                    
                    // Print debug info
                    info!("Obstacle hit! Type: {:?}, Health: {}/{}, Damage: {}", 
                          obstacle.obstacle_type, obstacle.health, obstacle.max_health, projectile.damage);
                    
                    // If obstacle is destroyed, despawn it
                    if is_destroyed {
                        info!("Obstacle destroyed!");
                        commands.entity(obstacle_entity).despawn();
                    }
                }
                
                // Despawn projectile if it should be despawned
                if should_despawn_projectile {
                    commands.entity(projectile_entity).despawn();
                    break; // No need to check other obstacles for this projectile
                }
            }
        }
    }
} 