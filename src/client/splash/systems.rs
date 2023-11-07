use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, Delay};

use crate::client::gamestates::ClientGameState;
use crate::client::loading_screen::TransitionToState;
use crate::client::splash::{SplashImageView, SplashImages};
use crate::client::ui_animations::{BackgroundColorLens, FadeInOut};

pub(super) fn build_splash(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    splash_images: Res<SplashImages>,
    mut commands: Commands,
) {
    let mut required_duration = splash_images.start_delay + splash_images.end_delay;
    if splash_images.images.is_empty() {
        commands.spawn(SplashImageView { end_time: 0.0 });
        return;
    }

    required_duration += splash_images.delay_between * (splash_images.images.len() - 1) as f32;
    for img in &splash_images.images {
        required_duration += img.fade_in_duration;
        required_duration += img.display_duration;
        required_duration += img.fade_out_duration;
    }

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            SplashImageView {
                end_time: time.elapsed_seconds() + required_duration,
            },
        ))
        .with_children(|parent| {
            for (splash_index, splash_image) in splash_images.images.iter().enumerate() {
                let mut start_delay = splash_images.start_delay;
                for (j, img) in splash_images.images.iter().enumerate() {
                    if splash_index == j {
                        break;
                    }
                    start_delay += splash_images.delay_between;
                    start_delay += img.fade_in_duration;
                    start_delay += img.display_duration;
                    start_delay += img.fade_out_duration;
                }

                parent.spawn((
                    ImageBundle {
                        style: Style {
                            width: Val::Auto,
                            height: Val::Percent(100.0 * splash_image.scale),
                            aspect_ratio: Some(1.0),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        image: asset_server.load(splash_image.path.clone()).into(),
                        ..default()
                    },
                    Animator::new(Delay::new(Duration::from_secs_f32(start_delay)).then(
                        FadeInOut::from(
                            splash_image.fade_in_duration,
                            splash_image.display_duration,
                            splash_image.fade_out_duration,
                            BackgroundColorLens {
                                start: Color::rgba_linear(1.0, 1.0, 1.0, 0.0),
                                end: Color::rgba_linear(1.0, 1.0, 1.0, 1.0),
                            },
                        ),
                    )),
                ));
            }
        });
}

pub(super) fn cleanup(view: Query<Entity, With<SplashImageView>>, mut commands: Commands) {
    for entity in view.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub(super) fn exit_state(
    time: Res<Time>,
    splash_view: Query<&SplashImageView>,
    mut transition_evs: EventWriter<TransitionToState>,
) {
    let end_time = splash_view.single().end_time;
    if time.elapsed_seconds() > end_time {
        transition_evs.send(TransitionToState {
            state: ClientGameState::MainMenu,
        });
    }
}
