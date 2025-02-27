use bevy::prelude::*;
use crate::resources::game_state::{GameState, GameData};
use crate::components::player::Player;
use crate::components::weapon::{Weapon, WeaponType};
use crate::components::pickup::WeaponUpgrades;

// UI Plugin
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Setup systems
            .add_systems(OnEnter(GameState::MainMenu), (setup_main_menu, spawn_menu_background_elements))
            .add_systems(OnEnter(GameState::Playing), setup_game_ui)
            .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
            .add_systems(OnEnter(GameState::GameOver), setup_game_over_screen)
            .add_systems(OnEnter(GameState::LevelComplete), setup_level_complete_screen)
            
            // Cleanup systems
            .add_systems(OnExit(GameState::MainMenu), (cleanup_ui::<MainMenuUI>, cleanup_menu_background))
            .add_systems(OnExit(GameState::Playing), cleanup_ui::<GameUI>)
            .add_systems(OnExit(GameState::Paused), cleanup_ui::<PauseMenuUI>)
            .add_systems(OnExit(GameState::GameOver), cleanup_ui::<GameOverUI>)
            .add_systems(OnExit(GameState::LevelComplete), cleanup_ui::<LevelCompleteUI>)
            
            // Update systems
            .add_systems(Update, update_health_ui.run_if(in_state(GameState::Playing)))
            .add_systems(Update, update_weapon_ui.run_if(in_state(GameState::Playing)))
            .add_systems(Update, update_score_ui.run_if(in_state(GameState::Playing)))
            .add_systems(Update, (animate_main_menu, animate_menu_background).run_if(in_state(GameState::MainMenu)));
    }
}

// UI Component markers for different screens
#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct GameUI;

#[derive(Component)]
struct PauseMenuUI;

#[derive(Component)]
struct GameOverUI;

#[derive(Component)]
struct LevelCompleteUI;

// Game UI Components
#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct WeaponText;

#[derive(Component)]
struct ScoreText;

// Animation component for the title
#[derive(Component)]
struct TitleAnimation {
    timer: Timer,
    scale_up: bool,
}

// Animation component for the start prompt
#[derive(Component)]
struct StartPromptAnimation {
    timer: Timer,
    visible: bool,
}

impl Default for TitleAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.8, TimerMode::Repeating),
            scale_up: true,
        }
    }
}

impl Default for StartPromptAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            visible: true,
        }
    }
}

// Generic cleanup system for UI elements
fn cleanup_ui<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Setup main menu UI
fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.2).into(), // More transparent background (0.5 -> 0.2)
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title with glow effect
            parent.spawn((
                TextBundle::from_section(
                    "BEVY EPIC SHOOTER",
                    TextStyle {
                        font: font.clone(),
                        font_size: 80.0,
                        color: Color::rgb(0.95, 0.95, 1.0), // Slightly brighter
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
                TitleAnimation::default(),
            ));
            
            // Subtitle
            parent.spawn(
                TextBundle::from_section(
                    "A fast-paced top-down shooter",
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::rgb(0.8, 0.8, 0.9), // Slightly brighter
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                }),
            );
            
            // Start game instruction with animation
            parent.spawn((
                TextBundle::from_section(
                    "Press SPACE or ENTER to start",
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::rgb(0.9, 0.9, 0.9), // Brighter for better visibility
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
                StartPromptAnimation::default(),
            ));
            
            // Controls info
            parent.spawn(
                TextBundle::from_sections([
                    TextSection::new(
                        "Controls:\n",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::rgb(1.0, 1.0, 1.0), // Pure white for better visibility
                        },
                    ),
                    TextSection::new(
                        "WASD - Move\nMouse - Aim\nLeft Click/Space - Shoot\nESC - Pause",
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::rgb(0.85, 0.85, 0.85), // Brighter for better visibility
                        },
                    ),
                ])
                .with_style(Style {
                    margin: UiRect::top(Val::Px(50.0)),
                    ..default()
                }),
            );
        });
}

// Setup game UI
fn setup_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            GameUI,
        ))
        .with_children(|parent| {
            // Health UI
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Health: ",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        "100",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::GREEN,
                        },
                    ),
                ])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..default()
                }),
                HealthText,
            ));
            
            // Weapon UI
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Weapon: ",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        "Pistol",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::YELLOW,
                        },
                    ),
                    TextSection::new(
                        "\nUpgrades: None",
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::SILVER,
                        },
                    ),
                ])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(40.0),
                    left: Val::Px(10.0),
                    ..default()
                }),
                WeaponText,
            ));
            
            // Score UI
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Score: ",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        "0",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::GOLD,
                        },
                    ),
                ])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..default()
                }),
                ScoreText,
            ));
        });
}

