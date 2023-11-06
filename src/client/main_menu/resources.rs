use bevy::prelude::*;

#[derive(Debug, Default, Resource, Clone)]
pub struct MainMenuProperties {
    pub title_screen: Option<TitleScreenProperties>,
    pub single_player_screen: Option<SinglePlayerScreenProperties>,
    pub multiplayer_screen: Option<MultiplayerScreenProperties>,
    pub settings_screen: Option<SettingsScreenProperties>,
    pub credits_screen: Option<CreditsScreenProperties>,
}

#[derive(Debug, Clone)]
pub struct TitleScreenProperties {
    pub bg_img_path: String,
    pub single_player_button: Option<ImageProperties>,
    pub multiplayer_button: Option<ImageProperties>,
    pub settings_button: Option<ImageProperties>,
    pub credits_button: Option<ImageProperties>,
    pub quit_button: Option<ImageProperties>,
}

#[derive(Debug, Clone)]
pub struct SinglePlayerScreenProperties {
    pub bg_img_path: String,
    pub back_button: ImageProperties,
}

#[derive(Debug, Clone)]
pub struct MultiplayerScreenProperties {
    pub bg_img_path: String,
    pub back_button: ImageProperties,
}

#[derive(Debug, Clone)]
pub struct SettingsScreenProperties {
    pub bg_img_path: String,
    pub back_button: ImageProperties,
}

#[derive(Debug, Clone)]
pub struct CreditsScreenProperties {
    pub bg_img_path: String,
    pub back_button: ImageProperties,
}

#[derive(Debug, Clone)]
pub struct ImageProperties {
    pub img_path: String,
    pub img_size: Vec2,
}

#[derive(Debug, Default, States, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MainMenuState {
    #[default]
    TitleScreen,
    SinglePlayerScreen,
    MultiplayerScreen,
    SettingsScreen,
    CreditsScreen,
}
