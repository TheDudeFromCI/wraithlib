use bevy::prelude::*;

use super::LoadingState;

pub fn is_not_loading(state: Res<State<LoadingState>>) -> bool {
    **state == LoadingState::None
}
