use bevy::prelude::*;

use super::*;

pub(super) fn build_ui(
    asset_server: Res<AssetServer>,
    mut properties: ResMut<MainMenuProperties>,
    mut commands: Commands,
) {
    let mut canvas = None;
    std::mem::swap(&mut canvas, &mut properties.canvas);

    if let Some(canvas) = canvas {
        canvas.build(&mut commands, &asset_server);
    }
}

pub(super) fn add_server_entry(
    properties: Res<MainMenuProperties>,
    server_list: Query<Entity, With<ServerListPane>>,
    asset_server: Res<AssetServer>,
    mut add_server_evs: EventReader<AddServerEntry>,
    mut commands: Commands,
) {
    let Ok(server_list) = server_list.get_single() else {
        return;
    };

    for ev in add_server_evs.read() {
        let Some(builder) = &properties.server_entry else {
            continue;
        };

        let elem = builder(ev.uuid.clone(), &ev.name, &ev.address);
        elem.build_child(&mut commands, &asset_server, Some(server_list));
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
    for _ in open_screen_evs.read() {
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
    for _ in open_screen_evs.read() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}

pub(super) fn show_multiplayer_screen(
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<MultiplayerScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<MultiplayerScreen>)>,
    mut open_screen_evs: EventReader<OpenMultiplayerScreenEvent>,
) {
    for _ in open_screen_evs.read() {
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
    for _ in open_screen_evs.read() {
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
    for _ in open_screen_evs.read() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}

pub(super) fn show_edit_server_screen(
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<EditServerScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<EditServerScreen>)>,
    mut open_screen_evs: EventReader<OpenEditServerScreenEvent>,
) {
    for _ in open_screen_evs.read() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}
