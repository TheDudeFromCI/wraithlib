use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClientGameState {
    #[default]
    Init,
    Splash,
    MainMenu,
    Connecting,
    Online,
}

#[derive(Debug, Default)]
pub struct ClientGameStatePlugin;
impl Plugin for ClientGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ClientGameState>()
            .add_systems(Startup, client_startup);
    }
}

fn client_startup(mut state: ResMut<NextState<ClientGameState>>) {
    state.set(ClientGameState::Splash);
}
