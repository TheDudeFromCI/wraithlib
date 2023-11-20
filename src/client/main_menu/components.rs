use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct MainMenuScreen;

#[derive(Debug, Default, Component)]
pub struct TitleScreen;

#[derive(Debug, Default, Component)]
pub struct SinglePlayerScreen;

#[derive(Debug, Default, Component)]
pub struct MultiplayerScreen;

#[derive(Debug, Default, Component)]
pub struct SettingsScreen;

#[derive(Debug, Default, Component)]
pub struct CreditsScreen;

#[derive(Debug, Default, Component)]
pub struct EditServerScreen;

#[derive(Debug, Default, Component)]
pub struct ServerListPane;

#[derive(Debug, Default, Component)]
pub struct ServerListEntry;

#[derive(Debug, Default, Component)]
pub struct SinglePlayerButton;

#[derive(Debug, Default, Component)]
pub struct MultiplayerButton;

#[derive(Debug, Default, Component)]
pub struct SettingsButton;

#[derive(Debug, Default, Component)]
pub struct CreditsButton;

#[derive(Debug, Default, Component)]
pub struct QuitButton;

#[derive(Debug, Default, Component)]
pub struct BackToTitleScreenButton;

#[derive(Debug, Default, Component)]
pub struct NewGameButton;

#[derive(Debug, Default, Component)]
pub struct LoadGameButton;

#[derive(Debug, Default, Component)]
pub struct AddServerButton;

#[derive(Debug, Default, Component)]
pub struct ConfirmEditServerButton;

#[derive(Debug, Default, Component)]
pub struct BackToMultiplayerButton;
