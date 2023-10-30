use bevy::prelude::*;

use crate::gamestates::client::ClientGameState;

#[derive(Debug)]
pub struct SplashPlugin {
    pub images: Vec<SplashImageProperties>,
    pub start_delay: f32,
    pub end_delay: f32,
    pub delay_between: f32,
}

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SplashImages {
            images: self.images.clone(),
            start_delay: self.start_delay,
            end_delay: self.end_delay,
            delay_between: self.delay_between,
        })
        .add_systems(OnEnter(ClientGameState::Splash), systems::build_splash)
        .add_systems(OnExit(ClientGameState::Splash), systems::cleanup)
        .add_systems(
            Update,
            systems::exit_state.run_if(in_state(ClientGameState::Splash)),
        );
    }
}

impl Default for SplashPlugin {
    fn default() -> Self {
        Self {
            images: vec![SplashImageProperties::default()],
            start_delay: 1.0,
            end_delay: 0.5,
            delay_between: 0.5,
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct SplashImages {
    pub images: Vec<SplashImageProperties>,
    pub start_delay: f32,
    pub end_delay: f32,
    pub delay_between: f32,
}

#[derive(Debug, Clone)]
pub struct SplashImageProperties {
    pub path: String,
    pub scale: f32,
    pub fade_in_duration: f32,
    pub display_duration: f32,
    pub fade_out_duration: f32,
}

impl Default for SplashImageProperties {
    fn default() -> Self {
        Self {
            path: "splash.png".into(),
            scale: 0.75,
            fade_in_duration: 0.5,
            display_duration: 1.5,
            fade_out_duration: 0.5,
        }
    }
}

#[derive(Debug, Component)]
struct SplashImageView {
    end_time: f32,
}

mod systems {
    use std::time::Duration;

    use bevy_tweening::{Animator, Delay};

    use super::*;
    use crate::ui_animations::{BackgroundColorLens, FadeInOut};

    pub(super) fn build_splash(
        time: Res<Time>,
        asset_server: Res<AssetServer>,
        splash_images: Res<SplashImages>,
        mut commands: Commands,
    ) {
        let mut required_duration = splash_images.start_delay + splash_images.end_delay;
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
        mut state: ResMut<NextState<ClientGameState>>,
    ) {
        let end_time = splash_view.single().end_time;
        if time.elapsed_seconds() > end_time {
            state.set(ClientGameState::MainMenu);
        }
    }
}
