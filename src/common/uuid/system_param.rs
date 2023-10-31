use bevy::ecs::query::{QueryItem, ROQueryItem, ReadOnlyWorldQuery, WorldQuery};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::common::uuid::{EntityPointer, Uuid};

#[derive(SystemParam)]
pub struct UuidQuery<'w, 's, Q, F = ()>
where
    Q: WorldQuery + 'static,
    F: ReadOnlyWorldQuery + 'static,
{
    query: Query<'w, 's, (Entity, &'static Uuid, Q), F>,
}

impl<'w, 's, 'a, Q, F> UuidQuery<'w, 's, Q, F>
where
    Q: WorldQuery + 'static,
    F: ReadOnlyWorldQuery + 'static,
{
    pub fn get(&'a self, pointer: &EntityPointer) -> Result<ROQueryItem<'_, Q>, UuidQueryError> {
        match &pointer {
            EntityPointer::Entity(id) => {
                let Ok((_, _, q)) = self.query.get(*id) else {
                    return Err(UuidQueryError::NoSuchEntity(*id));
                };
                Ok(q)
            }
            EntityPointer::Uuid(uuid) => {
                let Some((_, _, q)) = self.query.iter().find(|(_, u, _)| *u == uuid) else {
                    return Err(UuidQueryError::NoSuchUuid(uuid.clone()));
                };
                Ok(q)
            }
            EntityPointer::Both(id, _) => {
                let Ok((_, _, q)) = self.query.get(*id) else {
                    return Err(UuidQueryError::NoSuchEntity(*id));
                };
                Ok(q)
            }
        }
    }

    pub fn get_pointer(&'a self, uuid: &Uuid) -> Option<EntityPointer> {
        if let Some((entity, _, _)) = self.query.iter().find(|(_, u, _)| *u == uuid) {
            Some(EntityPointer::Both(entity, uuid.clone()))
        } else {
            None
        }
    }

    pub fn get_mut(
        &'a mut self,
        pointer: &EntityPointer,
    ) -> Result<QueryItem<'_, Q>, UuidQueryError> {
        match &pointer {
            EntityPointer::Entity(id) => {
                let Ok((_, _, q)) = self.query.get_mut(*id) else {
                    return Err(UuidQueryError::NoSuchEntity(*id));
                };
                Ok(q)
            }
            EntityPointer::Uuid(uuid) => {
                let Some((_, _, q)) = self.query.iter_mut().find(|(_, u, _)| *u == uuid) else {
                    return Err(UuidQueryError::NoSuchUuid(uuid.clone()));
                };
                Ok(q)
            }
            EntityPointer::Both(id, _) => {
                let Ok((_, _, q)) = self.query.get_mut(*id) else {
                    return Err(UuidQueryError::NoSuchEntity(*id));
                };
                Ok(q)
            }
        }
    }

    pub fn iter(&'a self) -> impl Iterator<Item = ROQueryItem<'_, Q>> + 'a {
        self.query.iter().map(|(_, _, q)| q)
    }

    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = QueryItem<'_, Q>> + 'a {
        self.query.iter_mut().map(|(_, _, q)| q)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UuidQueryError {
    NoSuchEntity(Entity),
    NoSuchUuid(Uuid),
}
