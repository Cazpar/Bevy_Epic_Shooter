use bevy::prelude::*;
use rand::Rng;
use crate::components::pickup::{Pickup, PickupType, WeaponUpgrades};
use crate::components::player::Player;
use crate::components::enemy::Enemy;
use crate::components::weapon::{Weapon, WeaponType};
use crate::resources::game_state::GameState;

// Chance for an enemy to drop a pickup when defeated
const DROP_CHANCE: f32 = 0.9; // 90% chance for testing (was 40%)

// Spawn a pickup when an enemy is defeated
pub fn spawn_enemy_drops(
    mut commands: Commands,
    query: Query<&Transform, (With<Enemy>, Changed<Enemy>)>,
    mut enemy_removal_events: RemovedComponents<Enemy>,
) {
    // Get positions of enemies that were just removed
    let mut positions = Vec::new();
    for entity in enemy_removal_events.read() {
        if let Ok(transform) = query.get(entity) {
            positions.push(transform.translation);
        }
    }
    
    // Spawn pickups at those positions
    for position in positions {
        let mut rng = rand::thread_rng();
        
        // Only spawn a pickup with a certain chance
        if rng.gen::<f32>() <= DROP_CHANCE {
            // Determine what type of pickup to spawn
            let pickup_type = match rng.gen_range(0..=100) {
                0..=20 => PickupType::Weapon(WeaponType::Pistol),
                21..=35 => PickupType::Weapon(WeaponType::Shotgun),
                36..=50 => PickupType::Weapon(WeaponType::MachineGun),
                51..=60 => PickupType::Weapon(WeaponType::RocketLauncher),
                61..=70 => PickupType::DoubleShot,
                71..=80 => PickupType::TripleShot,
                81..=90 => PickupType::RapidFire,
                91..=95 => PickupType::IncreasedDamage,
                _ => PickupType::HealthPack,
            };
            
            spawn_pickup(&mut commands, position, pickup_type);
        }
    }
}

// Helper function to spawn a pickup
fn spawn_pickup(commands: &mut Commands, position: Vec3, pickup_type: PickupType) {
    // Determine color and size based on pickup type
    let (color, size) = match pickup_type {
        PickupType::Weapon(WeaponType::Pistol) => (Color::YELLOW, Vec2::new(20.0, 10.0)),
        PickupType::Weapon(WeaponType::Shotgun) => (Color::ORANGE, Vec2::new(20.0, 10.0)),
        PickupType::Weapon(WeaponType::MachineGun) => (Color::CYAN, Vec2::new(20.0, 10.0)),
        PickupType::Weapon(WeaponType::RocketLauncher) => (Color::RED, Vec2::new(20.0, 10.0)),
        PickupType::DoubleShot => (Color::GREEN, Vec2::new(15.0, 15.0)),
        PickupType::TripleShot => (Color::rgb(0.0, 0.8, 0.0), Vec2::new(15.0, 15.0)),
        PickupType::RapidFire => (Color::PURPLE, Vec2::new(15.0, 15.0)),
        PickupType::IncreasedDamage => (Color::rgb(1.0, 0.0, 0.5), Vec2::new(15.0, 15.0)),
        PickupType::HealthPack => (Color::rgb(1.0, 0.0, 0.3), Vec2::new(15.0, 15.0)),
    };
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        Pickup::new(pickup_type),
    ));
    
    info!("Spawned pickup: {:?} at {:?}", pickup_type, position);
}

// Update pickups (rotation, lifetime)
pub fn update_pickups(
    mut commands: Commands,
    time: Res<Time>,
    game_state: Res<GameState>,
    mut query: Query<(Entity, &mut Transform, &mut Pickup)>,
) {
    // Skip if game is paused or over
    if game_state.paused || game_state.game_over {
        return;
    }
    
    for (entity, mut transform, mut pickup) in query.iter_mut() {
        // Update lifetime
        pickup.lifetime -= time.delta_seconds();
        
        // Despawn if lifetime is over
        if pickup.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        
        // Rotate the pickup for visual effect
        transform.rotate_z(pickup.rotation_speed * time.delta_seconds());
    }
}

// Handle player collecting pickups
pub fn handle_pickup_collection(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &mut Player, &mut Weapon, Option<&mut WeaponUpgrades>)>,
    pickup_query: Query<(Entity, &Transform, &Pickup)>,
) {
    for (player_entity, player_transform, mut player, mut weapon, weapon_upgrades_option) in player_query.iter_mut() {
        let player_pos = player_transform.translation.truncate();
        let player_radius = 16.0; // Assuming player sprite is roughly 32x32
        
        // Ensure player has WeaponUpgrades component
        let mut weapon_upgrades = if let Some(upgrades) = weapon_upgrades_option {
            upgrades
        } else {
            // Add WeaponUpgrades component if it doesn't exist
            commands.entity(player_entity).insert(WeaponUpgrades::new());
            info!("Added WeaponUpgrades component to player");
            continue; // Skip this frame, we'll have the component next frame
        };
        
        for (pickup_entity, pickup_transform, pickup) in pickup_query.iter() {
            let pickup_pos = pickup_transform.translation.truncate();
            let pickup_radius = 10.0; // Assuming pickup is roughly 20x20
            
            // Calculate the distance between player and pickup
            let distance = player_pos.distance(pickup_pos);
            let min_distance = player_radius + pickup_radius;
            
            // Debug distance to pickups
            info!("Player distance to pickup: {}, min_distance: {}, pickup_type: {:?}", 
                  distance, min_distance, pickup.pickup_type);
            
            // If player collects the pickup
            if distance < min_distance {
                // Add visual feedback - flash the player
                commands.entity(player_entity).insert(crate::components::debug::CollisionDebug::default());
                
                // Apply the pickup effect
                match pickup.pickup_type {
                    PickupType::Weapon(weapon_type) => {
                        // Change player's weapon
                        info!("WEAPON CHANGE: from {:?} to {:?}", player.current_weapon, weapon_type);
                        player.current_weapon = weapon_type;
                        *weapon = Weapon::new(weapon_type);
                        info!("Player picked up weapon: {:?}", weapon_type);
                    },
                    PickupType::DoubleShot => {
                        weapon_upgrades.double_shot = true;
                        weapon_upgrades.triple_shot = false; // Double shot overrides triple shot
                        info!("Player picked up double shot upgrade!");
                    },
                    PickupType::TripleShot => {
                        weapon_upgrades.triple_shot = true;
                        weapon_upgrades.double_shot = false; // Triple shot overrides double shot
                        info!("Player picked up triple shot upgrade!");
                    },
                    PickupType::RapidFire => {
                        // Increase fire rate by 30% (stacks up to a limit)
                        weapon_upgrades.rapid_fire_multiplier = (weapon_upgrades.rapid_fire_multiplier + 0.3).min(3.0);
                        info!("Player picked up rapid fire upgrade! New multiplier: {}", weapon_upgrades.rapid_fire_multiplier);
                    },
                    PickupType::IncreasedDamage => {
                        // Increase damage by 20% (stacks up to a limit)
                        weapon_upgrades.damage_multiplier = (weapon_upgrades.damage_multiplier + 0.2).min(3.0);
                        info!("Player picked up damage upgrade! New multiplier: {}", weapon_upgrades.damage_multiplier);
                    },
                    PickupType::HealthPack => {
                        // Increase player health by 25 (up to a max of 100)
                        player.health = (player.health + 25.0).min(100.0);
                        info!("Player picked up health pack! New health: {}", player.health);
                    },
                }
                
                // Despawn the pickup
                commands.entity(pickup_entity).despawn();
            }
        }
    }
} 