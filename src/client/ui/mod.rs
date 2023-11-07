use bevy::prelude::*;
use bevy_simple_text_input::TextInputPlugin;
use bevy_tweening::TweeningPlugin;

mod scroll_pane;
mod tweening;

pub use bevy_tweening;
pub use scroll_pane::*;
pub use tweening::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_plugins(TweeningPlugin)
            .add_plugins(TextInputPlugin)
            .add_systems(
                Update,
                (
                    bevy_tweening::component_animator_system::<BackgroundColor>,
                    scroll_pane::mouse_scroll,
                ),
            );
    }
}
