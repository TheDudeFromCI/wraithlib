use bevy::prelude::*;

mod components;
mod events;
mod resources;
mod systems;
mod ui;

pub use components::*;
pub use events::*;
pub use resources::*;

use crate::client::gamestates::ClientGameState;

#[derive(Debug, Default)]
pub struct MainMenuPlugin {
    pub display: MainMenuProperties,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .insert_resource(self.display.clone())
            .add_systems(
                OnEnter(ClientGameState::MainMenu),
                (systems::init_main_menu, ui::build_main_menu),
            )
            .add_systems(OnExit(ClientGameState::MainMenu), ui::cleanup);
    }
}
