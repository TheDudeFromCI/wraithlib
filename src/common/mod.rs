use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[cfg(feature = "uuid")]
pub mod uuid;

#[cfg(feature = "networking")]
pub mod networking;

#[derive(Debug, Default)]
pub struct WraithLibPlugins;
impl PluginGroup for WraithLibPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}
