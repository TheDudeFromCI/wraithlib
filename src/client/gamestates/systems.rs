use bevy::prelude::*;
use bevy_wh_net::client::{OnDisconnectFromServer, OnJoinedServer};

use crate::client::gamestates::ClientGameState;
use crate::client::loading_screen::TransitionToState;

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
    mut on_disconnect: EventReader<OnDisconnectFromServer>,
    mut do_transition: EventWriter<TransitionToState>,
) {
    for _ in on_disconnect.read().take(1) {
        do_transition.send(TransitionToState {
            state: ClientGameState::MainMenu,
        });
        debug!("Client closing world.");
    }
}
