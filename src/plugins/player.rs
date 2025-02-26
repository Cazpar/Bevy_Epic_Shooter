use bevy::prelude::*;
use crate::components::player::Player;
use crate::systems::player::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

fn spawn_player(mut commands: Commands) {
    // Spawn the player entity
    let player_entity = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.7, 0.9), // Blue color for player
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Player {
            health: 100.0,
            speed: 150.0,
            rotation: 0.0,
        },
    )).id();
    
    // Add a direction indicator as a child entity
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(15.0, 5.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(20.0, 0.0, 0.1)),
        ..default()
    }).set_parent(player_entity);
}