use bevy::prelude::*;

mod components;
mod events;
mod resources;
mod systems;
mod view;

pub use components::*;
pub use events::*;
pub use resources::*;

use crate::client::gamestates::ClientGameState;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_state::<MainMenuState>()
            .init_resource::<MainMenuProperties>()
            .add_event::<OpenTitleScreenEvent>()
            .add_event::<OpenSinglePlayerScreenEvent>()
            .add_event::<OpenMultiplayerScreenEvent>()
            .add_event::<OpenSettingsScreenEvent>()
            .add_event::<OpenCreditsScreenEvent>()
            .add_event::<OpenEditServerScreenEvent>()
            .add_event::<AddServerEntry>()
            .add_systems(
                OnEnter(ClientGameState::MainMenu),
                (systems::init_main_menu, view::build_ui),
            )
            .add_systems(OnExit(ClientGameState::MainMenu), view::cleanup)
            .add_systems(
                Update,
                (
                    systems::single_player_button,
                    systems::multiplayer_button,
                    systems::settings_button,
                    systems::credits_button,
                    systems::quit_button,
                    systems::back_button,
                    systems::add_server_button,
                    systems::back_to_multiplayer_button,
                    systems::confirm_edit_server_button,
                    view::button_hover,
                    view::show_title_screen,
                    view::show_single_player_screen,
                    view::show_multiplayer_screen,
                    view::show_settings_screen,
                    view::show_credits_screen,
                    view::show_edit_server_screen,
                    view::add_server_entry,
                    view::text_focus_handler,
                )
                    .run_if(in_state(ClientGameState::MainMenu)),
            );
    }
}
