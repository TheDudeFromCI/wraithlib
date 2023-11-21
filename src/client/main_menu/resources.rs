use bevy::prelude::*;

use crate::client::ui::elements::{BoxedElement, WhCanvas};
use crate::common::uuid::Uuid;

pub type ServerEntryBuilder = Box<dyn Fn(Uuid, &str, &str) -> BoxedElement + Send + Sync + 'static>;

#[derive(Default, Resource)]
pub struct MainMenuProperties {
    pub canvas: WhCanvas,
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
