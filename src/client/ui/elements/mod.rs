use bevy::prelude::*;
use bevy::text::BreakLineOn;

mod button;
mod canvas;
mod div;
mod screen;
mod scroll_pane;
mod text;
mod text_input;

pub use button::*;
pub use canvas::*;
pub use div::*;
pub use screen::*;
pub use scroll_pane::*;
pub use text::*;
pub use text_input::*;

use crate::client::assets::AssetLoader;

pub trait WhElement {
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &mut AssetLoader,
        parent: Option<Entity>,
    );
}

#[derive(Debug, Default, Clone)]
pub enum NodeBackground {
    #[default]
    None,
    Color(Color),
    Image(String),
}

impl NodeBackground {
    pub fn into_image_bundle(self, loader: &mut AssetLoader) -> ImageBundle {
        match self {
            NodeBackground::None => ImageBundle {
                background_color: Color::NONE.into(),
                ..default()
            },
            NodeBackground::Color(color) => ImageBundle {
                background_color: color.into(),
                ..default()
            },
            NodeBackground::Image(path) => ImageBundle {
                image: loader.load_string(path).into(),
                ..default()
            },
        }
    }

    pub fn into_button_bundle(self, loader: &mut AssetLoader) -> ButtonBundle {
        match self {
            NodeBackground::None => ButtonBundle::default(),
            NodeBackground::Color(color) => ButtonBundle {
                background_color: color.into(),
                ..default()
            },
            NodeBackground::Image(path) => ButtonBundle {
                image: loader.load_string(path).into(),
                ..default()
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementDirection {
    #[default]
    Column,
    Row,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementJustify {
    #[default]
    Center,
    Start,
    End,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementAlignment {
    #[default]
    Center,
    Start,
    End,
}

#[derive(Debug, Clone)]
pub struct NodeText {
    pub font: Option<String>,
    pub size: f32,
    pub color: Color,
    pub alignment: ElementAlignment,
    pub justify: ElementJustify,
    pub text: String,
}

impl Default for NodeText {
    fn default() -> Self {
        Self {
            font: Default::default(),
            size: 20.0,
            color: Color::BLACK,
            alignment: ElementAlignment::Center,
            justify: ElementJustify::Start,
            text: "Text".into(),
        }
    }
}

impl NodeText {
    pub fn into_text_bundle(self, loader: &mut AssetLoader) -> TextBundle {
        let alignment = match self.alignment {
            ElementAlignment::Center => AlignContent::Center,
            ElementAlignment::Start => AlignContent::FlexStart,
            ElementAlignment::End => AlignContent::FlexEnd,
        };

        let justify = match self.justify {
            ElementJustify::Center => TextAlignment::Center,
            ElementJustify::Start => TextAlignment::Left,
            ElementJustify::End => TextAlignment::Right,
        };

        let font = match self.font {
            Some(path) => loader.load_string(path),
            None => Default::default(),
        };

        TextBundle {
            style: Style {
                align_content: alignment,
                ..default()
            },
            text: Text {
                linebreak_behavior: BreakLineOn::WordBoundary,
                alignment: justify,
                sections: vec![TextSection {
                    value: self.text,
                    style: TextStyle {
                        font,
                        font_size: self.size,
                        color: self.color,
                    },
                }],
            },
            ..default()
        }
    }
}

pub type BoxedElement = Box<dyn WhElement + Send + Sync>;

pub trait BoxableElement {
    fn boxed(self) -> BoxedElement;
}

impl<T> BoxableElement for T
where
    T: WhElement + Send + Sync + 'static,
{
    fn boxed(self) -> BoxedElement {
        Box::new(self)
    }
}
