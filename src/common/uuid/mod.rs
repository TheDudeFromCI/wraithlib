use bevy::prelude::*;

mod components;
mod pointer;
mod system_param;

pub use components::*;
pub use pointer::*;
pub use system_param::*;

pub struct UuidPlugin;
impl Plugin for UuidPlugin {
    fn build(&self, _app: &mut App) {}
}
