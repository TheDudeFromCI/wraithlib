use bevy::prelude::*;

use crate::client::gamestates::ClientGameState;

#[derive(Debug, Resource, Clone)]
pub struct LoadingScreenProperties {
    pub path: Option<String>,
    pub fade_in_time: f32,
    pub fade_out_time: f32,
}

impl Default for LoadingScreenProperties {
    fn default() -> Self {
        Self {
            path: None,
            fade_in_time: 0.5,
            fade_out_time: 0.5,
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct ActiveLoadingScreen {
    pub state: ClientGameState,
    pub loading_img: Option<UiImage>,
    pub fade_time_end: f32,
}

#[derive(Debug, Default, States, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoadingState {
    #[default]
    None,
    StartingLoad,
    Loading,
    FinishingLoad,
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