// Setup pause menu UI
fn setup_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            PauseMenuUI,
        ))
        .with_children(|parent| {
            // Pause title
            parent.spawn(
                TextBundle::from_section(
                    "PAUSED",
                    TextStyle {
                        font: font.clone(),
                        font_size: 64.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );
            
            // Resume instruction
            parent.spawn(
                TextBundle::from_section(
                    "Press ESC or SPACE to resume",
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );
        });
}

// Setup game over screen UI
fn setup_game_over_screen(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    game_data: Res<GameData>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgba(0.1, 0.0, 0.0, 0.9).into(),
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            // Game over title
            parent.spawn(
                TextBundle::from_section(
                    "GAME OVER",
                    TextStyle {
                        font: font.clone(),
                        font_size: 64.0,
                        color: Color::RED,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );
            
            // Final score
            parent.spawn(
                TextBundle::from_section(
                    format!("Final Score: {}", game_data.score),
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::GOLD,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                }),
            );
            
            // High score
            parent.spawn(
                TextBundle::from_section(
                    format!("High Score: {}", game_data.high_score),
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::GOLD,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                }),
            );
            
            // Restart instruction
            parent.spawn(
                TextBundle::from_section(
                    "Press SPACE or ENTER to restart",
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );
        });
}

// Setup level complete screen UI
fn setup_level_complete_screen(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    game_data: Res<GameData>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.1, 0.0, 0.9).into(),
                ..default()
            },
            LevelCompleteUI,
        ))
        .with_children(|parent| {
            // Level complete title
            parent.spawn(
                TextBundle::from_section(
                    format!("LEVEL {} COMPLETE!", game_data.level),
                    TextStyle {
                        font: font.clone(),
                        font_size: 64.0,
                        color: Color::GREEN,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );
            
            // Current score
            parent.spawn(
                TextBundle::from_section(
                    format!("Score: {}", game_data.score),
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::GOLD,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                }),
            );
            
            // Next level instruction
            parent.spawn(
                TextBundle::from_section(
                    "Press SPACE or ENTER to continue",
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );
        });
}

// Update health UI
fn update_health_ui(
    mut query: Query<&mut Text, With<HealthText>>,
    player_query: Query<&Player>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        if let Ok(player) = player_query.get_single() {
            // Update health text
            text.sections[1].value = format!("{:.0}", player.health);
            
            // Update color based on health
            text.sections[1].style.color = if player.health > 70.0 {
                Color::GREEN
            } else if player.health > 30.0 {
                Color::YELLOW
            } else {
                Color::RED
            };
        }
    }
}

// Update weapon UI
fn update_weapon_ui(
    mut query: Query<&mut Text, With<WeaponText>>,
    player_query: Query<(&Player, &Weapon, Option<&WeaponUpgrades>)>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        if let Ok((player, weapon, upgrades)) = player_query.get_single() {
            // Update weapon text
            text.sections[1].value = format!("{:?}", player.current_weapon);
            
            // Update weapon color based on type
            text.sections[1].style.color = match player.current_weapon {
                WeaponType::Pistol => Color::YELLOW,
                WeaponType::Shotgun => Color::ORANGE,
                WeaponType::MachineGun => Color::CYAN,
                WeaponType::RocketLauncher => Color::RED,
            };
            
            // Update upgrades text
            if let Some(upgrades) = upgrades {
                let mut upgrade_text = String::new();
                
                if upgrades.double_shot {
                    upgrade_text.push_str("Double Shot, ");
                }
                
                if upgrades.triple_shot {
                    upgrade_text.push_str("Triple Shot, ");
                }
                
                if upgrades.rapid_fire_multiplier > 1.0 {
                    upgrade_text.push_str(&format!("Rapid Fire x{:.1}, ", upgrades.rapid_fire_multiplier));
                }
                
                if upgrades.damage_multiplier > 1.0 {
                    upgrade_text.push_str(&format!("Damage x{:.1}, ", upgrades.damage_multiplier));
                }
                
                if upgrade_text.is_empty() {
                    upgrade_text = "None".to_string();
                } else {
                    // Remove trailing comma and space
                    upgrade_text.truncate(upgrade_text.len() - 2);
                }
                
                text.sections[2].value = format!("\nUpgrades: {}", upgrade_text);
            } else {
                text.sections[2].value = "\nUpgrades: None".to_string();
            }
        }
    }
}

