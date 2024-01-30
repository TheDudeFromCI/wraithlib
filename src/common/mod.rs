use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[cfg(feature = "files")]
pub mod files;

#[cfg(feature = "uuid")]
pub mod uuid;

#[derive(Debug, Default)]
pub struct WraithLibPlugins;
impl PluginGroup for WraithLibPlugins {
    #[allow(unused_mut)]
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        #[cfg(feature = "files")]
        {
            group = group.add(files::FilesPlugin);
        }

        group
    }
}
