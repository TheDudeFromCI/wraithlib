use bevy::prelude::*;

use super::{NodeBackground, WhElement};
use crate::client::assets::AssetLoader;
use crate::client::ui::TextInput;

pub struct WhTextInput<Flags>
where
    Flags: Bundle,
{
    pub flags: Flags,
    pub background: NodeBackground,
    pub width: Val,
    pub height: Val,
    pub padding: UiRect,
    pub margin: UiRect,
    pub font_size: f32,
    pub font_color: Color,
}

impl<Flags> Default for WhTextInput<Flags>
where
    Flags: Bundle + Default,
{
    fn default() -> Self {
        Self {
            flags: Flags::default(),
            background: NodeBackground::default(),
            width: Val::default(),
            height: Val::default(),
            padding: UiRect::default(),
            margin: UiRect::default(),
            font_size: 20.0,
            font_color: Color::BLACK,
        }
    }
}

impl<Flags> WhElement for WhTextInput<Flags>
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

        let mut cmd = commands.spawn((
            self.flags,
            ImageBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_content: AlignContent::Center,
                    width: self.width,
                    height: self.height,
                    padding: self.padding,
                    margin: self.margin,
                    ..default()
                },
                ..background
            },
            TextInput {
                text_style: TextStyle {
                    font_size: self.font_size,
                    color: self.font_color,
                    ..default()
                },
                inactive: true,
            },
        ));

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }
    }
}

impl<Flags> WhTextInput<Flags>
where
    Flags: Bundle,
{
    pub fn set_flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
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

    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn font_color(mut self, font_color: Color) -> Self {
        self.font_color = font_color;
        self
    }
}
