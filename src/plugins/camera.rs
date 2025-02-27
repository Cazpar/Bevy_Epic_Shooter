use bevy::prelude::*;
use crate::systems::camera::*;
use crate::resources::game_state::GameState;

// Component to track main menu camera animation
#[derive(Component)]
struct MainMenuCamera {
    timer: Timer,
    direction: Vec2,
    center_position: Vec2,
}

impl Default for MainMenuCamera {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(8.0, TimerMode::Repeating),
            direction: Vec2::new(1.0, 0.5).normalize(),
            center_position: Vec2::new(100.0, 50.0), // Offset from center to make it more interesting
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu_camera)
            .add_systems(Update, animate_main_menu_camera.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), reset_camera)
            .add_systems(Update, camera_follow_player.run_if(in_state(GameState::Playing)));
    }
}

// Setup the camera for the main menu
fn setup_main_menu_camera(
    mut camera_query: Query<(Entity, &mut Transform), With<Camera>>,
    mut commands: Commands,
) {
    if let Ok((camera_entity, mut transform)) = camera_query.get_single_mut() {
        // Position the camera to show an interesting part of the background
        let main_menu_camera = MainMenuCamera::default();
        transform.translation.x = main_menu_camera.center_position.x;
        transform.translation.y = main_menu_camera.center_position.y;
        
        // Add the MainMenuCamera component to the camera entity
        commands.entity(camera_entity).insert(main_menu_camera);
    }
}

// Animate the camera in the main menu
fn animate_main_menu_camera(
    time: Res<Time>,
    mut query: Query<(&mut MainMenuCamera, &mut Transform), With<Camera>>,
) {
    if let Ok((mut main_menu_camera, mut transform)) = query.get_single_mut() {
        main_menu_camera.timer.tick(time.delta());
        
        // Calculate a slow, gentle movement
        let max_offset = 50.0;
        let t = main_menu_camera.timer.percent();
        let angle = t * std::f32::consts::TAU; // Full circle over the timer duration
        
        // Create a smooth, circular-ish movement
        let offset_x = angle.cos() * max_offset;
        let offset_y = angle.sin() * max_offset * 0.6; // Elliptical path
        
        // Update camera position
        transform.translation.x = main_menu_camera.center_position.x + offset_x;
        transform.translation.y = main_menu_camera.center_position.y + offset_y;
    }
}

// Reset the camera position when exiting the main menu
fn reset_camera(
    mut camera_query: Query<(Entity, &mut Transform), With<Camera>>,
    mut commands: Commands,
) {
    if let Ok((camera_entity, mut transform)) = camera_query.get_single_mut() {
        // Reset camera position to center
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        
        // Remove the MainMenuCamera component
        commands.entity(camera_entity).remove::<MainMenuCamera>();
    }
} 