use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::enemy::{Enemy, EnemyType};
use crate::components::player::Player;
use crate::components::animation::{SpriteAnimation, AnimatedSprite, AnimationState};
use crate::systems::animation::load_animation_frames;
use rand::{thread_rng, Rng};

// Spawn a new enemy at a random position around the screen edges
pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
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
    
    // Randomly select enemy variant (1, 2, or 3)
    let enemy_variant = rng.gen_range(1..=3);
    
    // Select enemy folder based on type
    let enemy_folder = match enemy_type {
        EnemyType::Basic => "Skeleton-Warriors",
        EnemyType::Fast => "Fallen-Angels",
        EnemyType::Tank => "Golems",
        EnemyType::Shooter => "Zombie-Villagers",
    };
    
    // Select character model based on enemy type
    let character_model = match enemy_type {
        EnemyType::Basic => format!("Skeleton_Warrior_{}", enemy_variant),
        EnemyType::Fast => format!("Fallen_Angels_{}", enemy_variant),
        EnemyType::Tank => format!("Golem_{}", enemy_variant),
        EnemyType::Shooter => format!("Zombie_Villager_{}", enemy_variant),
    };
    
    // Select animation prefix based on enemy type
    let animation_prefix = match enemy_type {
        EnemyType::Basic => "0_Skeleton_Warrior_",
        EnemyType::Fast => "0_Fallen_Angels_",
        EnemyType::Tank => "0_Golem_",
        EnemyType::Shooter => "0_Zombie_Villager_",
    };
    
    // Load idle animation frames for the enemy
    let idle_frames = load_animation_frames(
        &asset_server,
        &format!("sprites/enemies/{}/{}/PNG/PNG Sequences/Idle", enemy_folder, character_model),
        18,
        &format!("{}{}", animation_prefix, "Idle_")
    );
    
    // Create the idle animation
    let idle_animation = SpriteAnimation::new(idle_frames, 12.0, true);
    
    // Spawn the enemy entity with explicit z-index
    let enemy_entity = commands.spawn((
        SpriteBundle {
            texture: idle_animation.frames[0].clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(80.0, 80.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(spawn_x, spawn_y, 1.0)), // Set z to 1.0 for body
            visibility: Visibility::Visible, // Explicitly set visibility
            ..default()
        },
        Enemy::new(enemy_type),
        idle_animation,
        AnimationState::default(),
        AnimatedSprite,
    )).id();
    
    // Add a direction indicator as a child entity with higher z-index
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.9, 0.9, 0.9), // Light gray
                custom_size: Some(Vec2::new(15.0, 5.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(30.0, 0.0, 1.1)), // Set z to 1.1 for gun
            visibility: Visibility::Visible, // Explicitly set visibility
            ..default()
        },
    )).set_parent(enemy_entity);
}

// Move enemies toward the player
pub fn enemy_movement(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy), Without<Player>>,
) {
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

// Update enemy animation based on state
pub fn update_enemy_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_query: Query<(Entity, &Enemy, &mut AnimationState, &Transform), (With<Enemy>, With<AnimatedSprite>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    // Get player position
    let player_transform = if let Ok(transform) = player_query.get_single() {
        transform
    } else {
        return; // No player found
    };
    
    let player_pos = player_transform.translation.truncate();
    
    for (entity, enemy, mut animation_state, enemy_transform) in enemy_query.iter_mut() {
        // Determine enemy variant and folder
        let enemy_variant = 1; // Default to variant 1
        
        // Select enemy folder based on type
        let enemy_folder = match enemy.enemy_type {
            EnemyType::Basic => "Skeleton-Warriors",
            EnemyType::Fast => "Fallen-Angels",
            EnemyType::Tank => "Golems",
            EnemyType::Shooter => "Zombie-Villagers",
        };
        
        // Select character model based on enemy type
        let character_model = match enemy.enemy_type {
            EnemyType::Basic => format!("Skeleton_Warrior_{}", enemy_variant),
            EnemyType::Fast => format!("Fallen_Angels_{}", enemy_variant),
            EnemyType::Tank => format!("Golem_{}", enemy_variant),
            EnemyType::Shooter => format!("Zombie_Villager_{}", enemy_variant),
        };
        
        // Select animation prefix based on enemy type
        let animation_prefix = match enemy.enemy_type {
            EnemyType::Basic => "0_Skeleton_Warrior_",
            EnemyType::Fast => "0_Fallen_Angels_",
            EnemyType::Tank => "0_Golem_",
            EnemyType::Shooter => "0_Zombie_Villager_",
        };
        
        // Determine animation based on distance to player
        let enemy_pos = enemy_transform.translation.truncate();
        let distance_to_player = (player_pos - enemy_pos).length();
        
        let new_animation = if distance_to_player < 200.0 {
            "walking".to_string()
        } else {
            "idle".to_string()
        };
        
        // Only change animation if it's different from the current one
        if new_animation != animation_state.current_animation {
            animation_state.current_animation = new_animation.clone();
            
            // Load the appropriate animation frames
            let frames = match new_animation.as_str() {
                "walking" => load_animation_frames(
                    &asset_server,
                    &format!("sprites/enemies/{}/{}/PNG/PNG Sequences/Walking", enemy_folder, character_model),
                    18,
                    &format!("{}{}", animation_prefix, "Walking_")
                ),
                _ => load_animation_frames(
                    &asset_server,
                    &format!("sprites/enemies/{}/{}/PNG/PNG Sequences/Idle", enemy_folder, character_model),
                    18,
                    &format!("{}{}", animation_prefix, "Idle_")
                ),
            };
            
            // Create the new animation
            let new_animation = SpriteAnimation::new(frames, 12.0, true);
            
            // Replace the old animation component
            commands.entity(entity).insert(new_animation);
        }
    }
} 