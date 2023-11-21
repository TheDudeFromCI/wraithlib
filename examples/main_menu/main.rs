use bevy::prelude::*;

mod ui;

use bevy::log::{Level, LogPlugin};
use wraithlib::client::loading_screen::{LoadingScreenPlugin, LoadingScreenProperties};
use wraithlib::client::main_menu::MainMenuProperties;
use wraithlib::client::splash::{SplashImageProperties, SplashPlugin};
use wraithlib::client::ClientPlugins;
use wraithlib::common::WraithLibPlugins;

fn main() {
    App::new()
        .insert_resource(MainMenuProperties {
            canvas: ui::build_canvas(),
            server_entry: Some(Box::new(ui::server_entry_builder)),
        })
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::DEBUG,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "WraithLibrary [ Main Menu Example ]".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: true,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(WraithLibPlugins)
        .add_plugins(
            ClientPlugins
                .set(SplashPlugin {
                    images: vec![SplashImageProperties {
                        path: "images/wraithaven.png".into(),
                        ..default()
                    }],
                    ..default()
                })
                .set(LoadingScreenPlugin {
                    properties: LoadingScreenProperties {
                        path: Some("images/loading.png".into()),
                        ..default()
                    },
                }),
        )
        .run();
}
