use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, EaseFunction, Tween};

use super::{LoadingScreenBG, LoadingScreenFG, LoadingScreenProperties, LoadingScreenUI};
use crate::client::gamestates::ClientGameState;
use crate::client::loading_screen::{ActiveLoadingScreen, LoadingState, TransitionToState};
use crate::client::ui_animations::BackgroundColorLens;

pub(super) fn preload_loading_img(
    asset_server: Res<AssetServer>,
    loading_path: Res<LoadingScreenProperties>,
    mut active_loading: ResMut<ActiveLoadingScreen>,
) {
    active_loading.loading_img = loading_path
        .path
        .as_ref()
        .map(|path| asset_server.load(path).into());
}

pub(super) fn trigger_transition_to_state(
    time: Res<Time>,
    properties: Res<LoadingScreenProperties>,
    mut active_loading: ResMut<ActiveLoadingScreen>,
    mut next_state: ResMut<NextState<LoadingState>>,
    mut events: EventReader<TransitionToState>,
) {
    for ev in events.iter().take(1) {
        active_loading.state = ev.state;
        active_loading.fade_time_end = time.elapsed_seconds() + properties.fade_in_time;
        next_state.set(LoadingState::StartingLoad);
    }
}

pub(super) fn apply_transition_state(
    active_loading: ResMut<ActiveLoadingScreen>,
    mut next_state: ResMut<NextState<ClientGameState>>,
) {
    next_state.set(active_loading.state);
}

pub(super) fn wait_for_fade_out(
    time: Res<Time>,
    active_loading: Res<ActiveLoadingScreen>,
    mut next_state: ResMut<NextState<LoadingState>>,
) {
    if time.elapsed_seconds() > active_loading.fade_time_end {
        next_state.set(LoadingState::Loading);
    }
}

pub(super) fn wait_for_fade_in(
    time: Res<Time>,
    active_loading: Res<ActiveLoadingScreen>,
    mut next_state: ResMut<NextState<LoadingState>>,
) {
    if time.elapsed_seconds() > active_loading.fade_time_end {
        next_state.set(LoadingState::None);
    }
}

pub(super) fn set_finishing_state(
    time: Res<Time>,
    properties: Res<LoadingScreenProperties>,
    mut active_loading: ResMut<ActiveLoadingScreen>,
    mut next_state: ResMut<NextState<LoadingState>>,
) {
    active_loading.fade_time_end = time.elapsed_seconds() + properties.fade_out_time;
    next_state.set(LoadingState::FinishingLoad);
}

pub(super) fn build_loading_screen(
    properties: Res<LoadingScreenProperties>,
    active_loading: Res<ActiveLoadingScreen>,
    mut commands: Commands,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::NONE.into(),
                z_index: ZIndex::Global(10000),
                ..default()
            },
            LoadingScreenUI,
            LoadingScreenBG,
            Animator::new(Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs_f32(properties.fade_in_time),
                BackgroundColorLens {
                    start: Color::rgba_linear(0.0, 0.0, 0.0, 0.0),
                    end: Color::rgba_linear(0.0, 0.0, 0.0, 1.0),
                },
            )),
        ))
        .with_children(|p| {
            let Some(fg) = &active_loading.loading_img else {
                return;
            };

            p.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Auto,
                        aspect_ratio: Some(1.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    image: fg.clone(),
                    z_index: ZIndex::Global(10001),
                    ..default()
                },
                LoadingScreenUI,
                LoadingScreenFG,
                Animator::new(Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_secs_f32(properties.fade_in_time),
                    BackgroundColorLens {
                        start: Color::rgba_linear(1.0, 1.0, 1.0, 0.0),
                        end: Color::rgba_linear(1.0, 1.0, 1.0, 1.0),
                    },
                )),
            ));
        });
}

pub(super) fn close_loading_screen(
    properties: Res<LoadingScreenProperties>,
    mut bg: Query<
        &mut Animator<BackgroundColor>,
        (With<LoadingScreenBG>, Without<LoadingScreenFG>),
    >,
    mut fg: Query<
        &mut Animator<BackgroundColor>,
        (With<LoadingScreenFG>, Without<LoadingScreenBG>),
    >,
) {
    for mut animator in bg.iter_mut() {
        *animator = Animator::new(Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs_f32(properties.fade_out_time),
            BackgroundColorLens {
                start: Color::rgba_linear(0.0, 0.0, 0.0, 1.0),
                end: Color::rgba_linear(0.0, 0.0, 0.0, 0.0),
            },
        ));
    }

    for mut animator in fg.iter_mut() {
        *animator = Animator::new(Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs_f32(properties.fade_out_time),
            BackgroundColorLens {
                start: Color::rgba_linear(1.0, 1.0, 1.0, 1.0),
                end: Color::rgba_linear(1.0, 1.0, 1.0, 0.0),
            },
        ));
    }
}

pub(super) fn clear_loading_screen(
    ui: Query<Entity, With<LoadingScreenUI>>,
    mut commands: Commands,
) {
    for entity in ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
