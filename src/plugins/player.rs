use bevy::prelude::*;
use crate::components::player::Player;
use crate::components::weapon::{Weapon, WeaponType};
use crate::components::pickup::WeaponUpgrades;
use crate::components::animation::{SpriteAnimation, AnimatedSprite, AnimationState};
use crate::systems::player::*;
use crate::systems::animation::load_animation_frames;
use crate::resources::game_state::GameState;

// Component to mark the player entity for easy cleanup
#[derive(Component)]
pub struct PlayerRoot;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Spawn player when entering the Playing state
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            
            // Despawn player when exiting the Playing state
            .add_systems(OnExit(GameState::Playing), despawn_player)
            
            // Add systems to the Update schedule with state conditions
            .add_systems(
                Update, 
                (
                    player_movement,
                    update_player_appearance,
                    update_player_animation,
                ).run_if(in_state(GameState::Playing))
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load the player idle animation frames
    let idle_frames = load_animation_frames(
        &asset_server,
        "sprites/player/Skeleton-Crusaders/Skeleton_Crusader_1/PNG/PNG Sequences/Idle",
        18,
        "0_Skeleton_Crusader_Idle_"
    );
    
    // Create the idle animation
    let idle_animation = SpriteAnimation::new(idle_frames, 12.0, true);
    
    // Spawn the player entity
    commands.spawn((
        SpriteBundle {
            texture: idle_animation.frames[0].clone(),
            sprite: Sprite {
                // Use a larger size for the skeleton sprite
                custom_size: Some(Vec2::new(80.0, 80.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        },
        Player {
            health: 100.0,
            speed: 150.0,
            rotation: 0.0,
            current_weapon: WeaponType::Sword,
        },
        // Add a sword as the default weapon
        Weapon::new(WeaponType::Sword),
        // Add weapon upgrades component
        WeaponUpgrades::new(),
        // Add animation components
        idle_animation,
        AnimationState::default(),
        AnimatedSprite,
        // Mark as player root for easy cleanup
        PlayerRoot,
    ));
    
    info!("Player spawned with default Sword weapon and animations");
}

// System to despawn the player when exiting the Playing state
fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerRoot>>,
) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
        info!("Player despawned");
    }
}

// System to update player animation based on movement and actions
pub fn update_player_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(Entity, &Player, &mut AnimationState), (With<Player>, With<AnimatedSprite>)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (entity, _player, mut animation_state) in player_query.iter_mut() {
        let mut new_animation = "idle".to_string();
        
        // Determine animation based on keyboard input
        if keyboard_input.pressed(KeyCode::W) || 
           keyboard_input.pressed(KeyCode::A) || 
           keyboard_input.pressed(KeyCode::S) || 
           keyboard_input.pressed(KeyCode::D) {
            new_animation = "walking".to_string();
        }
        
        // Check for attack animation
        if keyboard_input.pressed(KeyCode::Space) || 
           keyboard_input.pressed(KeyCode::Return) {
            new_animation = "attacking".to_string();
        }
        
        // Only change animation if it's different from the current one
        if new_animation != animation_state.current_animation {
            animation_state.current_animation = new_animation.clone();
            
            // Load the appropriate animation frames
            let frames = match new_animation.as_str() {
                "walking" => load_animation_frames(
                    &asset_server,
                    "sprites/player/Skeleton-Crusaders/Skeleton_Crusader_1/PNG/PNG Sequences/Walking",
                    18,
                    "0_Skeleton_Crusader_Walking_"
                ),
                "attacking" => load_animation_frames(
                    &asset_server,
                    "sprites/player/Skeleton-Crusaders/Skeleton_Crusader_1/PNG/PNG Sequences/Slashing",
                    12,
                    "0_Skeleton_Crusader_Slashing_"
                ),
                _ => load_animation_frames(
                    &asset_server,
                    "sprites/player/Skeleton-Crusaders/Skeleton_Crusader_1/PNG/PNG Sequences/Idle",
                    18,
                    "0_Skeleton_Crusader_Idle_"
                ),
            };
            
            // Create the new animation
            let new_animation = SpriteAnimation::new(frames, 12.0, true);
            
            // Replace the old animation component
            commands.entity(entity).insert(new_animation);
            
            info!("Player animation changed to: {}", animation_state.current_animation);
        }
    }
}

// Define a system set for systems that run during the Playing state
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OnUpdate(pub GameState);