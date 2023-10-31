use bevy::prelude::*;

use crate::client::gamestates::ClientGameState;

pub(super) fn client_startup(mut state: ResMut<NextState<ClientGameState>>) {
    state.set(ClientGameState::Splash);
}
