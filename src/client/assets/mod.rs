use bevy::prelude::*;

mod conditions;
mod resources;
mod systems;

pub use conditions::*;
pub use resources::*;

#[derive(Debug, Default)]
pub struct WhAssetPlugin;

impl Plugin for WhAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetsWaitForLoad>()
            .add_systems(Update, systems::update_asset_queue);
    }
}
