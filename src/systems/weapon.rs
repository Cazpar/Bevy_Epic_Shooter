use bevy::prelude::*;
use crate::components::player::Player;
use crate::components::weapon::{Weapon, Projectile, WeaponType};
use crate::resources::game_state::GameState;

// Handle player shooting
pub fn player_shooting(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<GameState>,
    mut query: Query<(&Transform, &mut Weapon, &Player)>,
) {
    // Skip if game is paused or over
    if game_state.paused || game_state.game_over {
        return;
    }
    
    let current_time = time.elapsed_seconds();
    
    for (transform, mut weapon, _player) in query.iter_mut() {
        // Check if player is shooting
        if keyboard_input.pressed(KeyCode::Space) && weapon.can_shoot(current_time) {
            // Update last shot time
            weapon.last_shot = current_time;
            
            // Get player position and rotation
            let position = transform.translation;
            let rotation = transform.rotation;
            
            // Calculate projectile direction based on player rotation
            let forward = rotation * Vec3::X;
            
            // Spawn projectile
            spawn_projectile(
                &mut commands,
                position + forward * 20.0, // Offset from player
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