use bevy::prelude::*;

use super::{ElementAlignment, ElementJustify, NodeText, WhElement};
use crate::client::assets::AssetLoader;

#[derive(Default)]
pub struct WhText<Flags = ()>
where
    Flags: Bundle,
{
    pub flags: Flags,
    pub width: Val,
    pub height: Val,
    pub margin: UiRect,
    pub text: NodeText,
    pub flex_grow: f32,
}

impl<Flags> WhElement for WhText<Flags>
where
    Flags: Bundle,
{
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &mut AssetLoader,
        parent: Option<Entity>,
    ) {
        let text = self.text.into_text_bundle(loader);

        let mut cmd = commands.spawn((
            self.flags,
            TextBundle {
                style: Style {
                    width: self.width,
                    height: self.height,
                    margin: self.margin,
                    flex_grow: self.flex_grow,
                    ..text.style
                },
                ..text
            },
        ));

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }
    }
}

impl<Flags> WhText<Flags>
where
    Flags: Bundle,
{
    pub fn set_flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
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

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    pub fn font(mut self, font: String) -> Self {
        self.text.font = Some(font);
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.text.size = font_size;
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text.text = text.to_string();
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text.color = color;
        self
    }

    pub fn justify(mut self, justify: ElementJustify) -> Self {
        self.text.justify = justify;
        self
    }

    pub fn alignment(mut self, alignment: ElementAlignment) -> Self {
        self.text.alignment = alignment;
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
}
