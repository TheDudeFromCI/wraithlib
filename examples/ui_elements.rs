use bevy::prelude::*;
use wraithlib::client::assets::{AssetsWaitForLoad, WhAssetPlugin};
use wraithlib::client::ui::elements::*;
use wraithlib::client::ui::WhUiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WhUiPlugin)
        .add_plugins(WhAssetPlugin)
        .add_systems(Startup, init)
        .run();
}

#[derive(Default, Component)]
struct PlayButton;

#[derive(Default, Component)]
struct SettingsButton;

#[derive(Default, Component)]
struct QuitButton;

fn init(
    asset_server: Res<AssetServer>,
    mut asset_loader: ResMut<AssetsWaitForLoad>,
    mut commands: Commands,
) {
    let mut loader = asset_loader.with_server(&asset_server);
    let background_image = NodeBackground::Image("images/menu/titlescreen.png".into());
    let red_button = NodeBackground::Color(Color::rgb(1.0, 0.0, 0.0));
    let green_button = NodeBackground::Color(Color::rgb(0.0, 1.0, 0.0));
    let blue_button = NodeBackground::Color(Color::rgb(0.0, 0.0, 1.0));

    commands.spawn(Camera2dBundle::default());

    WhCanvas::<()>::default()
        .add_children(vec![
            WhScreen::<()>::default()
                .background(background_image.clone())
                .add_children(vec![
                    WhDiv::<()>::default()
                        .size(Val::Px(200.0), Val::Px(450.0))
                        .direction(ElementDirection::Column, Val::Px(10.0))
                        .add_children(vec![
                            WhButton::<PlayButton>::default()
                                .background(red_button.clone())
                                .size(Val::Px(200.0), Val::Px(50.0))
                                .boxed(),
                            WhButton::<SettingsButton>::default()
                                .background(green_button.clone())
                                .size(Val::Px(200.0), Val::Px(50.0))
                                .boxed(),
                            WhButton::<QuitButton>::default()
                                .background(blue_button.clone())
                                .size(Val::Px(200.0), Val::Px(50.0))
                                .boxed(),
                        ])
                        .boxed(),
                ])
                .boxed(),
        ])
        .build(&mut commands, &mut loader);
}
