use bevy::prelude::*;

use crate::client::ui::elements::WhCanvas;

#[derive(Default, Resource)]
pub struct MainMenuProperties {
    pub canvas: WhCanvas,
}

#[derive(Debug, Default, States, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MainMenuState {
    #[default]
    TitleScreen,
    SinglePlayerScreen,
    MultiplayerScreen,
    SettingsScreen,
    CreditsScreen,
    EditServerScreen,
}
