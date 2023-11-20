use bevy::prelude::*;

use super::AssetsWaitForLoad;

pub(super) fn update_asset_queue(
    asset_server: Res<AssetServer>,
    mut asset_queue: ResMut<AssetsWaitForLoad>,
) {
    asset_queue.remove_finished(&asset_server);
}
