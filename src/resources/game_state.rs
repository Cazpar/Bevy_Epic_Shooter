use bevy::prelude::*;

#[derive(Resource, Default, Debug, Clone)]
pub struct GameState {
    pub score: u32,
    pub level: u32,
    pub game_over: bool,
    pub paused: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            score: 0,
            level: 1,
            game_over: false,
            paused: false,
        }
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.level = 1;
        self.game_over = false;
        self.paused = false;
    }
}