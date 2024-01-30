use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_wh_net::server::ServerNetworkingPlugin;

use crate::server::gamestates::ServerGameStatePlugin;

pub mod gamestates;

#[derive(Debug, Default)]
pub struct ServerPlugins;
impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ServerGameStatePlugin)
            .add(ServerNetworkingPlugin::default())
    }
}