// Update score UI
fn update_score_ui(
    mut query: Query<&mut Text, With<ScoreText>>,
    game_data: Res<GameData>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[1].value = game_data.score.to_string();
    }
}

// Add this system to the UiPlugin's build method
fn animate_main_menu(
    time: Res<Time>,
    mut title_query: Query<(&mut TitleAnimation, &mut Text, &mut Style)>,
    mut prompt_query: Query<(&mut StartPromptAnimation, &mut Text), Without<TitleAnimation>>,
) {
    // Animate title
    for (mut animation, mut text, mut style) in title_query.iter_mut() {
        animation.timer.tick(time.delta());
        
        if animation.timer.just_finished() {
            // Pulse effect for title - much more subtle
            let scale = if animation.scale_up {
                1.02
            } else {
                0.98
            };
            
            for section in text.sections.iter_mut() {
                section.style.font_size = section.style.font_size * scale;
            }
            
            animation.scale_up = !animation.scale_up;
        }
    }
    
    // Animate start prompt (blinking)
    for (mut animation, mut text) in prompt_query.iter_mut() {
        animation.timer.tick(time.delta());
        
        if animation.timer.just_finished() {
            animation.visible = !animation.visible;
            
            for section in text.sections.iter_mut() {
                section.style.color = if animation.visible {
                    Color::rgb(0.8, 0.8, 0.8)
                } else {
                    Color::rgba(0.8, 0.8, 0.8, 0.0)
                };
            }
        }
    }
}

// Component to mark background elements
#[derive(Component)]
struct MenuBackground {
    speed: f32,
    rotation_speed: f32,
}

// Spawn decorative elements in the background
fn spawn_menu_background_elements(
    mut commands: Commands,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let mut rng = rand::thread_rng();
    
    // Calculate window dimensions with extra padding to ensure elements spawn beyond visible area
    let width_range = window.width() * 2.0; // Increased from 1.5 to 2.0 for wider coverage
    let height_range = window.height() * 2.0; // Increased from 1.5 to 2.0 for taller coverage
    let center_x_offset = 100.0; // Offset to match the camera's initial position
    let center_y_offset = 50.0;
    
    // Add a full-screen background to ensure no black bars are visible
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.3, 0.1, 1.0), // Dark olive green background
                custom_size: Some(Vec2::new(width_range * 1.5, height_range * 1.5)), // Extra large to ensure coverage
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(center_x_offset, center_y_offset, -0.1)), // Slightly behind other elements
            ..default()
        },
        MenuBackground {
            speed: 0.0, // Static background
            rotation_speed: 0.0,
        },
        MenuBackgroundMovement {
            direction: Vec2::new(0.0, 0.0),
        },
    ));
    
    // Spawn more decorative elements for a richer background
    for i in 0..80 { // Increased from 50 to 80 for more elements
        // Distribute elements across a wider area, centered around the camera's initial position
        // Use a more uniform distribution to ensure coverage of the sides
        let x = rand::random::<f32>() * width_range - width_range / 2.0 + center_x_offset;
        let y = rand::random::<f32>() * height_range - height_range / 2.0 + center_y_offset;
        
        // Determine element type with more variety
        let (size, color, speed, rotation_speed) = match i % 5 {
            0 => { // Enemy-like (red)
                let size = rand::random::<f32>() * 15.0 + 20.0;
                (
                    Vec2::new(size, size),
                    Color::rgba(0.8, 0.2, 0.2, 0.5),
                    rand::random::<f32>() * 20.0 + 10.0,
                    rand::random::<f32>() * 1.0 - 0.5,
                )
            },
            1 => { // Pickup-like (green)
                let size = rand::random::<f32>() * 8.0 + 12.0;
                (
                    Vec2::new(size, size),
                    Color::rgba(0.2, 0.8, 0.2, 0.5),
                    rand::random::<f32>() * 15.0 + 5.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                )
            },
            2 => { // Projectile-like (yellow)
                let width = rand::random::<f32>() * 8.0 + 10.0;
                let height = width * 0.4;
                (
                    Vec2::new(width, height),
                    Color::rgba(1.0, 1.0, 0.0, 0.5),
                    rand::random::<f32>() * 30.0 + 20.0,
                    0.0,
                )
            },
            3 => { // Blue element (power-up like)
                let size = rand::random::<f32>() * 10.0 + 15.0;
                (
                    Vec2::new(size, size),
                    Color::rgba(0.2, 0.2, 0.9, 0.5),
                    rand::random::<f32>() * 18.0 + 8.0,
                    rand::random::<f32>() * 3.0 - 1.5,
                )
            },
            _ => { // Purple element (special)
                let width = rand::random::<f32>() * 12.0 + 15.0;
                let height = width * 0.8;
                (
                    Vec2::new(width, height),
                    Color::rgba(0.8, 0.2, 0.8, 0.5),
                    rand::random::<f32>() * 25.0 + 15.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                )
            },
        };
        
        // Random direction with bias toward horizontal movement for more interesting patterns
        let direction = Vec2::new(
            rand::random::<f32>() * 2.0 - 1.0,
            (rand::random::<f32>() * 1.6 - 0.8) * 0.8, // Reduced vertical component
        ).normalize();
        
        // Spawn the element
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 0.0))
                    .with_rotation(Quat::from_rotation_z(direction.y.atan2(direction.x))),
                ..default()
            },
            MenuBackground {
                speed,
                rotation_speed,
            },
            MenuBackgroundMovement {
                direction,
            },
        ));
    }
    
    // Add some decorative elements specifically on the sides to ensure coverage
    for i in 0..20 {
        // Alternate between left and right sides
        let side_factor = if i % 2 == 0 { -1.0 } else { 1.0 };
        let x = side_factor * (half_width(window) * 0.8 + rand::random::<f32>() * 100.0) + center_x_offset;
        let y = rand::random::<f32>() * height_range - height_range / 2.0 + center_y_offset;
        
        let size = rand::random::<f32>() * 20.0 + 15.0;
        let color = match i % 3 {
            0 => Color::rgba(0.8, 0.2, 0.2, 0.5), // Red
            1 => Color::rgba(0.2, 0.8, 0.2, 0.5), // Green
            _ => Color::rgba(0.2, 0.2, 0.9, 0.5), // Blue
        };
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(size, size)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            },
            MenuBackground {
                speed: rand::random::<f32>() * 15.0 + 5.0,
                rotation_speed: rand::random::<f32>() * 1.0 - 0.5,
            },
            MenuBackgroundMovement {
                direction: Vec2::new(-side_factor, rand::random::<f32>() * 0.4 - 0.2).normalize(),
            },
        ));
    }
}

