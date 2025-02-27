use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use crate::components::player::Player;
use crate::components::weapon::{Weapon, MeleeAttack, WeaponType, Attacking};
use crate::components::enemy::Enemy;
use crate::components::obstacle::{Obstacle, ObstacleType};
use crate::components::debug::CollisionDebug;
use crate::components::pickup::WeaponUpgrades;

// Handle player melee attacks
pub fn player_attacking(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(Entity, &Transform, &mut Weapon, &Player, Option<&WeaponUpgrades>)>,
    attacking_query: Query<&Attacking>,
) {
    // Don't allow new attacks if player is already attacking
    if !attacking_query.is_empty() {
        return;
    }
    
    // Process attacking for each player with a weapon
    for (entity, transform, mut weapon, _player, weapon_upgrades) in query.iter_mut() {
        // Check if player is trying to attack
        let is_attacking = keyboard_input.pressed(KeyCode::Space) || 
                           keyboard_input.pressed(KeyCode::Return) || 
                           mouse_input.pressed(MouseButton::Left);
        
        // Get current time
        let current_time = time.elapsed_seconds();
        
        // If cooldown is over and player is attacking
        if weapon.can_attack(current_time) && is_attacking {
            // Create default upgrades if none exist
            let default_upgrades = WeaponUpgrades::new();
            
            // Get weapon upgrades if available
            let upgrades = weapon_upgrades.unwrap_or(&default_upgrades);
            
            // Apply attack speed multiplier
            let attack_speed_multiplier = upgrades.attack_speed_multiplier;
            let actual_attack_speed = weapon.attack_speed * attack_speed_multiplier;
            
            // Update last attack time
            weapon.last_attack = current_time;
            
            // Calculate actual damage with upgrades
            let damage_multiplier = upgrades.damage_multiplier;
            let base_damage = weapon.damage * damage_multiplier;
            
            // Apply critical hit chance
            let mut damage = base_damage;
            if rand::random::<f32>() < upgrades.critical_hit_chance {
                damage *= 2.0; // Critical hit does double damage
                info!("Critical hit! Damage: {}", damage);
            }
            
            // Get attack duration based on weapon type
            let attack_duration = match weapon.weapon_type {
                WeaponType::Dagger => 0.3,
                WeaponType::Sword => 0.5,
                WeaponType::Axe => 0.7,
                WeaponType::Hammer => 1.0,
            };
            
            // Add attacking component to player
            commands.entity(entity).insert(Attacking::new(attack_duration));
            
            // Get player position and forward direction
            let position = transform.translation;
            let forward = transform.rotation.mul_vec3(Vec3::X).truncate();
            
            // Determine attack width based on weapon type and upgrades
            let base_width = match weapon.weapon_type {
                WeaponType::Dagger => 0.5, // Narrow attack
                WeaponType::Sword => 1.0,  // Medium attack
                WeaponType::Axe => 1.2,    // Wide attack
                WeaponType::Hammer => 1.5, // Very wide attack
            };
            
            // Apply area attack upgrade if available
            let width = if upgrades.area_attack {
                base_width * 1.5 // 50% wider attack
            } else {
                base_width
            };
            
            // Spawn melee attack hitbox
            spawn_melee_attack(&mut commands, entity, position, forward, &weapon, damage, width);
            
            info!("Player performed a {:?} attack!", weapon.weapon_type);
        }
    }
}

// Update attacking state
pub fn update_attacking_state(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Attacking)>,
) {
    for (entity, mut attacking) in query.iter_mut() {
        attacking.timer.tick(time.delta());
        
        if attacking.timer.finished() {
            commands.entity(entity).remove::<Attacking>();
        }
    }
}

