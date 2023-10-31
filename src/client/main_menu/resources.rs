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
    pub title_screen_img_path: Option<String>,
    pub single_player_img_path: Option<String>,
    pub server_list_img_path: Option<String>,
    pub settings_img_path: Option<String>,
}
