use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::enemy::{Enemy, EnemyType};
use crate::components::player::Player;
use crate::resources::game_state::GameState;
use rand::{thread_rng, Rng};

// Spawn a new enemy at a random position around the screen edges
pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_state: Res<GameState>,
) {
    // Skip spawning if game is paused or over
    if game_state.paused || game_state.game_over {
        return;
    }
    
    let window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();
    
    let mut rng = thread_rng();
    
    // Determine spawn position (outside the screen)
    let spawn_side = rng.gen_range(0..4); // 0: top, 1: right, 2: bottom, 3: left
    let margin = 50.0; // Distance outside the screen to spawn
    
    let (spawn_x, spawn_y) = match spawn_side {
        0 => (rng.gen_range(-window_width/2.0..window_width/2.0), window_height/2.0 + margin), // Top
        1 => (window_width/2.0 + margin, rng.gen_range(-window_height/2.0..window_height/2.0)), // Right
        2 => (rng.gen_range(-window_width/2.0..window_width/2.0), -window_height/2.0 - margin), // Bottom
        _ => (-window_width/2.0 - margin, rng.gen_range(-window_height/2.0..window_height/2.0)), // Left
    };
    
    // Randomly select enemy type
    let enemy_type = match rng.gen_range(0..4) {
        0 => EnemyType::Basic,
        1 => EnemyType::Fast,
        2 => EnemyType::Tank,
        _ => EnemyType::Shooter,
    };
    
    // Select enemy sprite based on type
    let sprite_path = match enemy_type {
        EnemyType::Basic => "sprites/kenney_blocky-characters/Skins/Basic/skin_soldier.png",
        EnemyType::Fast => "sprites/kenney_blocky-characters/Skins/Basic/skin_robot.png",
        EnemyType::Tank => "sprites/kenney_blocky-characters/Skins/Basic/skin_orc.png",
        EnemyType::Shooter => "sprites/kenney_blocky-characters/Skins/Basic/skin_man.png",
    };
    
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(sprite_path),
            transform: Transform::from_translation(Vec3::new(spawn_x, spawn_y, 0.0))
                .with_scale(Vec3::new(0.4, 0.4, 1.0)),
            ..default()
        },
        Enemy::new(enemy_type),
    ));
}

// Move enemies toward the player
pub fn enemy_movement(
    time: Res<Time>,
    game_state: Res<GameState>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy), Without<Player>>,
) {
    // Skip movement if game is paused or over
    if game_state.paused || game_state.game_over {
        return;
    }
    
    // Get player position
    let player_transform = if let Ok(transform) = player_query.get_single() {
        transform
    } else {
        return; // No player found
    };
    
    let player_pos = player_transform.translation.truncate();
    
    // Move each enemy toward the player
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let enemy_pos = transform.translation.truncate();
        let direction = (player_pos - enemy_pos).normalize_or_zero();
        
        // Move enemy toward player
        transform.translation.x += direction.x * enemy.speed * time.delta_seconds();
        transform.translation.y += direction.y * enemy.speed * time.delta_seconds();
        
        // Update enemy rotation to face the player
        if direction != Vec2::ZERO {
            enemy.rotation = direction.y.atan2(direction.x);
            transform.rotation = Quat::from_rotation_z(enemy.rotation);
        }
    }
} 