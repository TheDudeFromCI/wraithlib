use bevy::prelude::*;

mod components;
mod events;
mod resources;
mod systems;

pub use components::*;
pub use events::*;
pub use resources::*;

use crate::client::gamestates::ClientGameState;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_state::<MainMenuState>()
            .init_resource::<MainMenuProperties>()
            .add_event::<AddServerEntry>()
            .add_systems(
                OnEnter(ClientGameState::MainMenu),
                (systems::init_main_menu, systems::build_ui),
            )
            .add_systems(OnExit(ClientGameState::MainMenu), systems::cleanup)
            .add_systems(
                Update,
                (systems::add_server_entry, systems::confirm_edit_server)
                    .run_if(in_state(ClientGameState::MainMenu)),
            );
    }
}
