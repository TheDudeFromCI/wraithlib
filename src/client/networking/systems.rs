use std::net::{ToSocketAddrs, UdpSocket};
use std::time::SystemTime;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetClient};

use super::events::*;
use super::resources::*;
use crate::client::gamestates::ClientGameState;
use crate::common::networking::{PacketContainer, PROTOCOL_ID};

pub(super) fn connect_to_server(
    mut events_conn_to_server: EventReader<DoConnectToServer>,
    mut events_failed_to_conn: EventWriter<OnCouldNotConnectToServer>,
    mut next_state: ResMut<NextState<NetworkState>>,
    mut commands: Commands,
) {
    for event in events_conn_to_server.read().take(1) {
        let ip = &event
            .ip
            .to_socket_addrs()
            .ok()
            .map(|mut addrs| addrs.next());

        let Some(Some(addr)) = ip else {
            warn!("Failed to resolve server address: {:?}", event.ip);
            events_failed_to_conn.send(OnCouldNotConnectToServer);
            continue;
        };

        let client = RenetClient::new(ConnectionConfig::default());
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let auth = ClientAuthentication::Unsecure {
            protocol_id: *PROTOCOL_ID,
            client_id: time.as_millis() as u64,
            server_addr: *addr,
            user_data: None,
        };

        let transport = NetcodeClientTransport::new(time, auth, socket).unwrap();

        commands.insert_resource(client);
        commands.insert_resource(transport);

        next_state.set(NetworkState::Connecting);
        debug!("Client connecting to server at {}.", addr);
    }
}

pub(super) fn wait_for_connection(
    client: Res<RenetClient>,
    mut events: EventWriter<OnJoinedServer>,
    mut next_state: ResMut<NextState<NetworkState>>,
    mut game_state: ResMut<NextState<ClientGameState>>,
) {
    if client.is_connected() {
        next_state.set(NetworkState::Connected);
        game_state.set(ClientGameState::BuildingWorld);
        events.send(OnJoinedServer);
        debug!("Client joined server.");
    }
}

pub(super) fn handle_broken_connection(
    current_state: Res<State<NetworkState>>,
    client: Res<RenetClient>,
    mut failed_con_events: EventWriter<OnCouldNotConnectToServer>,
    mut disconnected_events: EventWriter<OnDisconnectedFromServer>,
    mut next_state: ResMut<NextState<NetworkState>>,
    mut game_state: ResMut<NextState<ClientGameState>>,
) {
    if client.is_disconnected() {
        if *current_state == NetworkState::Connecting {
            failed_con_events.send(OnCouldNotConnectToServer);
            debug!("Client failed to connect to server.");
        } else {
            disconnected_events.send(OnDisconnectedFromServer);
            game_state.set(ClientGameState::ClosingWorld);
            debug!("Client disconnected from server.");
        }

        next_state.set(NetworkState::NotConnected);
    }
}

pub(super) fn send_packet(mut client: ResMut<RenetClient>, mut events: EventReader<DoSendPacket>) {
    for ev in events.read() {
        let Some(message) = ev.serialize() else {
            warn!("Failed to serialize packet!");
            continue;
        };
        client.send_message(DefaultChannel::ReliableOrdered, message);
    }
}

pub(super) fn receive_packets(
    mut client: ResMut<RenetClient>,
    mut events: EventWriter<OnReceivePacket>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let Some(packet) = PacketContainer::deserialize(&message) else {
            warn!("Failed to deserialize packet!");
            continue;
        };

        events.send(OnReceivePacket(packet));
    }
}

pub(super) fn close_connection_on_exit(
    mut app_exit_evs: EventReader<AppExit>,
    mut client: ResMut<RenetClient>,
    mut transport: ResMut<NetcodeClientTransport>,
    mut next_state: ResMut<NextState<NetworkState>>,
) {
    if app_exit_evs.read().next().is_none() {
        return;
    }

    client.disconnect();
    transport.disconnect();
    next_state.set(NetworkState::NotConnected);
}
