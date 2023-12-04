use bevy::prelude::*;

mod resources;
mod systems;

pub use resources::*;

#[derive(Debug, Default)]
pub struct ClientGameStatePlugin;
impl Plugin for ClientGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ClientGameState>()
            .add_systems(Startup, systems::client_startup)
            .add_systems(PostUpdate, (systems::open_world, systems::close_world));
    }
}
