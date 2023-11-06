use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClientGameState {
    #[default]
    Init,
    Splash,
    MainMenu,
    BuildingWorld,
    Online,
    ClosingWorld,
}

#[derive(Debug, Default, Resource)]
pub struct ActiveWorldBuildingSystems {
    count: usize,
}

impl ActiveWorldBuildingSystems {
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

#[derive(Debug, Default, Resource)]
pub struct ActiveWorldClosingSystems {
    count: usize,
}

impl ActiveWorldClosingSystems {
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
