use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::client::camera::CameraPlugin;
use crate::client::gamestates::ClientGameStatePlugin;
use crate::client::splash::SplashPlugin;
use crate::client::ui_animations::UiAnimationsPlugin;

pub mod camera;
pub mod gamestates;
pub mod splash;
pub mod ui_animations;

#[derive(Debug, Default)]
pub struct ClientPlugins;
impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CameraPlugin)
            .add(ClientGameStatePlugin)
            .add(SplashPlugin::default())
            .add(UiAnimationsPlugin)
    }
}
