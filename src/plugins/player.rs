use bevy::prelude::*;
use crate::components::player::Player;
use crate::components::weapon::{Weapon, WeaponType};
use crate::systems::player::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load the player sprite
    let texture_handle = asset_server.load("sprites/common.png");
    
    // Spawn the player entity
    commands.spawn((
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
        },
        // Add a pistol as the default weapon
        Weapon::new(WeaponType::Pistol),
    ));
}