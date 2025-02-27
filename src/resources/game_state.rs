use bevy::prelude::*;

/// Game state enum to represent different states of the game
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
    LevelComplete,
}

/// Game data resource to store game progress and statistics
#[derive(Resource, Debug, Clone)]
pub struct GameData {
    pub score: u32,
    pub level: u32,
    pub high_score: u32,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            score: 0,
            level: 1,
            high_score: 0,
        }
    }
}

impl GameData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        // Update high score before resetting
        if self.score > self.high_score {
            self.high_score = self.score;
        }
        
        self.score = 0;
        self.level = 1;
    }
    
    pub fn increment_score(&mut self, points: u32) {
        self.score += points;
    }
    
    pub fn next_level(&mut self) {
        self.level += 1;
    }
}