use bevy::app::AppExit;
use bevy::prelude::*;

use super::{
    BackButton,
    CreditsButton,
    MainMenuScreenLerp,
    MainMenuState,
    MultiplayerButton,
    OpenCreditsScreenEvent,
    OpenMultiplayerScreenEvent,
    OpenSettingsScreenEvent,
    OpenSinglePlayerScreenEvent,
    OpenTitleScreenEvent,
    QuitButton,
    SettingsButton,
    SinglePlayerButton,
};

pub(super) fn init_main_menu(mut next_state: ResMut<NextState<MainMenuState>>) {
    next_state.set(MainMenuState::TitleScreen);
}

pub(super) fn single_player_button(
    time: Res<Time>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<SinglePlayerButton>)>,
    mut screen_lerp: ResMut<MainMenuScreenLerp>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut show_screen_evs: EventWriter<OpenSinglePlayerScreenEvent>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            next_state.set(MainMenuState::SinglePlayerScreen);
            screen_lerp.start(time.elapsed_seconds(), false);
            show_screen_evs.send(OpenSinglePlayerScreenEvent);
        }
    }
}

pub(super) fn multiplayer_button(
    time: Res<Time>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<MultiplayerButton>)>,
    mut screen_lerp: ResMut<MainMenuScreenLerp>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut show_screen_evs: EventWriter<OpenMultiplayerScreenEvent>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            next_state.set(MainMenuState::MultiplayerScreen);
            screen_lerp.start(time.elapsed_seconds(), false);
            show_screen_evs.send(OpenMultiplayerScreenEvent);
        }
    }
}

pub(super) fn settings_button(
    time: Res<Time>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    mut screen_lerp: ResMut<MainMenuScreenLerp>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut show_screen_evs: EventWriter<OpenSettingsScreenEvent>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            next_state.set(MainMenuState::SettingsScreen);
            screen_lerp.start(time.elapsed_seconds(), false);
            show_screen_evs.send(OpenSettingsScreenEvent);
        }
    }
}

pub(super) fn credits_button(
    time: Res<Time>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<CreditsButton>)>,
    mut screen_lerp: ResMut<MainMenuScreenLerp>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut show_screen_evs: EventWriter<OpenCreditsScreenEvent>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            next_state.set(MainMenuState::CreditsScreen);
            screen_lerp.start(time.elapsed_seconds(), false);
            show_screen_evs.send(OpenCreditsScreenEvent);
        }
    }
}

pub(super) fn back_button(
    time: Res<Time>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut screen_lerp: ResMut<MainMenuScreenLerp>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut show_screen_evs: EventWriter<OpenTitleScreenEvent>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            next_state.set(MainMenuState::TitleScreen);
            screen_lerp.start(time.elapsed_seconds(), true);
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

pub(super) fn update_screen_lerp(time: Res<Time>, mut screen_lerp: ResMut<MainMenuScreenLerp>) {
    screen_lerp.update(time.elapsed_seconds());
}
