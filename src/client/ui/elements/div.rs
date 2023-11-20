use bevy::prelude::*;

use super::{ElementDirection, NodeBackground, WhElement};
use crate::client::assets::AssetLoader;

#[derive(Default)]
pub struct WhDiv<Flags>
where
    Flags: Bundle,
{
    pub flags: Flags,
    pub background: NodeBackground,
    pub width: Val,
    pub height: Val,
    pub direction: ElementDirection,
    pub gap: Val,
    pub children: Vec<Box<dyn WhElement>>,
}

impl<Flags> WhElement for WhDiv<Flags>
where
    Flags: Bundle,
{
    fn build(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &mut AssetLoader,
        parent: Option<Entity>,
    ) {
        let background = self.background.into_image_bundle(loader);

        let flex_direction = match self.direction {
            ElementDirection::Row => FlexDirection::Row,
            ElementDirection::Column => FlexDirection::Column,
        };

        let mut cmd = commands.spawn((
            self.flags,
            ImageBundle {
                style: Style {
                    flex_direction,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: self.gap,
                    column_gap: self.gap,
                    width: self.width,
                    height: self.height,
                    ..default()
                },
                ..background
            },
        ));

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }

        let id = cmd.id();
        for child in self.children.into_iter() {
            child.build(commands, loader, Some(id));
        }
    }

    fn children(&self) -> &[Box<dyn WhElement>] {
        &self.children
    }
}

impl<Flags> WhDiv<Flags>
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

    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn direction(mut self, direction: ElementDirection, gap: Val) -> Self {
        self.direction = direction;
        self.gap = gap;
        self
    }
}
