pub mod player;
pub mod enemy;

use bevy::prelude::*;
use player::PlayerPlugin;
use enemy::EnemyPlugin;

/// Collection of all game plugins
pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin);
    }
}