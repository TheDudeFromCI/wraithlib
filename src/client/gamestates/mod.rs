use bevy::prelude::*;

mod resources;
mod systems;

pub use resources::*;

#[derive(Debug, Default)]
pub struct ClientGameStatePlugin;
impl Plugin for ClientGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ClientGameState>()
            .init_resource::<ActiveWorldBuildingSystems>()
            .init_resource::<ActiveWorldClosingSystems>()
            .add_systems(Startup, systems::client_startup)
            .add_systems(
                PostUpdate,
                (
                    systems::finish_building_world.run_if(in_state(ClientGameState::BuildingWorld)),
                    systems::finish_closing_world.run_if(in_state(ClientGameState::ClosingWorld)),
                ),
            );
    }
}
