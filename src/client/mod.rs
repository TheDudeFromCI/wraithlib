use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::client::camera::CameraPlugin;
use crate::client::gamestates::ClientGameStatePlugin;
use crate::client::loading_screen::LoadingScreenPlugin;
use crate::client::main_menu::MainMenuPlugin;
use crate::client::splash::SplashPlugin;
use crate::client::ui::UiPlugin;

pub mod camera;
pub mod gamestates;
pub mod loading_screen;
pub mod main_menu;
pub mod splash;
pub mod ui;

#[cfg(feature = "networking")]
pub mod networking;

#[derive(Debug, Default)]
pub struct ClientPlugins;
impl PluginGroup for ClientPlugins {
    #[allow(unused_mut)]
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>()
            .add(CameraPlugin)
            .add(ClientGameStatePlugin)
            .add(LoadingScreenPlugin::default())
            .add(MainMenuPlugin)
            .add(SplashPlugin::default())
            .add(UiPlugin);

        #[cfg(feature = "networking")]
        {
            use crate::client::networking::ClientNetworkingPlugin;
            group = group.add(ClientNetworkingPlugin);
        }

        group
    }
}
