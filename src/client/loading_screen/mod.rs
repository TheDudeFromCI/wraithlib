use bevy::prelude::*;

mod components;
mod conditions;
mod events;
mod resources;
mod systems;

pub use components::*;
pub use conditions::*;
pub use events::*;
pub use resources::*;

#[derive(Debug, Default)]
pub struct LoadingScreenPlugin {
    pub properties: LoadingScreenProperties,
}

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LoadingState>()
            .insert_resource(self.properties.clone())
            .init_resource::<ActiveLoadingScreen>()
            .add_event::<TransitionToState>()
            .add_systems(Startup, systems::preload_loading_img)
            .add_systems(
                Update,
                (
                    systems::trigger_transition_to_state.run_if(in_state(LoadingState::None)),
                    systems::set_finishing_state
                        .run_if(in_state(LoadingState::Loading))
                        .run_if(condition_is_done_loading),
                    systems::wait_for_fade_out.run_if(in_state(LoadingState::StartingLoad)),
                    systems::wait_for_fade_in.run_if(in_state(LoadingState::FinishingLoad)),
                ),
            )
            .add_systems(
                OnEnter(LoadingState::StartingLoad),
                systems::build_loading_screen,
            )
            .add_systems(
                OnEnter(LoadingState::Loading),
                systems::apply_transition_state,
            )
            .add_systems(
                OnEnter(LoadingState::FinishingLoad),
                systems::close_loading_screen,
            )
            .add_systems(
                OnExit(LoadingState::FinishingLoad),
                systems::clear_loading_screen,
            );
    }
}
