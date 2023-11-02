use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainMenuState {
    #[default]
    TitleScreen,
    SinglePlayer,
    ServerList,
    Settings,
}

#[derive(Debug, Default, Resource, Clone)]
pub struct MainMenuProperties {
    pub title_screen: TitleScreenProperties,
    pub single_player_screen: Option<SinglePlayerScreenProperties>,
    pub server_list_screen: Option<ServerListScreenProperties>,
    pub settings_screen: Option<SettingsScreenProperties>,
}

#[derive(Debug, Default, Clone)]
pub struct TitleScreenProperties {
    pub bg_img_path: Option<String>,
    pub single_player_btn: Option<ButtonProperties>,
    pub server_list_btn: Option<ButtonProperties>,
    pub settings_btn: Option<ButtonProperties>,
    pub quit_btn: Option<ButtonProperties>,
}

#[derive(Debug, Default, Clone)]
pub struct SinglePlayerScreenProperties {
    pub bg_img_path: Option<String>,
    pub back_btn: Option<ButtonProperties>,
}

#[derive(Debug, Default, Clone)]
pub struct ServerListScreenProperties {
    pub bg_img_path: Option<String>,
    pub back_btn: Option<ButtonProperties>,
}

#[derive(Debug, Default, Clone)]
pub struct SettingsScreenProperties {
    pub bg_img_path: Option<String>,
    pub back_btn: Option<ButtonProperties>,
}

#[derive(Debug, Clone)]
pub struct ButtonProperties {
    pub img_path: Option<String>,
    pub img_size: Vec2,
    pub img_margin: UiRect,
}

impl Default for ButtonProperties {
    fn default() -> Self {
        Self {
            img_path: None,
            img_size: Vec2::new(0.0, 0.0),
            img_margin: UiRect {
                left: Val::Px(10.0),
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                bottom: Val::Px(10.0),
            },
        }
    }
}
