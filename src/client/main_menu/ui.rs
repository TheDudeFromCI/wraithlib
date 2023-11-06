use bevy::prelude::*;

use super::{
    ButtonProperties,
    CreditsScreen,
    MainMenuProperties,
    MenuScreenProperties,
    OpenCreditsScreenEvent,
    OpenMultiplayerScreenEvent,
    OpenSettingsScreenEvent,
    OpenSinglePlayerScreenEvent,
    OpenTitleScreenEvent,
    ServerListScreen,
    SettingsScreen,
    SinglePlayerScreen,
    TitleScreen,
};
use crate::client::main_menu::components::MainMenuScreen;

pub(super) fn build_main_menu(
    properties: Res<MainMenuProperties>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let cmd = &mut commands;
    let server = &asset_server;

    let screen = &properties.title_screen;
    spawn_screen(server, screen, true, cmd, TitleScreen);

    let screen = &properties.single_player_screen;
    if let Some(screen) = screen {
        spawn_screen(server, screen, false, cmd, SinglePlayerScreen);
    }

    let screen = &properties.multiplayer_screen;
    if let Some(screen) = screen {
        spawn_screen(server, screen, false, cmd, ServerListScreen);
    }

    let screen = &properties.settings_screen;
    if let Some(screen) = screen {
        spawn_screen(server, screen, false, cmd, SettingsScreen);
    }

    let screen = &properties.credits_screen;
    if let Some(screen) = screen {
        spawn_screen(server, screen, false, cmd, CreditsScreen);
    }
}

pub(super) fn cleanup(ui: Query<Entity, With<MainMenuScreen>>, mut commands: Commands) {
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

pub(super) fn show_title_screen(
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<TitleScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<TitleScreen>)>,
    mut open_screen_evs: EventReader<OpenTitleScreenEvent>,
) {
    for _ in open_screen_evs.iter() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}

pub(super) fn show_single_player_screen(
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<SinglePlayerScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<SinglePlayerScreen>)>,
    mut open_screen_evs: EventReader<OpenSinglePlayerScreenEvent>,
) {
    for _ in open_screen_evs.iter() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}

pub(super) fn show_multiplayer_screen(
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<ServerListScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<ServerListScreen>)>,
    mut open_screen_evs: EventReader<OpenMultiplayerScreenEvent>,
) {
    for _ in open_screen_evs.iter() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}

pub(super) fn show_settings_screen(
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<SettingsScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<SettingsScreen>)>,
    mut open_screen_evs: EventReader<OpenSettingsScreenEvent>,
) {
    for _ in open_screen_evs.iter() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}

pub(super) fn show_credits_screen(
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<CreditsScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<CreditsScreen>)>,
    mut open_screen_evs: EventReader<OpenCreditsScreenEvent>,
) {
    for _ in open_screen_evs.iter() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}

fn spawn_screen<B: Bundle>(
    asset_server: &Res<AssetServer>,
    properties: &MenuScreenProperties,
    display: bool,
    commands: &mut Commands,
    bundle: B,
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
        MainMenuScreen,
        bundle,
        match &properties.bg_img_path {
            Some(path) => ImageBundle {
                style: bg_style,
                image: asset_server.load(path).into(),
                background_color: Color::rgba(1.0, 1.0, 1.0, 1.0).into(),
                ..default()
            },
            None => ImageBundle {
                style: bg_style,
                background_color: Color::rgba(0.0, 0.0, 0.0, 1.0).into(),
                ..default()
            },
        },
    ));

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
