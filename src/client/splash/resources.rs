use bevy::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct SplashImages {
    pub images: Vec<SplashImageProperties>,
    pub start_delay: f32,
    pub end_delay: f32,
    pub delay_between: f32,
}

#[derive(Debug, Clone)]
pub struct SplashImageProperties {
    pub path: String,
    pub scale: f32,
    pub fade_in_duration: f32,
    pub display_duration: f32,
    pub fade_out_duration: f32,
}

impl Default for SplashImageProperties {
    fn default() -> Self {
        Self {
            path: "splash.png".into(),
            scale: 0.75,
            fade_in_duration: 0.5,
            display_duration: 1.5,
            fade_out_duration: 0.5,
        }
    }
}
