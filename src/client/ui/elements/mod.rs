use bevy::prelude::*;

mod button;
mod canvas;
mod div;
mod screen;
mod scroll_pane;
mod text_input;

pub use button::*;
pub use canvas::*;
pub use div::*;
pub use screen::*;
pub use scroll_pane::*;
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
