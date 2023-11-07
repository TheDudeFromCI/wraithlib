use bevy::prelude::*;
use wraithlib::common::WraithLibPlugins;
use wraithlib::server::ServerPlugins;

pub fn run() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(WraithLibPlugins)
        .add_plugins(ServerPlugins)
        .run();
}
