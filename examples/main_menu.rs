use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use wraithlib::client::loading_screen::{LoadingScreenPlugin, LoadingScreenProperties};
use wraithlib::client::main_menu::{
    EditServerScreenProperties,
    ImageProperties,
    MainMenuProperties,
    MultiplayerScreenProperties,
    SettingsScreenProperties,
    SinglePlayerScreenProperties,
    TitleScreenProperties,
};
use wraithlib::client::splash::{SplashImageProperties, SplashPlugin};
use wraithlib::client::ClientPlugins;
use wraithlib::common::WraithLibPlugins;

fn main() {
    App::new()
        .insert_resource(MainMenuProperties {
            title_screen: Some(TitleScreenProperties {
                bg_img_path: "images/menu/titlescreen.png".into(),
                single_player_button: Some(ImageProperties {
                    img_path: "images/menu/buttons/singleplayer.png".into(),
                    img_size: Vec2::new(200.0, 50.0),
                }),
                multiplayer_button: Some(ImageProperties {
                    img_path: "images/menu/buttons/multiplayer.png".into(),
                    img_size: Vec2::new(200.0, 50.0),
                }),
                settings_button: Some(ImageProperties {
                    img_path: "images/menu/buttons/settings.png".into(),
                    img_size: Vec2::new(200.0, 50.0),
                }),
                credits_button: None,
                quit_button: Some(ImageProperties {
                    img_path: "images/menu/buttons/quit.png".into(),
                    img_size: Vec2::new(200.0, 50.0),
                }),
            }),
            single_player_screen: Some(SinglePlayerScreenProperties {
                bg_img_path: "images/menu/singleplayer.png".into(),
                new_game_button: ImageProperties {
                    img_path: "images/menu/buttons/new_game.png".into(),
                    img_size: Vec2::new(200.0, 50.0),
                },
                load_game_button: ImageProperties {
                    img_path: "images/menu/buttons/load_game.png".into(),
                    img_size: Vec2::new(200.0, 50.0),
                },
                back_button: ImageProperties {
                    img_path: "images/menu/buttons/back.png".into(),
                    img_size: Vec2::new(200.0, 50.0),
                },
            }),
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
                edit_server_screen: EditServerScreenProperties {
                    bg_img_path: "images/menu/serverlist.png".into(),
                    confirm_button: ImageProperties {
                        img_path: "images/menu/buttons/confirm.png".into(),
                        img_size: Vec2::new(200.0, 50.0),
                    },
                    back_button: ImageProperties {
                        img_path: "images/menu/buttons/back.png".into(),
                        img_size: Vec2::new(200.0, 50.0),
                    },
                },
            }),
            settings_screen: Some(SettingsScreenProperties {
                bg_img_path: "images/menu/settings.png".into(),
                back_button: ImageProperties {
                    img_path: "images/menu/buttons/back.png".into(),
                    img_size: Vec2::new(200.0, 50.0),
                },
            }),
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
