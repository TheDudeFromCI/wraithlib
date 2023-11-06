use bevy::prelude::*;

use super::ActiveDownloadingSystems;
use crate::client::gamestates::ClientGameState;

pub(super) fn client_startup(mut state: ResMut<NextState<ClientGameState>>) {
    state.set(ClientGameState::Splash);
    debug!("Client entering splash game state.");
}

pub(super) fn finish_downloading_connection(
    active_downloading: Res<ActiveDownloadingSystems>,
    mut state: ResMut<NextState<ClientGameState>>,
) {
    if active_downloading.is_empty() {
        state.set(ClientGameState::Online);
        debug!("Client entering online game state.");
    }
}

pub(super) fn finish_disconnecting(mut state: ResMut<NextState<ClientGameState>>) {
    state.set(ClientGameState::MainMenu);
    debug!("Client re-entering main menu game state.");
}
