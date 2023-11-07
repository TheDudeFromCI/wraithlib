use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use wraithlib::client::main_menu::{
    ImageProperties,
    MainMenuProperties,
    MultiplayerScreenProperties,
    TitleScreenProperties,
};
use wraithlib::client::ClientPlugins;
use wraithlib::common::WraithLibPlugins;

pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(MainMenuProperties {
            title_screen: Some(TitleScreenProperties {
                bg_img_path: "images/menu/titlescreen.png".into(),
                single_player_button: None,
                multiplayer_button: Some(ImageProperties {
                    img_path: "images/menu/buttons/multiplayer.png".into(),
                    img_size: Vec2::new(200.0, 50.0),
                }),
                settings_button: None,
                credits_button: None,
                quit_button: Some(ImageProperties {
                    img_path: "images/menu/buttons/quit.png".into(),
                    img_size: Vec2::new(200.0, 50.0),
                }),
            }),
            single_player_screen: None,
            multiplayer_screen: Some(MultiplayerScreenProperties {
                bg_img_path: "images/menu/serverlist.png".into(),
                add_server_button: ImageProperties {
                    img_path: "images/menu/buttons/add_server.png".into(),
                    img_size: Vec2::new(200.0, 50.0),
                },
                back_button: ImageProperties {
                    img_path: "images/menu/buttons/back.png".into(),
                    img_size: Vec2::new(200.0, 50.0),
                },
            }),
            settings_screen: None,
            credits_screen: None,
        })
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::DEBUG,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "WraithLibrary [ Server Example ]".into(),
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
        .add_plugins(ClientPlugins)
        .run();
}
