use bevy::prelude::*;
use bevy_wh_elements::prelude::*;
use wraithlib::client::main_menu::*;
use wraithlib::common::uuid::Uuid;

pub fn build_canvas() -> BoxedElement {
    WhCanvas::new().add_children(vec![
        title_screen(),
        single_player_screen(),
        multiplayer_screen(),
        settings_screen(),
        credits_screen(),
        edit_server_screen(),
    ])
}

fn title_screen() -> BoxedElement {
    WhScreen::with_flags((MainMenuScreen, TitleScreen))
        .bg_img("images/menu/titlescreen.png")
        .add_children(vec![
            WhDiv::new()
                .direction(ElementDirection::Column, Val::Px(10.0))
                .size(Val::Px(250.0), Val::Auto)
                .add_children(vec![
                    button::<SinglePlayerButton>("images/menu/buttons/singleplayer.png"),
                    button::<MultiplayerButton>("images/menu/buttons/multiplayer.png"),
                    button::<SettingsButton>("images/menu/buttons/settings.png"),
                    button::<CreditsButton>("images/menu/buttons/credits.png"),
                    button::<QuitButton>("images/menu/buttons/quit.png"),
                ]),
        ])
}

fn single_player_screen() -> BoxedElement {
    WhScreen::with_flags((MainMenuScreen, SinglePlayerScreen))
        .bg_img("images/menu/singleplayer.png")
        .add_children(vec![
            WhDiv::new()
                .direction(ElementDirection::Column, Val::Px(10.0))
                .size(Val::Px(250.0), Val::Auto)
                .add_children(vec![
                    button::<NewGameButton>("images/menu/buttons/new_game.png"),
                    button::<LoadGameButton>("images/menu/buttons/load_game.png"),
                    button::<BackToTitleScreenButton>("images/menu/buttons/back.png"),
                ]),
        ])
}

fn multiplayer_screen() -> BoxedElement {
    WhScreen::with_flags((MainMenuScreen, MultiplayerScreen))
        .bg_img("images/menu/serverlist.png")
        .add_children(vec![
            WhDiv::new()
                .direction(ElementDirection::Column, Val::Px(10.0))
                .size(Val::Percent(80.0), Val::Percent(80.0))
                .padding(UiRect::all(Val::Px(10.0)))
                .add_children(vec![
                    WhScrollPane::with_flags((), ServerListPane)
                        .direction(ElementDirection::Column, Val::Px(10.0))
                        .justify(ElementAlignment::Top)
                        .scroll_direction(ScrollDirection::Vertical)
                        .size(Val::Percent(100.0), Val::Auto)
                        .growing(),
                    WhDiv::new()
                        .direction(ElementDirection::Row, Val::Px(10.0))
                        .justify(ElementAlignment::Right)
                        .size(Val::Percent(100.0), Val::Px(80.0))
                        .add_children(vec![
                            button::<AddServerButton>("images/menu/buttons/add_server.png"),
                            button::<BackToTitleScreenButton>("images/menu/buttons/back.png"),
                        ]),
                ]),
        ])
}

fn settings_screen() -> BoxedElement {
    WhScreen::with_flags((MainMenuScreen, SettingsScreen))
        .bg_img("images/menu/settings.png")
        .add_children(vec![
            WhDiv::new()
                .direction(ElementDirection::Column, Val::Px(10.0))
                .size(Val::Px(250.0), Val::Auto)
                .add_children(vec![
                    //
                    button::<BackToTitleScreenButton>("images/menu/buttons/back.png"),
                ]),
        ])
}

fn credits_screen() -> BoxedElement {
    WhScreen::with_flags((MainMenuScreen, CreditsScreen))
        .bg_img("images/menu/settings.png")
        .add_children(vec![
            WhDiv::new()
                .direction(ElementDirection::Column, Val::Px(10.0))
                .size(Val::Px(250.0), Val::Auto)
                .add_children(vec![
                    //
                    button::<BackToTitleScreenButton>("images/menu/buttons/back.png"),
                ]),
        ])
}

fn edit_server_screen() -> BoxedElement {
    WhScreen::with_flags((MainMenuScreen, EditServerScreen))
        .bg_img("images/menu/serverlist.png")
        .add_children(vec![
            WhDiv::new()
                .direction(ElementDirection::Column, Val::Px(10.0))
                .add_children(vec![
                    WhTextInput::new()
                        .bg_color(Color::WHITE)
                        .size(Val::Px(400.0), Val::Px(40.0))
                        .font_size(26.0),
                    WhTextInput::new()
                        .bg_color(Color::WHITE)
                        .size(Val::Px(400.0), Val::Px(40.0))
                        .font_size(26.0),
                    WhDiv::new()
                        .direction(ElementDirection::Row, Val::Px(5.0))
                        .size(Val::Px(400.0), Val::Px(60.0))
                        .add_children(vec![
                            button::<ConfirmEditServerButton>("images/menu/buttons/confirm.png"),
                            button::<BackToMultiplayerButton>("images/menu/buttons/back.png"),
                        ]),
                ]),
        ])
}

fn button<Flags>(img: &str) -> BoxedElement
where
    Flags: Bundle + Default,
{
    WhButton::with_flags(Flags::default())
        .bg_img(img)
        .size(Val::Px(200.0), Val::Px(50.0))
}

pub fn server_entry_builder(uuid: Uuid, name: &str, address: &str) -> BoxedElement {
    WhDiv::with_flags((uuid, ServerListEntry))
        .bg_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
        .size(Val::Percent(100.0), Val::Px(100.0))
        .padding(UiRect::all(Val::Px(10.0)))
        .direction(ElementDirection::Row, Val::Px(10.0))
        .add_children(vec![
            WhDiv::new()
                .bg_img("images/wraithaven.png")
                .size(Val::Auto, Val::Percent(100.0))
                .aspect_ratio(1.0),
            WhDiv::new()
                .direction(ElementDirection::Column, Val::Px(2.0))
                .size(Val::Px(1.0), Val::Percent(100.0))
                .growing()
                .add_children(vec![
                    WhDiv::new()
                        .direction(ElementDirection::Row, Val::Px(0.0))
                        .justify(ElementAlignment::Left)
                        .size(Val::Percent(100.0), Val::Px(30.0))
                        .add_children(vec![
                            WhText::new()
                                .text(name)
                                .font_size(26.0)
                                .text_color(Color::WHITE)
                                .size(Val::Auto, Val::Px(30.0))
                                .growing(),
                            WhText::new()
                                .text("0ms")
                                .font_size(26.0)
                                .text_color(Color::WHITE)
                                .size(Val::Auto, Val::Px(30.0))
                                .justify(ElementAlignment::Right),
                        ]),
                    WhText::new()
                        .text(address)
                        .font_size(26.0)
                        .text_color(Color::WHITE)
                        .size(Val::Percent(100.0), Val::Px(30.0)),
                    WhText::new()
                        .text("Server description.")
                        .font_size(26.0)
                        .text_color(Color::WHITE)
                        .size(Val::Percent(100.0), Val::Px(30.0)),
                ]),
        ])
}
