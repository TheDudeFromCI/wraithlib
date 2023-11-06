use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct OpenTitleScreenEvent;

#[derive(Debug, Event)]
pub struct OpenSinglePlayerScreenEvent;

#[derive(Debug, Event)]
pub struct OpenMultiplayerScreenEvent;

#[derive(Debug, Event)]
pub struct OpenSettingsScreenEvent;

#[derive(Debug, Event)]
pub struct OpenCreditsScreenEvent;
