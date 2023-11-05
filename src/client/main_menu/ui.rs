use bevy::prelude::*;

use super::{ButtonProperties, MainMenuProperties, MenuScreenProperties};
use crate::client::main_menu::components::MainMenuUI;

pub(super) fn build_main_menu(
    properties: Res<MainMenuProperties>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let title_screen = &properties.root_screen;
    spawn_screen(&asset_server, title_screen, 0, true, &mut commands);

    for screen in properties.child_screens.iter() {
        spawn_screen(&asset_server, screen, 100, false, &mut commands);
    }
}

pub(super) fn cleanup(ui: Query<Entity, With<MainMenuUI>>, mut commands: Commands) {
    for entity in ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub(super) fn button_hover(
    mut ui: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in ui.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *color = Color::rgba(0.75, 0.75, 0.75, 1.0).into();
            }
            Interaction::None => {
                *color = Color::rgba(1.0, 1.0, 1.0, 1.0).into();
            }
            _ => {}
        }
    }
}

fn spawn_screen(
    asset_server: &Res<AssetServer>,
    properties: &MenuScreenProperties,
    z_index: i32,
    display: bool,
    commands: &mut Commands,
) {
    let display = if display {
        Display::Flex
    } else {
        Display::None
    };

    let bg_style = Style {
        position_type: PositionType::Absolute,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        display,
        ..default()
    };

    let mut cmd = commands.spawn((
        MainMenuUI,
        match &properties.bg_img_path {
            Some(path) => ImageBundle {
                style: bg_style,
                image: asset_server.load(path).into(),
                background_color: Color::rgba(1.0, 1.0, 1.0, 1.0).into(),
                z_index: ZIndex::Global(z_index),
                ..default()
            },
            None => ImageBundle {
                style: bg_style,
                background_color: Color::rgba(0.0, 0.0, 0.0, 1.0).into(),
                z_index: ZIndex::Global(z_index),
                ..default()
            },
        },
    ));
    if let Some(comp) = &properties.screen_comp {
        comp(&mut cmd);
    };

    cmd.with_children(|p| {
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
            for btn in properties.buttons.iter() {
                let mut cmd = p.spawn(button_properties_into_bundle(asset_server, btn));
                if let Some(comp) = &btn.button_comp {
                    comp(&mut cmd);
                };
            }
        });
    });
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
