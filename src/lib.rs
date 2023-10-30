use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub mod gamestates;

#[cfg(feature = "client")]
pub mod ui_animations;

#[cfg(feature = "client")]
pub mod splash;

#[cfg(feature = "client")]
#[derive(Debug, Default)]
pub struct ClientPlugins;
impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        use crate::gamestates::client::ClientGameStatePlugin;
        use crate::splash::SplashPlugin;
        use crate::ui_animations::UiAnimationsPlugin;

        PluginGroupBuilder::start::<Self>()
            .add(ClientGameStatePlugin)
            .add(UiAnimationsPlugin)
            .add(SplashPlugin::default())
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
