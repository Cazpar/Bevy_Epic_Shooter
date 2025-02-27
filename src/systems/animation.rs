use bevy::prelude::*;
use crate::components::animation::{SpriteAnimation, AnimatedSprite};

// System to update sprite animations
pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut SpriteAnimation, &mut Handle<Image>), With<AnimatedSprite>>,
) {
    for (mut animation, mut texture) in query.iter_mut() {
        if animation.tick(time.delta()) {
            if let Some(new_texture) = animation.current_texture() {
                *texture = new_texture;
            }
        }
    }
}

// Helper function to load animation frames from a directory
pub fn load_animation_frames(
    asset_server: &AssetServer,
    base_path: &str,
    frame_count: usize,
    frame_name_pattern: &str,
) -> Vec<Handle<Image>> {
    let mut frames = Vec::with_capacity(frame_count);
    
    for i in 0..frame_count {
        let frame_path = format!("{}/{}{:03}.png", base_path, frame_name_pattern, i);
        frames.push(asset_server.load(&frame_path));
    }
    
    frames
}

// Helper function to load animation frames with a different naming pattern
pub fn load_animation_frames_custom(
    asset_server: &AssetServer,
    base_path: &str,
    frame_name_pattern: &str,
    start_index: usize,
    end_index: usize,
) -> Vec<Handle<Image>> {
    let mut frames = Vec::with_capacity(end_index - start_index + 1);
    
    for i in start_index..=end_index {
        let frame_path = format!("{}/{}{:03}.png", base_path, frame_name_pattern, i);
        frames.push(asset_server.load(&frame_path));
    }
    
    frames
} 