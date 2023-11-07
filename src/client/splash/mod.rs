use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub use components::*;
pub use resources::*;

use crate::client::gamestates::ClientGameState;
use crate::client::loading_screen::is_not_loading;

#[derive(Debug)]
pub struct SplashPlugin {
    pub images: Vec<SplashImageProperties>,
    pub start_delay: f32,
    pub end_delay: f32,
    pub delay_between: f32,
}

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SplashImages {
            images: self.images.clone(),
            start_delay: self.start_delay,
            end_delay: self.end_delay,
            delay_between: self.delay_between,
        })
        .add_systems(OnEnter(ClientGameState::Splash), systems::build_splash)
        .add_systems(OnExit(ClientGameState::Splash), systems::cleanup)
        .add_systems(
            Update,
            systems::exit_state
                .run_if(in_state(ClientGameState::Splash))
                .run_if(is_not_loading),
        );
    }
}

impl Default for SplashPlugin {
    fn default() -> Self {
        Self {
            images: vec![],
            start_delay: 1.0,
            end_delay: 0.5,
            delay_between: 0.5,
        }
    }
}
