use bevy::prelude::*;
use bevy_renet::renet::ClientId;

use crate::common::networking::PacketContainer;

#[derive(Debug, Event)]
pub struct OnClientConnected {
    pub client_id: ClientId,
    pub entity: Entity,
}

#[derive(Debug, Event)]
pub struct OnClientDisconnected {
    pub client_id: ClientId,
    pub entity: Entity,
}

#[derive(Debug, Event)]
pub struct DoSendPacket {
    pub packet: PacketContainer,
    pub client_id: ClientId,
}

#[derive(Debug, Event)]
pub struct OnReceivePacket {
    pub packet: PacketContainer,
    pub client_id: ClientId,
}
