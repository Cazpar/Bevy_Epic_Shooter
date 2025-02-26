pub mod player;
pub mod enemy;
pub mod weapon;
pub mod collision;
pub mod camera;
pub mod map;
pub mod pickup;

use bevy::prelude::*;
use player::PlayerPlugin;
use enemy::EnemyPlugin;
use weapon::WeaponPlugin;
use collision::CollisionPlugin;
use camera::CameraPlugin;
use map::MapPlugin;
use pickup::PickupPlugin;

/// Collection of all game plugins
pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MapPlugin)     // Add the map plugin first so it renders behind everything else
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(WeaponPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(CollisionPlugin)
            .add_plugins(PickupPlugin);
    }
}