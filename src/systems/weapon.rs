use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use crate::components::player::Player;
use crate::components::weapon::{Weapon, Projectile, WeaponType};
use crate::components::enemy::Enemy;
use crate::components::obstacle::{Obstacle, ObstacleType};
use crate::components::debug::CollisionDebug;
use crate::components::pickup::WeaponUpgrades;

// Handle player shooting
pub fn player_shooting(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(&Transform, &mut Weapon, &Player, Option<&WeaponUpgrades>)>,
) {
    // Process shooting for each player with a weapon
    for (transform, mut weapon, _player, weapon_upgrades) in query.iter_mut() {
        // Check if player is trying to shoot
        let is_shooting = keyboard_input.pressed(KeyCode::Space) || mouse_input.pressed(MouseButton::Left);
        
        // Get current time
        let current_time = time.elapsed_seconds();
        
        // If cooldown is over and player is shooting
        if weapon.can_shoot(current_time) && is_shooting {
            // Create default upgrades if none exist
            let default_upgrades = WeaponUpgrades::new();
            
            // Get weapon upgrades if available
            let upgrades = weapon_upgrades.unwrap_or(&default_upgrades);
            
            // Calculate actual fire rate with upgrades
            let fire_rate_multiplier = upgrades.rapid_fire_multiplier;
            
            // Update last shot time
            weapon.last_shot = current_time;
            
            // Calculate actual damage with upgrades
            let damage_multiplier = upgrades.damage_multiplier;
            let damage = weapon.damage * damage_multiplier;
            
            // Get player position and forward direction
            let position = transform.translation;
            let forward = transform.rotation.mul_vec3(Vec3::X).truncate();
            
            // Determine number of projectiles based on upgrades
            if upgrades.triple_shot {
                // Spawn three projectiles in a spread pattern
                spawn_projectile(&mut commands, position, forward.rotate(Vec2::from_angle(-0.2)), &weapon, damage);
                spawn_projectile(&mut commands, position, forward, &weapon, damage);
                spawn_projectile(&mut commands, position, forward.rotate(Vec2::from_angle(0.2)), &weapon, damage);
            } else if upgrades.double_shot {
                // Spawn two projectiles side by side
                spawn_projectile(&mut commands, position, forward.rotate(Vec2::from_angle(-0.1)), &weapon, damage);
                spawn_projectile(&mut commands, position, forward.rotate(Vec2::from_angle(0.1)), &weapon, damage);
            } else {
                // Spawn a single projectile
                spawn_projectile(&mut commands, position, forward, &weapon, damage);
            }
            
            // Special case for shotgun: spawn additional projectiles
            if weapon.weapon_type == WeaponType::Shotgun {
                for i in 1..=3 {
                    let angle = 0.15 * i as f32;
                    spawn_projectile(&mut commands, position, forward.rotate(Vec2::from_angle(angle)), &weapon, damage * 0.7);
                    spawn_projectile(&mut commands, position, forward.rotate(Vec2::from_angle(-angle)), &weapon, damage * 0.7);
                }
            }
        }
    }
}

// Move projectiles
pub fn projectile_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Projectile)>,
) {
    for (entity, mut transform, mut projectile) in query.iter_mut() {
        // Update projectile lifetime
        projectile.lifetime -= time.delta_seconds();
        
        // Despawn if lifetime is over
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        
        // Calculate movement based on projectile's current rotation
        let direction = transform.rotation.mul_vec3(Vec3::X).truncate();
        let movement = direction * projectile.speed * time.delta_seconds();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}

// Helper function to spawn a projectile
fn spawn_projectile(
    commands: &mut Commands,
    position: Vec3,
    direction: Vec2,
    weapon: &Weapon,
    damage: f32,
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
            damage,
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
    _time: Res<Time>,
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
                
                // If enemy health <= 0, despawn enemy and all its children
                if enemy.health <= 0.0 {
                    info!("Enemy defeated!");
                    // Despawn the enemy after we've processed everything for this frame
                    commands.entity(enemy_entity).despawn_recursive();
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
                    
                    // Add debug component to obstacle for visual feedback - only if it still exists
                    commands.entity(obstacle_entity).insert(CollisionDebug::default());
                    
                    // Print debug info
                    info!("Obstacle hit! Type: {:?}, Health: {}/{}, Damage: {}", 
                          obstacle.obstacle_type, obstacle.health, obstacle.max_health, projectile.damage);
                    
                    // If obstacle is destroyed, despawn it and all its children
                    if is_destroyed {
                        info!("Obstacle destroyed!");
                        commands.entity(obstacle_entity).despawn_recursive();
                    }
                }
                
                // Despawn projectile if it should be despawned
                if should_despawn_projectile {
                    commands.entity(projectile_entity).despawn_recursive();
                    break; // No need to check other obstacles for this projectile
                }
            }
        }
    }
} 