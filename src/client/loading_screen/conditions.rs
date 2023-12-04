use bevy::prelude::*;

use super::{ActiveLoadingSystems, LoadingState};

pub fn is_not_loading(
    active_loading: Res<ActiveLoadingSystems>,
    state: Res<State<LoadingState>>,
) -> bool {
    **state == LoadingState::None && active_loading.is_empty()
}
