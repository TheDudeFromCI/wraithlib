use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ServerGameState {
    #[default]
    Init,
    Loading,
    Online,
    ShuttingDown,
}

#[derive(Debug, Default, Resource)]
pub struct ActiveLoadingSystems {
    count: usize,
}

impl ActiveLoadingSystems {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn decrement(&mut self) {
        self.count -= 1;
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}
