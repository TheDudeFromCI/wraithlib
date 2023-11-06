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

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_state::<MainMenuState>()
            .init_resource::<MainMenuProperties>()
            .init_resource::<MainMenuScreenLerp>()
            .add_event::<OpenTitleScreenEvent>()
            .add_event::<OpenSinglePlayerScreenEvent>()
            .add_event::<OpenMultiplayerScreenEvent>()
            .add_event::<OpenSettingsScreenEvent>()
            .add_event::<OpenCreditsScreenEvent>()
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
                    ui::show_title_screen,
                    ui::show_single_player_screen,
                    ui::show_multiplayer_screen,
                    ui::show_settings_screen,
                    ui::show_credits_screen,
                )
                    .run_if(in_state(ClientGameState::MainMenu)),
            );
    }
}
