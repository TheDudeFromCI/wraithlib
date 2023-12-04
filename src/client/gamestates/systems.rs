use bevy::prelude::*;

use crate::client::gamestates::ClientGameState;
use crate::client::loading_screen::TransitionToState;
use crate::client::networking::{OnDisconnectedFromServer, OnJoinedServer};

pub(super) fn client_startup(mut state: ResMut<NextState<ClientGameState>>) {
    state.set(ClientGameState::Splash);
    debug!("Client entering splash game state.");
}

pub(super) fn open_world(
    mut on_connect: EventReader<OnJoinedServer>,
    mut do_transition: EventWriter<TransitionToState>,
) {
    for _ in on_connect.read().take(1) {
        do_transition.send(TransitionToState {
            state: ClientGameState::Online,
        });
        debug!("Client opening world.");
    }
}

pub(super) fn close_world(
    mut on_disconnect: EventReader<OnDisconnectedFromServer>,
    mut do_transition: EventWriter<TransitionToState>,
) {
    for _ in on_disconnect.read().take(1) {
        do_transition.send(TransitionToState {
            state: ClientGameState::MainMenu,
        });
        debug!("Client closing world.");
    }
}
