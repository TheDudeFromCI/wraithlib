use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[cfg(feature = "uuid")]
use crate::common::uuid::UuidPlugin;

#[cfg(feature = "uuid")]
pub mod uuid;

#[derive(Debug, Default)]
pub struct WraithLibPlugins;
impl PluginGroup for WraithLibPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        #[cfg(feature = "uuid")]
        {
            builder = builder.add(UuidPlugin);
        }

        builder
    }
}
