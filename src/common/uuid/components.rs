use std::fmt;

use bevy::prelude::*;
use uuid::Uuid as UuidLib;

use super::EntityPointer;

#[derive(Debug, Default, Component, Clone, PartialEq, Eq, Hash)]
pub struct Uuid {
    id: UuidLib,
}

impl Uuid {
    pub fn from_random() -> Self {
        Self {
            id: UuidLib::new_v4(),
        }
    }

    pub fn from_string<S>(s: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            id: UuidLib::parse_str(s.as_ref()).unwrap(),
        }
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self {
            id: UuidLib::from_bytes(bytes),
        }
    }

    pub fn get_pointer(&self) -> EntityPointer {
        EntityPointer::Uuid(self.clone())
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        self.id.as_bytes()
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}
