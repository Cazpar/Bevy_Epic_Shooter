use bevy::prelude::*;
use crate::components::player::Player;
use crate::components::weapon::{Weapon, WeaponType};
use crate::components::pickup::WeaponUpgrades;
use crate::systems::player::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, update_player_appearance)
            .add_systems(Update, update_weapon_indicator);
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

// Component to mark the weapon indicator
#[derive(Component)]
struct WeaponIndicator;

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