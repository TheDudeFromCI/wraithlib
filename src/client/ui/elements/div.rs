use bevy::prelude::*;

use super::{
    BoxedElement,
    ElementDirection,
    ElementJustify,
    NodeBackground,
    NodeBorder,
    WhElement,
};
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
    pub justify: ElementJustify,
    pub gap: Val,
    pub padding: UiRect,
    pub margin: UiRect,
    pub flex_grow: f32,
    pub flex_wrap: bool,
    pub children: Vec<BoxedElement>,
    pub border: NodeBorder,
    pub aspect_ratio: Option<f32>,
}

impl<Flags> WhElement for WhDiv<Flags>
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

        let flex_direction = match self.direction {
            ElementDirection::Row => FlexDirection::Row,
            ElementDirection::Column => FlexDirection::Column,
        };

        let justify_content = match self.justify {
            ElementJustify::Start => JustifyContent::FlexStart,
            ElementJustify::Center => JustifyContent::Center,
            ElementJustify::End => JustifyContent::FlexEnd,
        };

        let flex_wrap = if self.flex_wrap {
            FlexWrap::Wrap
        } else {
            FlexWrap::NoWrap
        };

        let (border_color, border_thickness) = match self.border {
            NodeBorder::None => (Color::NONE, Val::Px(0.0)),
            NodeBorder::Border { color, thickness } => (color, thickness),
        };

        let mut cmd = commands.spawn((
            self.flags,
            ImageBundle {
                style: Style {
                    flex_direction,
                    flex_wrap,
                    justify_content,
                    align_items: AlignItems::Center,
                    flex_grow: self.flex_grow,
                    row_gap: self.gap,
                    column_gap: self.gap,
                    width: self.width,
                    height: self.height,
                    padding: self.padding,
                    margin: self.margin,
                    aspect_ratio: self.aspect_ratio,
                    ..default()
                },
                ..background
            },
            Outline {
                width: border_thickness,
                color: border_color,
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

impl<Flags> WhDiv<Flags>
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

    pub fn background(mut self, background: NodeBackground) -> Self {
        self.background = background;
        self
    }

    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.width = width;
        self.height = height;

        if self.flex_grow > 0.0 {
            if self.width == Val::Auto {
                self.width = Val::Px(1.0);
            }

            if self.height == Val::Auto {
                self.height = Val::Px(1.0);
            }
        }

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

        if self.width == Val::Auto {
            self.width = Val::Px(1.0);
        }

        if self.height == Val::Auto {
            self.height = Val::Px(1.0);
        }

        self
    }

    pub fn wrap_contents(mut self) -> Self {
        self.flex_wrap = true;
        self
    }

    pub fn border(mut self, thickness: Val, color: Color) -> Self {
        self.border = NodeBorder::Border { thickness, color };
        self
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = Some(aspect_ratio);
        self
    }
}
