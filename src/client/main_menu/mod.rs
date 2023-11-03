use bevy::prelude::*;

mod components;
mod resources;
mod systems;
mod ui;

pub use components::*;
pub use resources::*;

use crate::client::gamestates::ClientGameState;

pub const TITLE_SCREEN_INDEX: usize = 0;
pub const SINGLE_PLAYER_SCREEN_INDEX: usize = 1;
pub const SERVER_LIST_SCREEN_INDEX: usize = 2;
pub const SETTINGS_SCREEN_INDEX: usize = 3;
pub const CREDITS_SCREEN_INDEX: usize = 4;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app_: &mut App) {
        app_.init_resource::<MainMenuProperties>()
            .init_resource::<MainMenuScreenLerp>()
            .init_resource::<MainMenuActiveScreen>()
            .add_systems(
                OnEnter(ClientGameState::MainMenu),
                (systems::init_main_menu, ui::build_main_menu),
            )
            .add_systems(OnExit(ClientGameState::MainMenu), ui::cleanup)
            .add_systems(
                Update,
                (
                    systems::single_player_button,
                    systems::multiplayer_button,
                    systems::settings_button,
                    systems::credits_button,
                    systems::quit_button,
                    systems::back_button,
                    systems::update_screen_lerp,
                    ui::button_hover,
                )
                    .run_if(in_state(ClientGameState::MainMenu)),
            );
    }
}
