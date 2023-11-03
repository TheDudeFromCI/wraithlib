use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct MainMenuProperties {
    pub root_screen: MenuScreenProperties,
    pub child_screens: Vec<MenuScreenProperties>,
}

#[derive(Default)]
pub struct MenuScreenProperties {
    pub bg_img_path: Option<String>,
    pub buttons: Vec<ButtonProperties>,
    pub screen_comp: Option<fn(&mut EntityCommands)>,
}

pub struct ButtonProperties {
    pub img_path: Option<String>,
    pub img_size: Vec2,
    pub img_margin: UiRect,
    pub button_comp: Option<fn(&mut EntityCommands)>,
}

impl Default for ButtonProperties {
    fn default() -> Self {
        Self {
            img_path: None,
            img_size: Vec2::new(0.0, 0.0),
            img_margin: UiRect {
                left: Val::Px(10.0),
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                bottom: Val::Px(10.0),
            },
            button_comp: None,
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct MainMenuScreenLerp {
    start_time: f32,
    end_time: f32,
    lerp: f32,
    invert: bool,
}

impl MainMenuScreenLerp {
    pub fn start(&mut self, time: f32, invert: bool) {
        self.start_time = time;
        self.end_time = time + 0.5;
        self.invert = invert;
        self.lerp = if invert { 1.0 } else { 0.0 };
    }

    pub fn update(&mut self, time: f32) {
        if self.start_time == self.end_time {
            return;
        }

        self.lerp = (time - self.start_time) / (self.end_time - self.start_time);
        self.lerp = self.lerp.clamp(0.0, 1.0);

        if self.invert {
            self.lerp = 1.0 - self.lerp;
        }
    }

    pub fn get_lerp(&self) -> f32 {
        self.lerp
    }
}

#[derive(Debug, Default, Resource)]
pub struct MainMenuActiveScreen {
    pub screen_index: usize,
}
