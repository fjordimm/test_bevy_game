use bevy::{math::DVec3, prelude::*};

pub mod plugin;

#[derive(Resource)]
pub struct CoordRebasingOrigin(pub DVec3);

#[derive(Component)]
pub struct WorldSpaceEntity;

pub fn world_space_transf(t: Transform) -> impl Bundle {
    (WorldSpaceEntity, t)
}
