use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct MainMenuCanvas;

#[derive(Debug, Default, Component)]
pub struct ServerListPane;

#[derive(Debug, Default, Component, Clone)]
pub struct ServerListEntry {
    pub ip: String,
    pub name: String,
    pub description: String,
    pub ping: u32,
    pub icon: Option<UiImage>,
}

#[derive(Debug, Default, Component)]
pub struct ConfirmEditServerButton;

#[derive(Debug, Default, Component)]
pub struct ServerNameTextInput;

#[derive(Debug, Default, Component)]
pub struct ServerAddressTextInput;

#[derive(Debug, Default, Component)]
pub struct JoinServerButton;
