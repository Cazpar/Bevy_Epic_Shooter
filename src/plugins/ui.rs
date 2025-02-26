use bevy::prelude::*;
use crate::resources::game_state::GameState;
use crate::components::player::Player;
use crate::components::weapon::{Weapon, WeaponType};
use crate::components::pickup::WeaponUpgrades;

// UI Plugin
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui)
            .add_systems(Update, update_health_ui)
            .add_systems(Update, update_weapon_ui);
    }
}

// UI Components
#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct WeaponText;

// Setup UI
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Health UI
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Health: ",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "100",
                TextStyle {
                    font_size: 24.0,
                    color: Color::GREEN,
                    ..default()
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
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Weapon: ",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "Pistol",
                TextStyle {
                    font_size: 24.0,
                    color: Color::YELLOW,
                    ..default()
                },
            ),
            TextSection::new(
                "\nUpgrades: None",
                TextStyle {
                    font_size: 20.0,
                    color: Color::SILVER,
                    ..default()
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