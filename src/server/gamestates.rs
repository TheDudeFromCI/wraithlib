use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ServerGameState {
    #[default]
    Init,
    Loading,
    Online,
    ShuttingDown,
}
#[derive(Debug, Default)]
pub struct ServerGameStatePlugin;
impl Plugin for ServerGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ServerGameState>()
            .add_systems(Startup, server_startup);
    }
}

fn server_startup(mut state: ResMut<NextState<ServerGameState>>) {
    state.set(ServerGameState::Online);
}
