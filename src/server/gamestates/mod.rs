use bevy::prelude::*;

mod resources;
mod systems;

pub use resources::*;

#[derive(Debug, Default)]
pub struct ServerGameStatePlugin;
impl Plugin for ServerGameStatePlugin {
    fn build(&self, app_: &mut App) {
        app_.add_state::<ServerGameState>()
            .init_resource::<ActiveLoadingSystems>()
            .add_systems(Startup, systems::server_startup)
            .add_systems(
                PostUpdate,
                systems::server_finish_loading.run_if(in_state(ServerGameState::Loading)),
            );
    }
}
