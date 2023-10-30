use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use gamestates::{ClientGameStatePlugin, ServerGameStatePlugin};

pub mod gamestates;

#[derive(Debug, Default)]
pub struct ClientPlugins;
impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(ClientGameStatePlugin)
    }
}

#[derive(Debug, Default)]
pub struct ServerPlugins;
impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(ServerGameStatePlugin)
    }
}
