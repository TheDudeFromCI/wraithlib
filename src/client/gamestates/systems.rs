use bevy::prelude::*;

use super::{ActiveWorldBuildingSystems, ActiveWorldClosingSystems};
use crate::client::gamestates::ClientGameState;

pub(super) fn client_startup(mut state: ResMut<NextState<ClientGameState>>) {
    state.set(ClientGameState::Splash);
    debug!("Client entering splash game state.");
}

pub(super) fn finish_building_world(
    active_building: Res<ActiveWorldBuildingSystems>,
    mut state: ResMut<NextState<ClientGameState>>,
) {
    if active_building.is_empty() {
        state.set(ClientGameState::Online);
        debug!("Client entering online game state.");
    }
}

pub(super) fn finish_closing_world(
    active_closing: Res<ActiveWorldClosingSystems>,
    mut state: ResMut<NextState<ClientGameState>>,
) {
    if active_closing.is_empty() {
        state.set(ClientGameState::MainMenu);
        debug!("Client re-entering main menu game state.");
    }
}
