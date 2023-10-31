use bevy::prelude::*;

use super::LoadingState;

pub fn is_not_loading(state: Res<State<LoadingState>>) -> bool {
    **state == LoadingState::None
}

pub(super) fn condition_is_done_loading(
    time: Res<Time>,
    mut local_time: Local<Option<f32>>,
) -> bool {
    // TODO: This method of checking if the loading screen is done is a bit hacky.
    //       It would be better to have a way to check if all assets are loaded.

    if local_time.is_none() {
        *local_time = Some(time.elapsed_seconds() + 5.0);
    }

    time.elapsed_seconds() > local_time.unwrap()
}
