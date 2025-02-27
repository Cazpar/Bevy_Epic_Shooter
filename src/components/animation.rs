use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Clone)]
pub struct SpriteAnimation {
    pub frames: Vec<Handle<Image>>,
    pub frame_timer: Timer,
    pub current_frame: usize,
    pub is_looping: bool,
    pub is_playing: bool,
}

impl SpriteAnimation {
    pub fn new(frames: Vec<Handle<Image>>, fps: f32, is_looping: bool) -> Self {
        Self {
            frames,
            frame_timer: Timer::new(Duration::from_secs_f32(1.0 / fps), TimerMode::Repeating),
            current_frame: 0,
            is_looping,
            is_playing: true,
        }
    }

    pub fn tick(&mut self, delta: Duration) -> bool {
        if !self.is_playing || self.frames.is_empty() {
            return false;
        }

        let timer_finished = self.frame_timer.tick(delta).just_finished();
        
        if timer_finished {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            
            // If we've reached the end and we're not looping, stop the animation
            if self.current_frame == 0 && !self.is_looping {
                self.is_playing = false;
            }
        }
        
        timer_finished
    }

    pub fn current_texture(&self) -> Option<Handle<Image>> {
        if self.frames.is_empty() {
            None
        } else {
            Some(self.frames[self.current_frame].clone())
        }
    }

    pub fn play(&mut self) {
        self.is_playing = true;
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.frame_timer.reset();
        self.is_playing = true;
    }
}

#[derive(Component)]
pub struct AnimationState {
    pub current_animation: String,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current_animation: "idle".to_string(),
        }
    }
}

// Component to mark entities that need animation updates
#[derive(Component)]
pub struct AnimatedSprite; 