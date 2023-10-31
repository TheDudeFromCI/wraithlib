use std::fmt;

use bevy::prelude::*;
use uuid::Uuid as UuidLib;

use super::EntityPointer;

#[derive(Debug, Component, Clone, PartialEq, Eq, Hash)]
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

    pub fn get_pointer(&self) -> EntityPointer {
        EntityPointer::Uuid(self.clone())
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}
