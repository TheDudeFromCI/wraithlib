use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct ClientConnection {
    client_id: u64,
    connected: bool,
}

impl ClientConnection {
    pub fn new(client_id: u64) -> Self {
        Self {
            client_id,
            connected: true,
        }
    }

    pub fn client_id(&self) -> u64 {
        self.client_id
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub(super) fn disconnect(&mut self) {
        self.connected = false;
    }
}
