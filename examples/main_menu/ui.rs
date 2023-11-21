use bevy::prelude::*;
use wraithlib::client::main_menu::*;
use wraithlib::client::ui::elements::*;

pub fn build_canvas() -> WhCanvas {
    WhCanvas::<()>::default().add_children(vec![
        title_screen(),
        single_player_screen(),
        multiplayer_screen(),
        settings_screen(),
        credits_screen(),
        edit_server_screen(),
    ])
}

fn title_screen() -> BoxedElement {
    WhScreen::<(MainMenuScreen, TitleScreen)>::default()
        .background(NodeBackground::Image("images/menu/titlescreen.png".into()))
        .add_children(vec![
            WhDiv::<()>::default()
                .direction(ElementDirection::Column, Val::Px(10.0))
                .size(Val::Px(250.0), Val::Auto)
                .add_children(vec![
                    button::<SinglePlayerButton>("images/menu/buttons/singleplayer.png"),
                    button::<MultiplayerButton>("images/menu/buttons/multiplayer.png"),
                    button::<SettingsButton>("images/menu/buttons/settings.png"),
                    button::<CreditsButton>("images/menu/buttons/credits.png"),
                    button::<QuitButton>("images/menu/buttons/quit.png"),
                ])
                .boxed(),
        ])
        .hidden(false)
        .boxed()
}

fn single_player_screen() -> BoxedElement {
    WhScreen::<(MainMenuScreen, SinglePlayerScreen)>::default()
        .background(NodeBackground::Image("images/menu/singleplayer.png".into()))
        .add_children(vec![
            WhDiv::<()>::default()
                .direction(ElementDirection::Column, Val::Px(10.0))
                .size(Val::Px(250.0), Val::Auto)
                .add_children(vec![
                    button::<NewGameButton>("images/menu/buttons/new_game.png"),
                    button::<LoadGameButton>("images/menu/buttons/load_game.png"),
                    button::<BackToTitleScreenButton>("images/menu/buttons/back.png"),
                ])
                .boxed(),
        ])
        .hidden(true)
        .boxed()
}

fn multiplayer_screen() -> BoxedElement {
    WhScreen::<(MainMenuScreen, MultiplayerScreen)>::default()
        .background(NodeBackground::Image("images/menu/serverlist.png".into()))
        .add_children(vec![
            WhDiv::<()>::default()
                .direction(ElementDirection::Column, Val::Px(10.0))
                .size(Val::Percent(80.0), Val::Percent(80.0))
                .padding(UiRect::all(Val::Px(10.0)))
                .add_children(vec![
                    WhScrollPane::<(), ServerListPane>::default()
                        .direction(ElementDirection::Column, Val::Px(10.0))
                        .justify(ElementJustify::Start)
                        .growing()
                        .outer_size(Val::Percent(100.0), Val::Auto)
                        .inner_size(Val::Percent(100.0), Val::Auto)
                        .add_children(vec![
                            //
                            server_entry(),
                            server_entry(),
                            server_entry(),
                            server_entry(),
                            server_entry(),
                            server_entry(),
                            server_entry(),
                        ])
                        .boxed(),
                    WhDiv::<()>::default()
                        .direction(ElementDirection::Row, Val::Px(10.0))
                        .justify(ElementJustify::End)
                        .size(Val::Percent(100.0), Val::Px(80.0))
                        .add_children(vec![
                            button::<AddServerButton>("images/menu/buttons/add_server.png"),
                            button::<BackToTitleScreenButton>("images/menu/buttons/back.png"),
                        ])
                        .boxed(),
                ])
                .boxed(),
        ])
        .hidden(true)
        .boxed()
}

fn settings_screen() -> BoxedElement {
    WhScreen::<(MainMenuScreen, SettingsScreen)>::default()
        .background(NodeBackground::Image("images/menu/settings.png".into()))
        .add_children(vec![
            WhDiv::<()>::default()
                .direction(ElementDirection::Column, Val::Px(10.0))
                .size(Val::Px(250.0), Val::Auto)
                .add_children(vec![
                    //
                    button::<BackToTitleScreenButton>("images/menu/buttons/back.png"),
                ])
                .boxed(),
        ])
        .hidden(true)
        .boxed()
}

