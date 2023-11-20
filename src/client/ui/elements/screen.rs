use bevy::prelude::*;

use super::{NodeBackground, WhElement};
use crate::client::assets::AssetLoader;

#[derive(Default)]
pub struct WhScreen<Flags = ()>
where
    Flags: Bundle,
{
    pub flags: Flags,
    pub background: NodeBackground,
    pub hidden: bool,
    pub children: Vec<Box<dyn WhElement>>,
}

impl<Flags> WhElement for WhScreen<Flags>
where
    Flags: Bundle,
{
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &mut AssetLoader,
        parent: Option<Entity>,
    ) {
        let background = self.background.into_image_bundle(loader);

        let display = if self.hidden {
            Display::None
        } else {
            Display::Flex
        };

        let mut cmd = commands.spawn((
            self.flags,
            ImageBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Relative,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display,
                    ..default()
                },
                ..background
            },
        ));

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }

        let id = cmd.id();
        for child in self.children {
            child.build_child(commands, loader, Some(id));
        }
    }
}

impl<Flags> WhScreen<Flags>
where
    Flags: Bundle,
{
    pub fn add_children(mut self, mut children: Vec<Box<dyn WhElement>>) -> Self {
        self.children.append(&mut children);
        self
    }

    pub fn background(mut self, background: NodeBackground) -> Self {
        self.background = background;
        self
    }

    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
}
