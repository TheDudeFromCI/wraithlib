use bevy::prelude::*;

use super::{
    BackButton,
    CreditsButton,
    CreditsScreen,
    LoadGameButton,
    MainMenuProperties,
    MainMenuScreen,
    MultiplayerButton,
    MultiplayerScreen,
    NewGameButton,
    QuitButton,
    SettingsButton,
    SettingsScreen,
    SinglePlayerButton,
    SinglePlayerScreen,
    TitleScreen,
};

pub(super) fn build_ui(
    properties: Res<MainMenuProperties>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let bg_style = Style {
        position_type: PositionType::Absolute,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        display: Display::None,
        ..default()
    };

    let btn_col_style = Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(250.0),
        height: Val::Px(400.0),
        ..default()
    };

    let btn_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect {
            left: Val::Px(10.0),
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            bottom: Val::Px(10.0),
        },
        ..default()
    };

    if let Some(screen) = &properties.title_screen {
        commands
            .spawn((
                MainMenuScreen,
                TitleScreen,
                ImageBundle {
                    style: Style {
                        display: Display::Flex,
                        ..bg_style.clone()
                    },
                    image: asset_server.load(&screen.bg_img_path).into(),
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: btn_col_style.clone(),
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|p| {
                    if let Some(button) = &screen.single_player_button {
                        p.spawn((
                            SinglePlayerButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));
                    }

                    if let Some(button) = &screen.multiplayer_button {
                        p.spawn((
                            MultiplayerButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));
                    }

                    if let Some(button) = &screen.settings_button {
                        p.spawn((
                            SettingsButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));
                    }

                    if let Some(button) = &screen.credits_button {
                        p.spawn((
                            CreditsButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));
                    }

                    if let Some(button) = &screen.quit_button {
                        p.spawn((
                            QuitButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));
                    }
                });
            });
    }

    if let Some(screen) = &properties.single_player_screen {
        commands
            .spawn((
                MainMenuScreen,
                SinglePlayerScreen,
                ImageBundle {
                    style: bg_style.clone(),
                    image: asset_server.load(&screen.bg_img_path).into(),
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: btn_col_style.clone(),
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|p| {
                    let button = &screen.new_game_button;
                    p.spawn((
                        NewGameButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(button.img_size.x),
                                height: Val::Px(button.img_size.y),
                                ..btn_style.clone()
                            },
                            image: asset_server.load(&button.img_path).into(),
                            ..default()
                        },
                    ));

                    let button = &screen.load_game_button;
                    p.spawn((
                        LoadGameButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(button.img_size.x),
                                height: Val::Px(button.img_size.y),
                                ..btn_style.clone()
                            },
                            image: asset_server.load(&button.img_path).into(),
                            ..default()
                        },
                    ));

                    let button = &screen.back_button;
                    p.spawn((
                        BackButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(button.img_size.x),
                                height: Val::Px(button.img_size.y),
                                ..btn_style.clone()
                            },
                            image: asset_server.load(&button.img_path).into(),
                            ..default()
                        },
                    ));
                });
            });
    }

    if let Some(screen) = &properties.multiplayer_screen {
        commands
            .spawn((
                MainMenuScreen,
                MultiplayerScreen,
                ImageBundle {
                    style: bg_style.clone(),
                    image: asset_server.load(&screen.bg_img_path).into(),
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: btn_col_style.clone(),
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|p| {
                    let button = &screen.back_button;
                    p.spawn((
                        BackButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(button.img_size.x),
                                height: Val::Px(button.img_size.y),
                                ..btn_style.clone()
                            },
                            image: asset_server.load(&button.img_path).into(),
                            ..default()
                        },
                    ));
                });
            });
    }

    if let Some(screen) = &properties.settings_screen {
        commands
            .spawn((
                MainMenuScreen,
                SettingsScreen,
                ImageBundle {
                    style: bg_style.clone(),
                    image: asset_server.load(&screen.bg_img_path).into(),
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: btn_col_style.clone(),
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|p| {
                    let button = &screen.back_button;
                    p.spawn((
                        BackButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(button.img_size.x),
                                height: Val::Px(button.img_size.y),
                                ..btn_style.clone()
                            },
                            image: asset_server.load(&button.img_path).into(),
                            ..default()
                        },
                    ));
                });
            });
    }

    if let Some(screen) = &properties.credits_screen {
        commands
            .spawn((
                MainMenuScreen,
                CreditsScreen,
                ImageBundle {
                    style: bg_style.clone(),
                    image: asset_server.load(&screen.bg_img_path).into(),
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: btn_col_style.clone(),
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|p| {
                    let button = &screen.back_button;
                    p.spawn((
                        BackButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(button.img_size.x),
                                height: Val::Px(button.img_size.y),
                                ..btn_style.clone()
                            },
                            image: asset_server.load(&button.img_path).into(),
                            ..default()
                        },
                    ));
                });
            });
    }
}
