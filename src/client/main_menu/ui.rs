use bevy::prelude::*;

use super::{
    MainMenuProperties,
    MultiplayerButton,
    QuitButton,
    ServerListBGImage,
    SettingsBGImage,
    SettingsButton,
    SinglePlayerBGImage,
    SinglePlayerButton,
    TitleScreenBGImage,
};
use crate::client::main_menu::components::MainMenuUI;

pub(super) fn build_main_menu(
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
        ..default()
    };

    let button_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(200.0),
        height: Val::Px(50.0),
        margin: UiRect {
            left: Val::Px(10.0),
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            bottom: Val::Px(10.0),
        },
        ..default()
    };

    match &properties.title_screen_img_path {
        Some(path) => commands.spawn((
            MainMenuUI,
            TitleScreenBGImage,
            ImageBundle {
                style: bg_style.clone(),
                image: asset_server.load(path).into(),
                background_color: Color::WHITE.into(),
                z_index: ZIndex::Global(0),
                ..default()
            },
        )),
        None => commands.spawn((
            MainMenuUI,
            TitleScreenBGImage,
            NodeBundle {
                style: bg_style.clone(),
                background_color: Color::BLACK.into(),
                z_index: ZIndex::Global(0),
                ..default()
            },
        )),
    }
    .with_children(|p| {
        p.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(250.0),
                height: Val::Px(400.0),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|p| {
            p.spawn((
                SinglePlayerButton,
                ButtonBundle {
                    style: button_style.clone(),
                    image: asset_server
                        .load("images/menu/buttons/singleplayer.png")
                        .into(),
                    ..default()
                },
            ));
            p.spawn((
                MultiplayerButton,
                ButtonBundle {
                    style: button_style.clone(),
                    image: asset_server
                        .load("images/menu/buttons/multiplayer.png")
                        .into(),
                    ..default()
                },
            ));
            p.spawn((
                SettingsButton,
                ButtonBundle {
                    style: button_style.clone(),
                    image: asset_server.load("images/menu/buttons/settings.png").into(),
                    ..default()
                },
            ));
            p.spawn((
                QuitButton,
                ButtonBundle {
                    style: button_style.clone(),
                    image: asset_server.load("images/menu/buttons/quit.png").into(),
                    ..default()
                },
            ));
        });
    });

    match &properties.single_player_img_path {
        Some(path) => commands.spawn((
            MainMenuUI,
            SinglePlayerBGImage,
            ImageBundle {
                style: bg_style.clone(),
                image: asset_server.load(path).into(),
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.0).into(),
                z_index: ZIndex::Global(100),
                ..default()
            },
        )),
        None => commands.spawn((
            MainMenuUI,
            SinglePlayerBGImage,
            NodeBundle {
                style: bg_style.clone(),
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                z_index: ZIndex::Global(100),
                ..default()
            },
        )),
    };

    match &properties.server_list_img_path {
        Some(path) => commands.spawn((
            MainMenuUI,
            ServerListBGImage,
            ImageBundle {
                style: bg_style.clone(),
                image: asset_server.load(path).into(),
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.0).into(),
                z_index: ZIndex::Global(100),
                ..default()
            },
        )),
        None => commands.spawn((
            MainMenuUI,
            ServerListBGImage,
            NodeBundle {
                style: bg_style.clone(),
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                z_index: ZIndex::Global(100),
                ..default()
            },
        )),
    };

    match &properties.settings_img_path {
        Some(path) => commands.spawn((
            MainMenuUI,
            SettingsBGImage,
            ImageBundle {
                style: bg_style.clone(),
                image: asset_server.load(path).into(),
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.0).into(),
                z_index: ZIndex::Global(100),
                ..default()
            },
        )),
        None => commands.spawn((
            MainMenuUI,
            SettingsBGImage,
            NodeBundle {
                style: bg_style.clone(),
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                z_index: ZIndex::Global(100),
                ..default()
            },
        )),
    };
}

pub(super) fn cleanup(ui: Query<Entity, With<MainMenuUI>>, mut commands: Commands) {
    for entity in ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
