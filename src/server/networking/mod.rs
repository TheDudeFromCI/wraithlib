use std::net::{ToSocketAddrs, UdpSocket};
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{ConnectionConfig, RenetServer};
use bevy_renet::transport::NetcodeServerPlugin;
use bevy_renet::RenetServerPlugin;

use crate::common::networking::PROTOCOL_ID;

mod components;
mod events;
mod systems;

pub use components::*;
pub use events::*;

use super::gamestates::ServerGameState;

pub struct ServerNetworkingPlugin {
    pub ip: String,
    pub max_clients: usize,
}

impl Default for ServerNetworkingPlugin {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1:8123".into(),
            max_clients: 64,
        }
    }
}

impl Plugin for ServerNetworkingPlugin {
    fn build(&self, app_: &mut App) {
        let (server, transport) = build_socket(&self.ip, self.max_clients);

        app_.insert_resource(server)
            .insert_resource(transport)
            .add_event::<ClientConnectedEvent>()
            .add_event::<ClientDisconnectedEvent>()
            .add_event::<SendPacket>()
            .add_event::<ReceivePacket>()
            .add_plugins((RenetServerPlugin, NetcodeServerPlugin))
            .add_systems(
                Update,
                (
                    systems::server_event_handler,
                    systems::error_handling,
                    systems::send_packet,
                    systems::receive_packets,
                ),
            )
            .add_systems(
                Last,
                systems::close_connections_on_exit.run_if(in_state(ServerGameState::Online)),
            );
    }
}

fn build_socket(ip: &str, max_clients: usize) -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(ConnectionConfig::default());
    let public_addr = ip.to_socket_addrs().unwrap().next().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let protocol_id = *PROTOCOL_ID;

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let authentication = ServerAuthentication::Unsecure;

    let config = ServerConfig {
        max_clients,
        protocol_id,
        authentication,
        current_time,
        public_addresses: vec![public_addr],
    };

    let transport = NetcodeServerTransport::new(config, socket).unwrap();

    (server, transport)
}
