use bevy::prelude::*;

use super::AssetsWaitForLoad;

pub fn condition_is_done_loading(asset_queue: Res<AssetsWaitForLoad>) -> bool {
    asset_queue.is_empty()
}
