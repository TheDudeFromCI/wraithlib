use bevy::prelude::*;

use crate::common::uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityPointer {
    Entity(Entity),
    Uuid(Uuid),
    Both(Entity, Uuid),
}

impl EntityPointer {
    pub fn with_entity(self, entity: Entity) -> Self {
        match self {
            Self::Entity(_) => Self::Entity(entity),
            Self::Uuid(uuid) => Self::Both(entity, uuid),
            Self::Both(_, uuid) => Self::Both(entity, uuid),
        }
    }

    pub fn with_uuid(self, uuid: Uuid) -> Self {
        match self {
            Self::Entity(entity) => Self::Both(entity, uuid),
            Self::Uuid(_) => Self::Uuid(uuid),
            Self::Both(entity, _) => Self::Both(entity, uuid),
        }
    }

    pub fn get_entity(&self) -> Option<Entity> {
        match self {
            Self::Entity(entity) => Some(*entity),
            Self::Uuid(_) => None,
            Self::Both(entity, _) => Some(*entity),
        }
    }

    pub fn get_uuid(&self) -> Option<&Uuid> {
        match self {
            Self::Entity(_) => None,
            Self::Uuid(uuid) => Some(uuid),
            Self::Both(_, uuid) => Some(uuid),
        }
    }
}
