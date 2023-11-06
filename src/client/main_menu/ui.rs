use bevy::prelude::*;

use super::{
    CreditsScreen,
    MultiplayerScreen,
    OpenCreditsScreenEvent,
    OpenMultiplayerScreenEvent,
    OpenSettingsScreenEvent,
    OpenSinglePlayerScreenEvent,
    OpenTitleScreenEvent,
    SettingsScreen,
    SinglePlayerScreen,
    TitleScreen,
};
use crate::client::main_menu::components::MainMenuScreen;

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
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<MultiplayerScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<MultiplayerScreen>)>,
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
