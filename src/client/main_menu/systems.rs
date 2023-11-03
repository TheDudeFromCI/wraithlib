use bevy::app::AppExit;
use bevy::prelude::*;

use super::{
    BackButton,
    CreditsButton,
    MainMenuActiveScreen,
    MainMenuScreenLerp,
    MultiplayerButton,
    QuitButton,
    SettingsButton,
    SinglePlayerButton,
    CREDITS_SCREEN_INDEX,
    SERVER_LIST_SCREEN_INDEX,
    SETTINGS_SCREEN_INDEX,
    SINGLE_PLAYER_SCREEN_INDEX,
    TITLE_SCREEN_INDEX,
};

pub(super) fn init_main_menu(mut active_screen: ResMut<MainMenuActiveScreen>) {
    active_screen.screen_index = TITLE_SCREEN_INDEX;
}

pub(super) fn single_player_button(
    time: Res<Time>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<SinglePlayerButton>)>,
    mut screen_lerp: ResMut<MainMenuScreenLerp>,
    mut active_screen: ResMut<MainMenuActiveScreen>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            active_screen.screen_index = SINGLE_PLAYER_SCREEN_INDEX;
            screen_lerp.start(time.elapsed_seconds(), false);
        }
    }
}

pub(super) fn multiplayer_button(
    time: Res<Time>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<MultiplayerButton>)>,
    mut screen_lerp: ResMut<MainMenuScreenLerp>,
    mut active_screen: ResMut<MainMenuActiveScreen>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            active_screen.screen_index = SERVER_LIST_SCREEN_INDEX;
            screen_lerp.start(time.elapsed_seconds(), false);
        }
    }
}

pub(super) fn settings_button(
    time: Res<Time>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    mut screen_lerp: ResMut<MainMenuScreenLerp>,
    mut active_screen: ResMut<MainMenuActiveScreen>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            active_screen.screen_index = SETTINGS_SCREEN_INDEX;
            screen_lerp.start(time.elapsed_seconds(), false);
        }
    }
}

pub(super) fn credits_button(
    time: Res<Time>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<CreditsButton>)>,
    mut screen_lerp: ResMut<MainMenuScreenLerp>,
    mut active_screen: ResMut<MainMenuActiveScreen>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            active_screen.screen_index = CREDITS_SCREEN_INDEX;
            screen_lerp.start(time.elapsed_seconds(), false);
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

pub(super) fn back_button(
    time: Res<Time>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut screen_lerp: ResMut<MainMenuScreenLerp>,
    mut active_screen: ResMut<MainMenuActiveScreen>,
) {
    for ev in interactions.iter() {
        if let Interaction::Pressed = *ev {
            active_screen.screen_index = TITLE_SCREEN_INDEX;
            screen_lerp.start(time.elapsed_seconds(), true);
        }
    }
}

pub(super) fn update_screen_lerp(time: Res<Time>, mut screen_lerp: ResMut<MainMenuScreenLerp>) {
    screen_lerp.update(time.elapsed_seconds());
}
