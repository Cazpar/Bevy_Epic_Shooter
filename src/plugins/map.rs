use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_map);
    }
}

// Map generation constants
const MAP_SIZE_X: usize = 20;
const MAP_SIZE_Y: usize = 20;
const TILE_SIZE: f32 = 32.0;

fn generate_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load the tileset
    let grass_texture = asset_server.load("map/tx_tileset_grass.png");
    
    // Create a parent entity for all map tiles
    let map_entity = commands.spawn(SpatialBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).id();
    
    let mut rng = thread_rng();
    
    // Generate a random map
    for y in 0..MAP_SIZE_Y {
        for x in 0..MAP_SIZE_X {
            // Calculate the position of the tile
            let position = Vec3::new(
                (x as f32 - MAP_SIZE_X as f32 / 2.0) * TILE_SIZE,
                (y as f32 - MAP_SIZE_Y as f32 / 2.0) * TILE_SIZE,
                0.0,
            );
            
            // Randomly choose a color variation for visual interest
            let color = if rng.gen_bool(0.8) {
                // Most tiles are green (grass)
                Color::rgb(
                    0.1 + rng.gen::<f32>() * 0.1,
                    0.6 + rng.gen::<f32>() * 0.2,
                    0.1 + rng.gen::<f32>() * 0.1,
                )
            } else if rng.gen_bool(0.5) {
                // Some tiles are blue (water)
                Color::rgb(
                    0.1 + rng.gen::<f32>() * 0.1,
                    0.3 + rng.gen::<f32>() * 0.2,
                    0.7 + rng.gen::<f32>() * 0.2,
                )
            } else {
                // A few tiles are brown (dirt/path)
                Color::rgb(
                    0.5 + rng.gen::<f32>() * 0.2,
                    0.3 + rng.gen::<f32>() * 0.1,
                    0.1 + rng.gen::<f32>() * 0.1,
                )
            };
            
            // Spawn the tile
            let tile_entity = commands.spawn(SpriteBundle {
                texture: grass_texture.clone(),
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            }).id();
            
            // Add the tile as a child of the map
            commands.entity(map_entity).add_child(tile_entity);
        }
    }
} 