use bevy::prelude::*;

use super::{BoxedElement, ElementDirection, ElementJustify, NodeBackground, WhElement};
use crate::client::assets::AssetLoader;
use crate::client::ui::ScrollPane;

#[derive(Default)]
pub struct WhScrollPane<OuterFlags, InnerFlags>
where
    OuterFlags: Bundle,
    InnerFlags: Bundle,
{
    pub outer_flags: OuterFlags,
    pub inner_flags: InnerFlags,
    pub background: NodeBackground,
    pub outer_width: Val,
    pub outer_height: Val,
    pub inner_width: Val,
    pub inner_height: Val,
    pub direction: ElementDirection,
    pub justify: ElementJustify,
    pub gap: Val,
    pub padding: UiRect,
    pub margin: UiRect,
    pub flex_grow: f32,
    pub children: Vec<BoxedElement>,
}

impl<OuterFlags, InnerFlags> WhElement for WhScrollPane<OuterFlags, InnerFlags>
where
    OuterFlags: Bundle,
    InnerFlags: Bundle,
{
    fn build_child(
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

        let justify_content = match self.justify {
            ElementJustify::Start => JustifyContent::FlexStart,
            ElementJustify::Center => JustifyContent::Center,
            ElementJustify::End => JustifyContent::FlexEnd,
        };

        let mut cmd = commands.spawn((
            self.outer_flags,
            ImageBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    flex_grow: self.flex_grow,
                    width: self.outer_width,
                    height: self.outer_height,
                    margin: self.margin,
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                ..background
            },
        ));

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }
        let outer_id = cmd.id();

        let inner_id = commands
            .spawn((
                self.inner_flags,
                ScrollPane::default(),
                NodeBundle {
                    style: Style {
                        flex_direction,
                        justify_content,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Stretch,
                        row_gap: self.gap,
                        column_gap: self.gap,
                        width: self.inner_width,
                        height: self.inner_height,
                        padding: self.padding,
                        ..default()
                    },
                    ..default()
                },
            ))
            .set_parent(outer_id)
            .id();

        for child in self.children.into_iter() {
            child.build_child(commands, loader, Some(inner_id));
        }
    }
}

impl<OuterFlags, InnerFlags> WhScrollPane<OuterFlags, InnerFlags>
where
    OuterFlags: Bundle,
    InnerFlags: Bundle,
{
    pub fn set_outer_flags(mut self, flags: OuterFlags) -> Self {
        self.outer_flags = flags;
        self
    }

    pub fn set_inner_flags(mut self, flags: InnerFlags) -> Self {
        self.inner_flags = flags;
        self
    }

    pub fn add_children(mut self, mut children: Vec<BoxedElement>) -> Self {
        self.children.append(&mut children);
        self
    }

    pub fn background(mut self, background: NodeBackground) -> Self {
        self.background = background;
        self
    }

    pub fn outer_size(mut self, width: Val, height: Val) -> Self {
        self.outer_width = width;
        self.outer_height = height;
        self
    }

    pub fn inner_size(mut self, width: Val, height: Val) -> Self {
        self.inner_width = width;
        self.inner_height = height;
        self
    }

    pub fn direction(mut self, direction: ElementDirection, gap: Val) -> Self {
        self.direction = direction;
        self.gap = gap;
        self
    }

    pub fn justify(mut self, justify: ElementJustify) -> Self {
        self.justify = justify;
        self
    }

    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    pub fn growing(mut self) -> Self {
        self.flex_grow = 1.0;
        self
    }
}
