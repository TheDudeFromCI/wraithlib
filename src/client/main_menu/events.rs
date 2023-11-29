use bevy::prelude::*;

use crate::common::uuid::Uuid;

#[derive(Debug, Event, Clone)]
pub struct AddServerEntry {
    pub uuid: Uuid,
    pub name: String,
    pub address: String,
    pub update_database: bool,
}
