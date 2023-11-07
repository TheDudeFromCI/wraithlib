use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Delay, EaseFunction, Lens, Sequence, Tween, TweeningDirection};

#[derive(Debug, Clone)]
pub struct BackgroundColorLens {
    pub start: Color,
    pub end: Color,
}

impl Lens<BackgroundColor> for BackgroundColorLens {
    fn lerp(&mut self, target: &mut BackgroundColor, ratio: f32) {
        let a = [ratio, ratio, ratio, ratio];
        let b = [1.0 - ratio, 1.0 - ratio, 1.0 - ratio, 1.0 - ratio];
        let color = self.start * b + self.end * a;
        *target = color.into();
    }
}

pub struct FadeInOut;
impl FadeInOut {
    pub fn from<L, T>(
        fade_in_duration: f32,
        display_duration: f32,
        fade_out_duration: f32,
        lens: L,
    ) -> Sequence<T>
    where
        L: Lens<T> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        let fade_in = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs_f32(fade_in_duration),
            lens.clone(),
        );

        let display = Delay::new(Duration::from_secs_f32(display_duration));

        let mut fade_out = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs_f32(fade_out_duration),
            lens,
        );
        fade_out.set_direction(TweeningDirection::Backward);

        Sequence::with_capacity(3)
            .then(fade_in)
            .then(display)
            .then(fade_out)
    }
}
