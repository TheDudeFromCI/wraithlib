use bevy::prelude::*;

use crate::client::main_menu::MainMenuState;

pub(super) fn init_main_menu(mut menu_state: ResMut<NextState<MainMenuState>>) {
    menu_state.set(MainMenuState::TitleScreen);
}
