use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct BlinkingTimer {
    timer: Timer,
    interval_timer: Timer,
}

impl BlinkingTimer {
    pub fn new(length: Duration, interval: Duration) -> Self {
        Self {
            timer: Timer::new(length, TimerMode::Once),
            interval_timer: Timer::new(interval, TimerMode::Repeating),
        }
    }
    
    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
        self.interval_timer.tick(delta);
    }
    
    pub fn just_blinked(&self) -> bool {
        self.interval_timer.just_finished()
    }
    
    pub fn just_finished(&self) -> bool {
        self.timer.just_finished()
    }
}