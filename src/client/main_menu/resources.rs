use bevy::prelude::*;
use bevy_wh_elements::element::BoxedElement;

use super::ServerListEntry;
use crate::common::uuid::Uuid;

pub type MainMenuBuilder = fn() -> BoxedElement;
pub type ServerEntryBuilder = fn(Uuid, ServerListEntry) -> BoxedElement;

#[derive(Default, Resource)]
pub struct MainMenuProperties {
    pub canvas_builder: Option<MainMenuBuilder>,
    pub server_entry_builder: Option<ServerEntryBuilder>,
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
