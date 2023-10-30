use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub mod gamestates;

#[cfg(feature = "client")]
#[derive(Debug, Default)]
pub struct ClientPlugins;
impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        use gamestates::client::ClientGameStatePlugin;

        PluginGroupBuilder::start::<Self>().add(ClientGameStatePlugin)
    }
}

#[cfg(feature = "server")]
#[derive(Debug, Default)]
pub struct ServerPlugins;
impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        use gamestates::server::ServerGameStatePlugin;

        PluginGroupBuilder::start::<Self>().add(ServerGameStatePlugin)
    }
}
