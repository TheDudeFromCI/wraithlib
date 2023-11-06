use bevy::prelude::*;

use crate::common::networking::PacketContainer;

#[derive(Debug, Event)]
pub struct ClientConnectedEvent {
    pub client_id: u64,
    pub entity: Entity,
}

#[derive(Debug, Event)]
pub struct ClientDisconnectedEvent {
    pub client_id: u64,
    pub entity: Entity,
}

#[derive(Debug, Event)]
pub struct SendPacket {
    pub packet: PacketContainer,
    pub client_id: u64,
}

#[derive(Debug, Event)]
pub struct ReceivePacket {
    pub packet: PacketContainer,
    pub client_id: u64,
}