// Helper function to get half width
fn half_width(window: &Window) -> f32 {
    window.width() / 2.0
}

// Helper function to get half height
fn half_height(window: &Window) -> f32 {
    window.height() / 2.0
}

// Component to track movement direction
#[derive(Component)]
struct MenuBackgroundMovement {
    direction: Vec2,
}

// Animate background elements
fn animate_menu_background(
    time: Res<Time>,
    mut commands: Commands,
    window_query: Query<&Window>,
    mut query: Query<(Entity, &mut Transform, &MenuBackground, &mut MenuBackgroundMovement)>,
) {
    let window = window_query.single();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;
    
    for (entity, mut transform, bg, mut movement) in query.iter_mut() {
        // Move in the current direction
        let delta = movement.direction * bg.speed * time.delta_seconds();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
        
        // Rotate if needed
        if bg.rotation_speed != 0.0 {
            transform.rotate_z(bg.rotation_speed * time.delta_seconds());
        }
        
        // Check if out of bounds
        let pos = transform.translation;
        let out_of_bounds = pos.x < -half_width - 50.0 || pos.x > half_width + 50.0 || 
                           pos.y < -half_height - 50.0 || pos.y > half_height + 50.0;
        
        if out_of_bounds {
            // Respawn on the opposite side with a slight randomization
            let (new_x, new_y) = if pos.x < -half_width - 50.0 {
                (half_width + 20.0, rand::random::<f32>() * window.height() - half_height)
            } else if pos.x > half_width + 50.0 {
                (-half_width - 20.0, rand::random::<f32>() * window.height() - half_height)
            } else if pos.y < -half_height - 50.0 {
                (rand::random::<f32>() * window.width() - half_width, half_height + 20.0)
            } else {
                (rand::random::<f32>() * window.width() - half_width, -half_height - 20.0)
            };
            
            transform.translation.x = new_x;
            transform.translation.y = new_y;
            
            // Randomize direction slightly
            let mut new_dir = movement.direction;
            new_dir.x += rand::random::<f32>() * 0.4 - 0.2;
            new_dir.y += rand::random::<f32>() * 0.4 - 0.2;
            movement.direction = new_dir.normalize();
            
            // Update rotation to match direction
            transform.rotation = Quat::from_rotation_z(movement.direction.y.atan2(movement.direction.x));
        }
    }
}

// Cleanup background elements
fn cleanup_menu_background(
    mut commands: Commands,
    query: Query<Entity, With<MenuBackground>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
} 