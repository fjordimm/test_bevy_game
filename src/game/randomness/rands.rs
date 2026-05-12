use bevy::prelude::*;

#[derive(Resource)]
pub struct GeneralRand(pub Entity);

#[derive(Component)]
pub(super) struct GeneralRandTag;

#[derive(Resource)]
pub struct DumbRand(pub Entity);

#[derive(Component)]
pub(super) struct DumbRandTag;

// pub(super) fn rands() -> Vec<&Resource> {
//     vec![]
// }
