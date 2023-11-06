use bevy::prelude::*;

use super::{ActiveLoadingSystems, ServerGameState};

pub(super) fn server_startup(mut state: ResMut<NextState<ServerGameState>>) {
    state.set(ServerGameState::Loading);
}

pub(super) fn server_finish_loading(
    active_loading_systems: Res<ActiveLoadingSystems>,
    mut state: ResMut<NextState<ServerGameState>>,
) {
    if active_loading_systems.is_empty() {
        state.set(ServerGameState::Online);
    }
}
