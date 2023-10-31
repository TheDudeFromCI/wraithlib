use bevy::prelude::*;

use crate::client::gamestates::ClientGameState;

/// An event that can be triggered by the client in order to begin a transition
/// effect to the main menu. Using this event is the recommended way to trigger
/// a transition. Note that some systems, such as the loading screen, may delay
/// the transition for a small period of time before the transition actually
/// begins.
#[derive(Debug, Event)]
pub struct TransitionToState {
    pub state: ClientGameState,
}
