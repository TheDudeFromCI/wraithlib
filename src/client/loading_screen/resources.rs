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

pub fn is_not_loading(state: Res<State<LoadingState>>) -> bool {
    **state == LoadingState::None
}
