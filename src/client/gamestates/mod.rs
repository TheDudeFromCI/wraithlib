use bevy::prelude::*;

mod resources;
mod systems;

pub use resources::*;

#[derive(Debug, Default)]
pub struct ClientGameStatePlugin;
impl Plugin for ClientGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ClientGameState>()
            .init_resource::<ActiveDownloadingSystems>()
            .add_systems(Startup, systems::client_startup)
            .add_systems(
                PostUpdate,
                (
                    systems::finish_downloading_connection
                        .run_if(in_state(ClientGameState::Downloading)),
                    systems::finish_disconnecting.run_if(in_state(ClientGameState::Disconnecting)),
                ),
            );
    }
}