fn credits_screen() -> BoxedElement {
    WhScreen::<(MainMenuScreen, CreditsScreen)>::default()
        .background(NodeBackground::Image("images/menu/settings.png".into()))
        .add_children(vec![
            WhDiv::<()>::default()
                .direction(ElementDirection::Column, Val::Px(10.0))
                .size(Val::Px(250.0), Val::Auto)
                .add_children(vec![
                    //
                    button::<BackToTitleScreenButton>("images/menu/buttons/back.png"),
                ])
                .boxed(),
        ])
        .hidden(true)
        .boxed()
}

fn edit_server_screen() -> BoxedElement {
    WhScreen::<(MainMenuScreen, EditServerScreen)>::default()
        .background(NodeBackground::Image("images/menu/serverlist.png".into()))
        .add_children(vec![
            WhDiv::<()>::default()
                .direction(ElementDirection::Column, Val::Px(10.0))
                .add_children(vec![
                    WhTextInput::<()>::default()
                        .background(NodeBackground::Color(Color::WHITE))
                        .size(Val::Px(400.0), Val::Px(40.0))
                        .font_size(26.0)
                        .boxed(),
                    WhTextInput::<()>::default()
                        .background(NodeBackground::Color(Color::WHITE))
                        .size(Val::Px(400.0), Val::Px(40.0))
                        .font_size(26.0)
                        .boxed(),
                    WhDiv::<()>::default()
                        .direction(ElementDirection::Row, Val::Px(5.0))
                        .size(Val::Px(400.0), Val::Px(60.0))
                        .add_children(vec![
                            button::<ConfirmEditServerButton>("images/menu/buttons/confirm.png"),
                            button::<BackToMultiplayerButton>("images/menu/buttons/back.png"),
                        ])
                        .boxed(),
                ])
                .boxed(),
        ])
        .hidden(true)
        .boxed()
}

fn button<Flags>(img: &str) -> BoxedElement
where
    Flags: Bundle + Default,
{
    WhButton::<Flags>::default()
        .background(NodeBackground::Image(img.into()))
        .size(Val::Px(200.0), Val::Px(50.0))
        .boxed()
}

fn server_entry() -> BoxedElement {
    WhDiv::<()>::default()
        .background(NodeBackground::Color(Color::rgba(0.0, 0.0, 0.0, 0.5)))
        .size(Val::Percent(100.0), Val::Px(100.0))
        .padding(UiRect::all(Val::Px(10.0)))
        .direction(ElementDirection::Row, Val::Px(10.0))
        .add_children(vec![
            WhDiv::<()>::default()
                .background(NodeBackground::Image("images/wraithaven.png".into()))
                .size(Val::Auto, Val::Percent(100.0))
                .aspect_ratio(1.0)
                .boxed(),
            WhDiv::<()>::default()
                .direction(ElementDirection::Column, Val::Px(2.0))
                .size(Val::Px(1.0), Val::Percent(100.0))
                .growing()
                .add_children(vec![
                    WhDiv::<()>::default()
                        .direction(ElementDirection::Row, Val::Px(0.0))
                        .justify(ElementJustify::Start)
                        .size(Val::Percent(100.0), Val::Px(30.0))
                        .add_children(vec![
                            WhText::<()>::default()
                                .text("Wraithaven")
                                .font_size(26.0)
                                .text_color(Color::WHITE)
                                .size(Val::Auto, Val::Px(30.0))
                                .growing()
                                .boxed(),
                            WhText::<()>::default()
                                .text("200ms")
                                .font_size(26.0)
                                .text_color(Color::WHITE)
                                .size(Val::Auto, Val::Px(30.0))
                                .justify(ElementJustify::End)
                                .boxed(),
                        ])
                        .boxed(),
                    WhText::<()>::default()
                        .text("wraithaven.com")
                        .font_size(26.0)
                        .text_color(Color::WHITE)
                        .size(Val::Percent(100.0), Val::Px(30.0))
                        .boxed(),
                    WhText::<()>::default()
                        .text("Wraithaven is a server for Wraithaven.")
                        .font_size(26.0)
                        .text_color(Color::WHITE)
                        .size(Val::Percent(100.0), Val::Px(30.0))
                        .boxed(),
                ])
                .boxed(),
        ])
        .boxed()
}
