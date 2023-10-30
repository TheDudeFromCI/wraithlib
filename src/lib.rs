use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub mod gamestates;

#[cfg(feature = "client")]
pub mod camera;

#[cfg(feature = "client")]
pub mod splash;

#[cfg(feature = "client")]
pub mod ui_animations;

#[cfg(feature = "client")]
#[derive(Debug, Default)]
pub struct ClientPlugins;

#[cfg(feature = "client")]
impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        use crate::camera::CameraPlugin;
        use crate::gamestates::client::ClientGameStatePlugin;
        use crate::splash::SplashPlugin;
        use crate::ui_animations::UiAnimationsPlugin;

        PluginGroupBuilder::start::<Self>()
            .add(CameraPlugin)
            .add(ClientGameStatePlugin)
            .add(SplashPlugin::default())
            .add(UiAnimationsPlugin)
    }
}

#[cfg(feature = "server")]
#[derive(Debug, Default)]
pub struct ServerPlugins;

#[cfg(feature = "server")]
impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        use gamestates::server::ServerGameStatePlugin;

        PluginGroupBuilder::start::<Self>().add(ServerGameStatePlugin)
    }
}
