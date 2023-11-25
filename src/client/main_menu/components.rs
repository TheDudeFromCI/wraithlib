use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct MainMenuCanvas;

#[derive(Debug, Default, Component)]
pub struct ServerListPane;

#[derive(Debug, Default, Component)]
pub struct ServerListEntry;

#[derive(Debug, Default, Component)]
pub struct ConfirmEditServerButton;

#[derive(Debug, Default, Component)]
pub struct ServerNameTextInput;

#[derive(Debug, Default, Component)]
pub struct ServerAddressTextInput;
