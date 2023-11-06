use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::server::gamestates::ServerGameStatePlugin;
use crate::server::networking::ServerNetworkingPlugin;

pub mod gamestates;
pub mod networking;

#[derive(Debug, Default)]
pub struct ServerPlugins;
impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ServerGameStatePlugin)
            .add(ServerNetworkingPlugin::default())
    }
}
