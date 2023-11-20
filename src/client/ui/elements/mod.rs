use bevy::prelude::*;

mod button;
mod canvas;
mod div;
mod screen;

pub use button::*;
pub use canvas::*;
pub use div::*;
pub use screen::*;

use crate::client::assets::AssetLoader;

pub trait WhElement {
    fn build(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &mut AssetLoader,
        parent: Option<Entity>,
    );

    fn children(&self) -> &[Box<dyn WhElement>] {
        &[]
    }
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

pub trait BoxedElement {
    fn boxed(self) -> Box<dyn WhElement>;
}

impl<T> BoxedElement for T
where
    T: WhElement + 'static,
{
    fn boxed(self) -> Box<dyn WhElement> {
        Box::new(self)
    }
}
