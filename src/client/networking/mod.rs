use bevy::prelude::*;
use bevy_renet::transport::NetcodeClientPlugin;
use bevy_renet::RenetClientPlugin;

mod events;
mod resources;
mod systems;

pub use events::*;
pub use resources::*;

pub struct ClientNetworkingPlugin;
impl Plugin for ClientNetworkingPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_state::<NetworkState>()
            .add_event::<TryConnectToServerEvent>()
            .add_event::<JoinedServerEvent>()
            .add_event::<DisconnectedFromServerEvent>()
            .add_event::<CouldNotConnectToServerEvent>()
            .add_event::<SendPacket>()
            .add_event::<ReceivePacket>()
            .add_plugins((RenetClientPlugin, NetcodeClientPlugin))
            .add_systems(
                Update,
                (
                    systems::connect_to_server.run_if(in_state(NetworkState::NotConnected)),
                    systems::wait_for_connection.run_if(in_state(NetworkState::Connecting)),
                    systems::handle_broken_connection
                        .run_if(not(in_state(NetworkState::NotConnected))),
                    systems::send_packet.run_if(in_state(NetworkState::Connected)),
                    systems::receive_packets.run_if(in_state(NetworkState::Connected)),
                ),
            )
            .add_systems(
                Last,
                systems::close_connection_on_exit.run_if(in_state(NetworkState::Connected)),
            );
    }
}
