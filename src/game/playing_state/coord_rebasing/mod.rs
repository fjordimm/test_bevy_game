use crate::game::playing_state::coord_rebasing::plugin::WorldSpaceEntity;
use bevy::prelude::*;

pub mod plugin;
pub mod resources;

pub fn world_space_transf(t: Transform) -> impl Bundle {
    (WorldSpaceEntity, t)
}
