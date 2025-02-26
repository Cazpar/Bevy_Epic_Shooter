pub mod player;
pub mod enemy;
pub mod weapon;

use bevy::prelude::*;
use player::PlayerPlugin;
use enemy::EnemyPlugin;
use weapon::WeaponPlugin;

/// Collection of all game plugins
pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(WeaponPlugin);
    }
}