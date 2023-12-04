use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClientGameState {
    #[default]
    Init,
    Splash,
    MainMenu,
    Online,
}
