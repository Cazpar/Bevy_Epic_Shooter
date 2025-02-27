use bevy::prelude::*;
use crate::systems::animation::animate_sprites;
use crate::resources::game_state::GameState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animate_sprites.run_if(in_state(GameState::Playing))
        );
    }
} 