// Handle melee attack collisions
pub fn handle_melee_attack_collisions(
    mut commands: Commands,
    time: Res<Time>,
    mut melee_query: Query<(Entity, &Transform, &MeleeAttack, &Parent)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    mut obstacle_query: Query<(Entity, &Transform, &mut Obstacle)>,
) {
    for (attack_entity, attack_transform, attack, parent) in melee_query.iter_mut() {
        let attack_pos = attack_transform.translation.truncate();
        let attack_range = match attack.weapon_type {
            WeaponType::Dagger => 40.0,
            WeaponType::Sword => 60.0,
            WeaponType::Axe => 50.0,
            WeaponType::Hammer => 45.0,
        };
        
        // Check for enemy collisions
        for (enemy_entity, enemy_transform, mut enemy) in enemy_query.iter_mut() {
            let enemy_pos = enemy_transform.translation.truncate();
            let enemy_radius = 16.0; // Assuming enemy sprite is roughly 32x32
            
            // Calculate the distance between attack and enemy
            let distance = attack_pos.distance(enemy_pos);
            
            // If collision detected
            if distance < attack_range + enemy_radius {
                // Apply damage to enemy
                enemy.health -= attack.damage;
                
                // Add debug component to enemy for visual feedback
                commands.entity(enemy_entity).insert(CollisionDebug::default());
                
                // Print debug info
                info!("Enemy hit with melee attack! Health: {}, Damage: {}", enemy.health, attack.damage);
                
                // If enemy health <= 0, despawn enemy
                if enemy.health <= 0.0 {
                    info!("Enemy defeated!");
                    commands.entity(enemy_entity).despawn_recursive();
                }
            }
        }
        
        // Check for obstacle collisions
        for (obstacle_entity, obstacle_transform, mut obstacle) in obstacle_query.iter_mut() {
            let obstacle_pos = obstacle_transform.translation.truncate();
            let obstacle_size = obstacle.size;
            
            // Calculate the distance between attack and obstacle
            let distance = attack_pos.distance(obstacle_pos);
            let min_distance = attack_range + obstacle_size.x / 2.0;
            
            // If collision detected
            if distance < min_distance {
                // Apply damage to obstacle if it's destructible
                if obstacle.is_destructible {
                    let is_destroyed = obstacle.take_damage(attack.damage);
                    
                    // Add debug component to obstacle for visual feedback
                    commands.entity(obstacle_entity).insert(CollisionDebug::default());
                    
                    // Print debug info
                    info!("Obstacle hit with melee attack! Type: {:?}, Health: {}/{}, Damage: {}", 
                          obstacle.obstacle_type, obstacle.health, obstacle.max_health, attack.damage);
                    
                    // If obstacle is destroyed, despawn it
                    if is_destroyed {
                        info!("Obstacle destroyed!");
                        commands.entity(obstacle_entity).despawn_recursive();
                    }
                }
            }
        }
        
        // Update attack lifetime
        let mut should_despawn = false;
        
        // Despawn attack after a short duration
        if time.elapsed_seconds() - attack.lifetime > 0.2 {
            should_despawn = true;
        }
        
        if should_despawn {
            commands.entity(attack_entity).despawn_recursive();
        }
    }
}

// Helper function to spawn a melee attack hitbox
fn spawn_melee_attack(
    commands: &mut Commands,
    parent_entity: Entity,
    position: Vec3,
    direction: Vec2,
    weapon: &Weapon,
    damage: f32,
    width: f32,
) {
    // Calculate rotation from direction
    let angle = direction.y.atan2(direction.x);
    let rotation = Quat::from_rotation_z(angle);
    
    // Determine attack color and size based on weapon type
    let (color, size) = match weapon.weapon_type {
        WeaponType::Dagger => (Color::rgba(1.0, 1.0, 0.0, 0.5), Vec2::new(40.0, 20.0)), // Yellow
        WeaponType::Sword => (Color::rgba(0.0, 0.5, 1.0, 0.5), Vec2::new(60.0, 30.0)), // Blue
        WeaponType::Axe => (Color::rgba(1.0, 0.5, 0.0, 0.5), Vec2::new(50.0, 40.0)), // Orange
        WeaponType::Hammer => (Color::rgba(1.0, 0.0, 0.0, 0.5), Vec2::new(45.0, 45.0)), // Red
    };
    
    // Calculate offset position in front of the player
    let offset_distance = weapon.attack_range / 2.0;
    let offset_position = Vec3::new(
        position.x + direction.x * offset_distance,
        position.y + direction.y * offset_distance,
        position.z + 0.1, // Slightly in front of player
    );
    
    // Spawn the attack hitbox
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(offset_position)
                .with_rotation(rotation),
            ..default()
        },
        MeleeAttack {
            damage,
            weapon_type: weapon.weapon_type,
            lifetime: 0.0, // Will be updated in the collision system
            angle,
            width,
        },
    )).set_parent(parent_entity);
} 