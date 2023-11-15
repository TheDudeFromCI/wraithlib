use bevy::prelude::*;
use bevy_renet::renet::ClientId;

use crate::common::networking::PacketContainer;

#[derive(Debug, Event)]
pub struct ClientConnectedEvent {
    pub client_id: ClientId,
    pub entity: Entity,
}

#[derive(Debug, Event)]
pub struct ClientDisconnectedEvent {
    pub client_id: ClientId,
    pub entity: Entity,
}

#[derive(Debug, Event)]
pub struct SendPacket {
    pub packet: PacketContainer,
    pub client_id: ClientId,
}

#[derive(Debug, Event)]
pub struct ReceivePacket {
    pub packet: PacketContainer,
    pub client_id: ClientId,
}
