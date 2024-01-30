use std::thread;

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_wh_net::server::{ClientConnection, DoSendPacketToClient, ServerNetworkingPlugin};
use wraithlib::common::WraithLibPlugins;
use wraithlib::server::gamestates::ServerGameState;
use wraithlib::server::ServerPlugins;

use crate::game::RotatePacket;

pub fn run() {
    thread::spawn(|| {
        App::new()
            .add_plugins((
                MinimalPlugins,
                LogPlugin {
                    level: Level::DEBUG,
                    ..default()
                },
            ))
            .add_plugins(WraithLibPlugins)
            .add_plugins(ServerPlugins.set(ServerNetworkingPlugin {
                ip: "127.0.0.1:12345".into(),
                max_clients: 64,
            }))
            .add_systems(
                FixedUpdate,
                update.run_if(in_state(ServerGameState::Online)),
            )
            .run();
    });
}

pub fn update(
    time: Res<Time>,
    query_clients: Query<&ClientConnection>,
    mut do_send_packet_evs: EventWriter<DoSendPacketToClient>,
) {
    let s = time.elapsed_seconds();
    let x = 7.0 * s;
    let y = 11.0 * s;
    let z = 13.0 * s;
    let rotation = Quat::from_euler(EulerRot::YXZ, x, y, z);

    for client in query_clients.iter() {
        do_send_packet_evs.send(DoSendPacketToClient {
            client_id: client.client_id(),
            packet: RotatePacket { rotation }.into(),
        });
    }
}
