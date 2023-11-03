use bevy::prelude::*;

use super::{ButtonProperties, MainMenuProperties, MenuScreenProperties, TitleScreen};
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

    let title_screen = &properties.root_screen;
    match &title_screen.bg_img_path {
        Some(path) => commands.spawn((
            MainMenuUI,
            TitleScreen,
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
            TitleScreen,
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
            for btn in title_screen.buttons.iter() {
                let mut cmd = p.spawn(button_properties_into_bundle(&asset_server, btn));
                if let Some(comp) = &btn.button_comp {
                    comp(&mut cmd);
                };
            }
        });
    });

    for screen in properties.child_screens.iter() {
        let mut cmd = commands.spawn((
            MainMenuUI,
            screen_bg_into_bundle(&asset_server, screen, 100),
        ));
        if let Some(comp) = &screen.screen_comp {
            comp(&mut cmd);
        };
    }
}

pub(super) fn cleanup(ui: Query<Entity, With<MainMenuUI>>, mut commands: Commands) {
    for entity in ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn screen_bg_into_bundle(
    asset_server: &Res<AssetServer>,
    properties: &MenuScreenProperties,
    z_index: i32,
) -> ImageBundle {
    let bg_style = Style {
        position_type: PositionType::Absolute,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    };

    match &properties.bg_img_path {
        Some(path) => ImageBundle {
            style: bg_style,
            image: asset_server.load(path).into(),
            background_color: Color::rgba(1.0, 1.0, 1.0, 0.0).into(),
            z_index: ZIndex::Global(z_index),
            ..default()
        },
        None => ImageBundle {
            style: bg_style,
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            z_index: ZIndex::Global(z_index),
            ..default()
        },
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
