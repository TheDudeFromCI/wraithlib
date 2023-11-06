use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClientGameState {
    #[default]
    Init,
    Splash,
    MainMenu,
    Downloading,
    Online,
    Disconnecting,
}

#[derive(Debug, Default, Resource)]
pub struct ActiveDownloadingSystems {
    count: usize,
}

impl ActiveDownloadingSystems {
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
