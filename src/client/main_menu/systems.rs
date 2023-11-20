use bevy::app::AppExit;
use bevy::prelude::*;

use super::*;

pub(super) fn init_main_menu(mut next_state: ResMut<NextState<MainMenuState>>) {
    next_state.set(MainMenuState::TitleScreen);
}

pub(super) fn single_player_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<SinglePlayerButton>)>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut show_screen_evs: EventWriter<OpenSinglePlayerScreenEvent>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            next_state.set(MainMenuState::SinglePlayerScreen);
            show_screen_evs.send(OpenSinglePlayerScreenEvent);
        }
    }
}

pub(super) fn multiplayer_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<MultiplayerButton>)>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut show_screen_evs: EventWriter<OpenMultiplayerScreenEvent>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            next_state.set(MainMenuState::MultiplayerScreen);
            show_screen_evs.send(OpenMultiplayerScreenEvent);
        }
    }
}

pub(super) fn settings_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut show_screen_evs: EventWriter<OpenSettingsScreenEvent>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            next_state.set(MainMenuState::SettingsScreen);
            show_screen_evs.send(OpenSettingsScreenEvent);
        }
    }
}

pub(super) fn credits_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<CreditsButton>)>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut show_screen_evs: EventWriter<OpenCreditsScreenEvent>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            next_state.set(MainMenuState::CreditsScreen);
            show_screen_evs.send(OpenCreditsScreenEvent);
        }
    }
}

pub(super) fn back_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<BackToTitleScreenButton>)>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut show_screen_evs: EventWriter<OpenTitleScreenEvent>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            next_state.set(MainMenuState::TitleScreen);
            show_screen_evs.send(OpenTitleScreenEvent);
        }
    }
}

pub(super) fn quit_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut exit_events: EventWriter<AppExit>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            exit_events.send(AppExit);
        }
    }
}

pub(super) fn add_server_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<AddServerButton>)>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut show_screen_evs: EventWriter<OpenEditServerScreenEvent>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            next_state.set(MainMenuState::EditServerScreen);
            show_screen_evs.send(OpenEditServerScreenEvent);
        }
    }
}

pub(super) fn back_to_multiplayer_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<BackToMultiplayerButton>)>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut show_screen_evs: EventWriter<OpenMultiplayerScreenEvent>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            next_state.set(MainMenuState::MultiplayerScreen);
            show_screen_evs.send(OpenMultiplayerScreenEvent);
        }
    }
}

pub(super) fn confirm_edit_server_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<ConfirmEditServerButton>)>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut show_screen_evs: EventWriter<OpenMultiplayerScreenEvent>,
    mut add_server_evs: EventWriter<AddServerEntry>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            next_state.set(MainMenuState::MultiplayerScreen);
            show_screen_evs.send(OpenMultiplayerScreenEvent);
            add_server_evs.send(AddServerEntry);
        }
    }
}
