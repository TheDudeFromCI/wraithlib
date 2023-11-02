use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use wraithlib::client::loading_screen::{LoadingScreenPlugin, LoadingScreenProperties};
use wraithlib::client::main_menu::{
    ButtonProperties,
    MainMenuPlugin,
    MainMenuProperties,
    ServerListScreenProperties,
    SettingsScreenProperties,
    SinglePlayerScreenProperties,
    TitleScreenProperties,
};
use wraithlib::client::splash::{SplashImageProperties, SplashPlugin};
use wraithlib::client::ClientPlugins;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::DEBUG,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "WraithLibrary [ Simple Preview ]".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: true,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(
            ClientPlugins
                .set(SplashPlugin {
                    images: vec![SplashImageProperties {
                        path: "images/wraithaven.png".into(),
                        ..default()
                    }],
                    ..default()
                })
                .set(MainMenuPlugin {
                    display: MainMenuProperties {
                        title_screen: TitleScreenProperties {
                            bg_img_path: Some("images/menu/titlescreen.png".into()),
                            single_player_btn: Some(ButtonProperties {
                                img_path: Some("images/menu/buttons/singleplayer.png".into()),
                                img_size: Vec2::new(200.0, 50.0),
                                ..default()
                            }),
                            server_list_btn: Some(ButtonProperties {
                                img_path: Some("images/menu/buttons/multiplayer.png".into()),
                                img_size: Vec2::new(200.0, 50.0),
                                ..default()
                            }),
                            settings_btn: Some(ButtonProperties {
                                img_path: Some("images/menu/buttons/settings.png".into()),
                                img_size: Vec2::new(200.0, 50.0),
                                ..default()
                            }),
                            quit_btn: Some(ButtonProperties {
                                img_path: Some("images/menu/buttons/quit.png".into()),
                                img_size: Vec2::new(200.0, 50.0),
                                ..default()
                            }),
                        },
                        single_player_screen: Some(SinglePlayerScreenProperties {
                            bg_img_path: Some("images/menu/singleplayer.png".into()),
                            back_btn: Some(ButtonProperties {
                                img_path: Some("images/menu/buttons/back.png".into()),
                                img_size: Vec2::new(200.0, 50.0),
                                ..default()
                            }),
                        }),
                        server_list_screen: Some(ServerListScreenProperties {
                            bg_img_path: Some("images/menu/serverlist.png".into()),
                            back_btn: Some(ButtonProperties {
                                img_path: Some("images/menu/buttons/back.png".into()),
                                img_size: Vec2::new(200.0, 50.0),
                                ..default()
                            }),
                        }),
                        settings_screen: Some(SettingsScreenProperties {
                            bg_img_path: Some("images/menu/settings.png".into()),
                            back_btn: Some(ButtonProperties {
                                img_path: Some("images/menu/buttons/back.png".into()),
                                img_size: Vec2::new(200.0, 50.0),
                                ..default()
                            }),
                        }),
                    },
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
