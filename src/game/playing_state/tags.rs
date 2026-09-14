use bevy::prelude::*;

// Will be despawned when exiting OverallState::Playing.
#[derive(Component, FromTemplate)]
pub struct PlayingStateEntity;

#[derive(Component, FromTemplate)]
pub struct PrimaryCamera;
