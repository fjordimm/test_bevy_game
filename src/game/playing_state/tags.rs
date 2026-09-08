use bevy::prelude::*;

// Will be despawned when exiting OverallState::Playing.
#[derive(Component)]
pub struct PlayingStateEntity;

#[derive(Component)]
pub struct PrimaryCamera;
