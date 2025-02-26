use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use crate::components::player::Player;
use crate::components::obstacle::{Obstacle, ObstacleType};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MapState::default())
            .add_systems(Startup, setup_map)
            .add_systems(Update, check_for_new_chunks);
    }
}

// Map generation constants
const CHUNK_SIZE: usize = 10; // Size of each chunk in tiles
const TILE_SIZE: f32 = 32.0;  // Size of each tile in pixels
const LOAD_DISTANCE: f32 = 4.0; // Distance in chunks to load from player
const OBSTACLE_DENSITY: f32 = 0.1; // Probability of an obstacle per tile (0.0-1.0)

// Resource to track map state
#[derive(Resource, Default)]
struct MapState {
    loaded_chunks: HashMap<(i32, i32), Entity>,
}

// Component to mark a chunk entity
#[derive(Component)]
struct MapChunk {
    chunk_x: i32,
    chunk_y: i32,
}

// Setup the initial map
fn setup_map(
    mut commands: Commands,
    mut map_state: ResMut<MapState>,
    asset_server: Res<AssetServer>,
) {
    // Load the tileset
    let tileset_handle = asset_server.load("map/tx_tileset_grass.png");
    
    // Load obstacle textures
    let rock_texture = asset_server.load("map/obstacles/rock.png");
    let crate_texture = asset_server.load("map/obstacles/crate.png");
    let bush_texture = asset_server.load("map/obstacles/bush.png");
    
    // Generate the initial chunks around (0,0)
    for y in -1..=1 {
        for x in -1..=1 {
            generate_chunk(
                &mut commands, 
                &mut map_state, 
                tileset_handle.clone(), 
                rock_texture.clone(),
                crate_texture.clone(),
                bush_texture.clone(),
                x, y
            );
        }
    }
}

// Check if new chunks need to be generated
fn check_for_new_chunks(
    mut commands: Commands,
    mut map_state: ResMut<MapState>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        // Calculate which chunk the player is in
        let player_chunk_x = (player_transform.translation.x / (CHUNK_SIZE as f32 * TILE_SIZE)).floor() as i32;
        let player_chunk_y = (player_transform.translation.y / (CHUNK_SIZE as f32 * TILE_SIZE)).floor() as i32;
        
        // Load chunks in a square around the player
        let load_radius = LOAD_DISTANCE as i32;
        
        // Check each potential chunk position
        for y in (player_chunk_y - load_radius)..=(player_chunk_y + load_radius) {
            for x in (player_chunk_x - load_radius)..=(player_chunk_x + load_radius) {
                // If this chunk isn't loaded yet, generate it
                if !map_state.loaded_chunks.contains_key(&(x, y)) {
                    let tileset_handle = asset_server.load("map/tx_tileset_grass.png");
                    let rock_texture = asset_server.load("map/obstacles/rock.png");
                    let crate_texture = asset_server.load("map/obstacles/crate.png");
                    let bush_texture = asset_server.load("map/obstacles/bush.png");
                    
                    generate_chunk(
                        &mut commands, 
                        &mut map_state, 
                        tileset_handle, 
                        rock_texture,
                        crate_texture,
                        bush_texture,
                        x, y
                    );
                }
            }
        }
        
        // Optional: Unload chunks that are too far away
        // This would improve performance for very large maps
        let unload_radius = load_radius + 2; // Unload chunks that are further than this
        let mut chunks_to_remove = Vec::new();
        
        for (&(chunk_x, chunk_y), &chunk_entity) in map_state.loaded_chunks.iter() {
            if (chunk_x - player_chunk_x).abs() > unload_radius || 
               (chunk_y - player_chunk_y).abs() > unload_radius {
                chunks_to_remove.push(((chunk_x, chunk_y), chunk_entity));
            }
        }
        
        // Remove the far away chunks
        for ((chunk_x, chunk_y), chunk_entity) in chunks_to_remove {
            commands.entity(chunk_entity).despawn_recursive();
            map_state.loaded_chunks.remove(&(chunk_x, chunk_y));
        }
    }
}

