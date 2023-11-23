use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;
use bevy_wh_elements::WhElementsPlugin;

mod tweening;

pub use bevy_tweening;
pub use tweening::*;

pub struct WhUiPlugin;
impl Plugin for WhUiPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_plugins(TweeningPlugin)
            .add_plugins(WhElementsPlugin)
            .add_systems(
                Update,
                bevy_tweening::component_animator_system::<BackgroundColor>,
            );
    }
}
