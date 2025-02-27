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
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnEnter(GameState::Playing), setup_game_ui)
            .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
            .add_systems(OnEnter(GameState::GameOver), setup_game_over_screen)
            .add_systems(OnEnter(GameState::LevelComplete), setup_level_complete_screen)
            
            // Cleanup systems
            .add_systems(OnExit(GameState::MainMenu), cleanup_ui::<MainMenuUI>)
            .add_systems(OnExit(GameState::Playing), cleanup_ui::<GameUI>)
            .add_systems(OnExit(GameState::Paused), cleanup_ui::<PauseMenuUI>)
            .add_systems(OnExit(GameState::GameOver), cleanup_ui::<GameOverUI>)
            .add_systems(OnExit(GameState::LevelComplete), cleanup_ui::<LevelCompleteUI>)
            
            // Update systems
            .add_systems(Update, update_health_ui.run_if(in_state(GameState::Playing)))
            .add_systems(Update, update_weapon_ui.run_if(in_state(GameState::Playing)))
            .add_systems(Update, update_score_ui.run_if(in_state(GameState::Playing)));
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
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(
                TextBundle::from_section(
                    "BEVY EPIC SHOOTER",
                    TextStyle {
                        font: font.clone(),
                        font_size: 64.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );
            
            // Start game instruction
            parent.spawn(
                TextBundle::from_section(
                    "Press SPACE or ENTER to start",
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