// Generate a single chunk of the map
fn generate_chunk(
    commands: &mut Commands,
    map_state: &mut MapState,
    tileset_handle: Handle<Image>,
    rock_texture: Handle<Image>,
    crate_texture: Handle<Image>,
    bush_texture: Handle<Image>,
    chunk_x: i32,
    chunk_y: i32,
) {
    // Create a parent entity for this chunk
    let chunk_entity = commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        MapChunk {
            chunk_x,
            chunk_y,
        },
    )).id();
    
    let mut rng = thread_rng();
    
    // Calculate the world position of the chunk's corner
    let chunk_world_x = chunk_x as f32 * CHUNK_SIZE as f32 * TILE_SIZE;
    let chunk_world_y = chunk_y as f32 * CHUNK_SIZE as f32 * TILE_SIZE;
    
    // Track positions where obstacles are placed to avoid overlaps
    let mut obstacle_positions = Vec::new();
    
    // Generate tiles for this chunk
    for local_y in 0..CHUNK_SIZE {
        for local_x in 0..CHUNK_SIZE {
            // Calculate the world position of this tile
            let tile_world_x = chunk_world_x + local_x as f32 * TILE_SIZE;
            let tile_world_y = chunk_world_y + local_y as f32 * TILE_SIZE;
            let tile_position = Vec2::new(tile_world_x, tile_world_y);
            
            // Determine tile type based on position and randomness
            let (tile_x, tile_y) = determine_tile_coords(chunk_x, chunk_y, local_x, local_y, &mut rng);
            
            // Spawn the tile
            let tile_entity = commands.spawn(SpriteBundle {
                texture: tileset_handle.clone(),
                sprite: Sprite {
                    rect: Some(Rect {
                        min: Vec2::new(tile_x, tile_y),
                        max: Vec2::new(tile_x + 32.0, tile_y + 32.0),
                    }),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(tile_world_x, tile_world_y, 0.0)),
                ..default()
            }).id();
            
            // Add the tile as a child of the chunk
            commands.entity(chunk_entity).add_child(tile_entity);
            
            // Randomly decide if we should place an obstacle on this tile
            // Don't place obstacles on edges or if the tile is not grass
            let is_edge = local_x == 0 || local_y == 0 || local_x == CHUNK_SIZE - 1 || local_y == CHUNK_SIZE - 1;
            let is_grass = tile_y < 1.0; // Assuming grass tiles are in the first row
            
            if !is_edge && is_grass && rng.gen::<f32>() < OBSTACLE_DENSITY {
                // Choose a random obstacle type
                let obstacle_type = match rng.gen_range(0..3) {
                    0 => ObstacleType::Rock,
                    1 => ObstacleType::Crate,
                    _ => ObstacleType::Bush,
                };
                
                // Get the appropriate texture
                let obstacle_texture = match obstacle_type {
                    ObstacleType::Rock => rock_texture.clone(),
                    ObstacleType::Crate => crate_texture.clone(),
                    ObstacleType::Bush => bush_texture.clone(),
                };
                
                // Add some randomness to the obstacle position within the tile
                let offset_x = rng.gen_range(-5.0..5.0);
                let offset_y = rng.gen_range(-5.0..5.0);
                
                // Spawn the obstacle
                let obstacle_entity = commands.spawn((
                    SpriteBundle {
                        texture: obstacle_texture,
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(32.0, 32.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            tile_world_x + offset_x, 
                            tile_world_y + offset_y, 
                            1.0 // Slightly above the tiles
                        )),
                        ..default()
                    },
                    Obstacle::new(obstacle_type),
                )).id();
                
                // Add the obstacle as a child of the chunk
                commands.entity(chunk_entity).add_child(obstacle_entity);
                
                // Record the obstacle position
                obstacle_positions.push(tile_position);
            }
        }
    }
    
    // Store the chunk in our map state
    map_state.loaded_chunks.insert((chunk_x, chunk_y), chunk_entity);
}

// Determine which tile from the tileset to use
fn determine_tile_coords(chunk_x: i32, chunk_y: i32, local_x: usize, local_y: usize, rng: &mut impl Rng) -> (f32, f32) {
    // Use the tileset information from tx_tileset_grass.tsx
    // The tileset is 8x8 tiles, each 32x32 pixels
    
    // Create some variety based on position and randomness
    let is_edge = local_x == 0 || local_y == 0 || local_x == CHUNK_SIZE - 1 || local_y == CHUNK_SIZE - 1;
    let is_water = rng.gen_ratio(1, 10) && !is_edge; // 10% chance for water, but not on edges
    let is_path = (chunk_x + chunk_y) % 2 == 0 && (local_x + local_y) % 5 < 2 && !is_water; // Some paths
    
    if is_water {
        // Water tiles (row 1, columns 0-1)
        let col = rng.gen_range(0..2);
        (col as f32 * 32.0, 1.0 * 32.0)
    } else if is_path {
        // Path tiles (row 2, columns 0-1)
        let col = rng.gen_range(0..2);
        (col as f32 * 32.0, 2.0 * 32.0)
    } else if is_edge {
        // Edge/obstacle tiles (row 3, columns 0-1)
        let col = rng.gen_range(0..2);
        (col as f32 * 32.0, 3.0 * 32.0)
    } else {
        // Grass tiles (row 0, columns 0-3)
        let col = rng.gen_range(0..4);
        (col as f32 * 32.0, 0.0)
    }
} 