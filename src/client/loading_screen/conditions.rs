use bevy::prelude::*;

use super::{AssetsWaitForLoad, LoadingState};

pub fn is_not_loading(state: Res<State<LoadingState>>) -> bool {
    **state == LoadingState::None
}

pub(super) fn condition_is_done_loading(asset_queue: Res<AssetsWaitForLoad>) -> bool {
    asset_queue.is_empty()
}
