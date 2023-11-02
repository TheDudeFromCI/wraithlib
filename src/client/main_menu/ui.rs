use bevy::prelude::*;

use super::{
    ButtonProperties,
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

    let title_screen = &properties.title_screen;
    match &title_screen.bg_img_path {
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
            if let Some(btn) = &title_screen.single_player_btn {
                p.spawn((
                    SinglePlayerButton,
                    button_properties_into_bundle(&asset_server, btn),
                ));
            }

            if let Some(btn) = &title_screen.server_list_btn {
                p.spawn((
                    MultiplayerButton,
                    button_properties_into_bundle(&asset_server, btn),
                ));
            }

            if let Some(btn) = &title_screen.settings_btn {
                p.spawn((
                    SettingsButton,
                    button_properties_into_bundle(&asset_server, btn),
                ));
            }

            if let Some(btn) = &title_screen.quit_btn {
                p.spawn((
                    QuitButton,
                    button_properties_into_bundle(&asset_server, btn),
                ));
            }
        });
    });

    if let Some(single_player_screen) = &properties.single_player_screen {
        match &single_player_screen.bg_img_path {
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
    }

    if let Some(server_list_screen) = &properties.server_list_screen {
        match &server_list_screen.bg_img_path {
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
    }

    if let Some(settings_screen) = &properties.settings_screen {
        match &settings_screen.bg_img_path {
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
}

pub(super) fn cleanup(ui: Query<Entity, With<MainMenuUI>>, mut commands: Commands) {
    for entity in ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn button_properties_into_bundle(
    asset_server: &Res<AssetServer>,
    properties: &ButtonProperties,
) -> ButtonBundle {
    let image_handle = match &properties.img_path {
        Some(path) => asset_server.load(path).into(),
        None => UiImage::default(),
    };

    ButtonBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Px(properties.img_size.x),
            height: Val::Px(properties.img_size.y),
            margin: properties.img_margin,
            ..default()
        },
        image: image_handle,
        ..default()
    }
}
