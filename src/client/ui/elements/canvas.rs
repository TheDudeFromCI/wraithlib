use bevy::prelude::*;

use super::{BoxableElement, BoxedElement, WhElement};
use crate::client::assets::AssetLoader;

#[derive(Default)]
pub struct WhCanvas<Flags = ()>
where
    Flags: Bundle,
{
    pub flags: Flags,
    pub children: Vec<BoxedElement>,
}

impl<Flags> WhElement for WhCanvas<Flags>
where
    Flags: Bundle,
{
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &mut AssetLoader,
        parent: Option<Entity>,
    ) {
        let mut cmd = commands.spawn((
            self.flags,
            ImageBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
        ));

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }

        let id = cmd.id();
        for child in self.children.into_iter() {
            child.build_child(commands, loader, Some(id));
        }
    }
}

impl<Flags> WhCanvas<Flags>
where
    Flags: Bundle,
{
    pub fn set_flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    pub fn add_children(mut self, mut children: Vec<BoxedElement>) -> Self {
        self.children.append(&mut children);
        self
    }

    pub fn build(self, commands: &mut Commands, loader: &mut AssetLoader) {
        self.boxed().build_child(commands, loader, None);
    }
}
