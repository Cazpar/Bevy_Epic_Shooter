pub mod player;

use bevy::prelude::*;
use player::PlayerPlugin;

/// Collection of all game plugins
pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin);
    }
}