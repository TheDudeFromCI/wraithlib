use bevy::prelude::*;
use bevy_wh_elements::element::BoxedElement;

use crate::common::uuid::Uuid;

pub type ServerEntryBuilder = fn(Uuid, &str, &str) -> BoxedElement;

#[derive(Default, Resource)]
pub struct MainMenuProperties {
    pub canvas: Option<BoxedElement>,
    pub server_entry: Option<ServerEntryBuilder>,
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

#[derive(Debug, Default, Resource)]
pub struct SelectedServerEntry {
    pub server: Option<Uuid>,
}
