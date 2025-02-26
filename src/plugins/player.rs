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

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("sprites/kenney_blocky-characters/Skins/Basic/skin_adventurer.png");
    
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .with_scale(Vec3::new(0.5, 0.5, 1.0)),
            ..default()
        },
        Player {
            health: 100.0,
            speed: 150.0,
            rotation: 0.0,
        },
    ));
}