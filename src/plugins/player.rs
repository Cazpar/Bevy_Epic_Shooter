use bevy::prelude::*;
use crate::components::player::Player;
use crate::components::weapon::{Weapon, WeaponType};
use crate::components::pickup::WeaponUpgrades;
use crate::systems::player::*;
use crate::resources::game_state::GameState;

// Component to mark the weapon indicator
#[derive(Component)]
pub struct WeaponIndicator;

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
                    update_weapon_indicator
                ).run_if(in_state(GameState::Playing))
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load the player sprite
    let texture_handle = asset_server.load("sprites/common.png");
    
    // Spawn the player entity
    let player_entity = commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            sprite: Sprite {
                // Use the entire image as the sprite
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        },
        Player {
            health: 100.0,
            speed: 150.0,
            rotation: 0.0,
            current_weapon: WeaponType::Pistol,
        },
        // Add a pistol as the default weapon
        Weapon::new(WeaponType::Pistol),
        // Add weapon upgrades component
        WeaponUpgrades::new(),
        // Mark as player root for easy cleanup
        PlayerRoot,
    )).id();
    
    // Add a weapon indicator as a child entity
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW, // Pistol color
                custom_size: Some(Vec2::new(15.0, 15.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(30.0, 30.0, 11.0)), // Position relative to player
            ..default()
        },
        WeaponIndicator,
    )).set_parent(player_entity);
    
    info!("Player spawned with default Pistol weapon");
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

// System to update the weapon indicator
pub fn update_weapon_indicator(
    player_query: Query<(&Player, &Children), Changed<Player>>,
    mut indicator_query: Query<&mut Sprite, With<WeaponIndicator>>,
) {
    for (player, children) in player_query.iter() {
        for &child in children.iter() {
            if let Ok(mut sprite) = indicator_query.get_mut(child) {
                // Update indicator color based on weapon type
                match player.current_weapon {
                    WeaponType::Pistol => sprite.color = Color::YELLOW,
                    WeaponType::Shotgun => sprite.color = Color::ORANGE,
                    WeaponType::MachineGun => sprite.color = Color::CYAN,
                    WeaponType::RocketLauncher => sprite.color = Color::RED,
                }
                info!("Weapon indicator updated to: {:?}", player.current_weapon);
            }
        }
    }
}

// Define a system set for systems that run during the Playing state
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OnUpdate(pub GameState);