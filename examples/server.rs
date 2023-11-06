use bevy::prelude::*;
use wraithlib::common::WraithLibPlugins;
use wraithlib::server::ServerPlugins;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(WraithLibPlugins)
        .add_plugins(ServerPlugins)
        .run();
}
