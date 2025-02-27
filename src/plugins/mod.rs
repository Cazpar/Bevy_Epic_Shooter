pub mod player;
pub mod enemy;
pub mod map;
pub mod weapon;
pub mod pickup;
pub mod ui;
pub mod collision;
pub mod camera;
pub mod game_state;
pub mod animation;


use bevy::prelude::*;
use player::PlayerPlugin;
use enemy::EnemyPlugin;
use map::MapPlugin;
use weapon::WeaponPlugin;
use pickup::PickupPlugin;
use ui::UiPlugin;
use collision::CollisionPlugin;
use camera::CameraPlugin;
use game_state::GameStatePlugin;
use animation::AnimationPlugin;


/// Collection of all game plugins
pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(GameStatePlugin)  // Add the game state plugin first to manage game states
            .add_plugins(MapPlugin)        // Add the map plugin so it renders behind everything else
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(WeaponPlugin)
            .add_plugins(PickupPlugin)
            .add_plugins(UiPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(CollisionPlugin)
            .add_plugins(AnimationPlugin);
    }
}