use bevy::prelude::*;

use crate::common::networking::PacketContainer;

#[derive(Debug, Event)]
pub struct TryConnectToServerEvent {
    pub ip: String,
}

#[derive(Debug, Event)]
pub struct JoinedServerEvent;

#[derive(Debug, Event)]
pub struct DisconnectedFromServerEvent;

#[derive(Debug, Event)]
pub struct CouldNotConnectToServerEvent;

#[derive(Debug, Event, Deref)]
pub struct SendPacket(pub PacketContainer);

#[derive(Debug, Event, Deref)]
pub struct ReceivePacket(pub PacketContainer);
