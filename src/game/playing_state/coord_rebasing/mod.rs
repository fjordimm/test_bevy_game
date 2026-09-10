use bevy::{math::DVec3, prelude::*};

pub mod plugin;

/// WARNING: The inner value is left public, but do not modify it.
#[derive(Resource)]
pub struct CoordRebasingOrigin(pub DVec3);

#[derive(Component)]
pub struct WorldSpaceEntity;

/// Input should be in transform space rather than world space.
pub fn world_space_transf(t: Transform) -> impl Bundle {
    (WorldSpaceEntity, t)
}

pub fn to_world_space(inp: Vec3, coord_rebasing_origin: &CoordRebasingOrigin) -> DVec3 {
    inp.as_dvec3() + coord_rebasing_origin.0
}

pub fn to_transf_space(inp: DVec3, coord_rebasing_origin: &CoordRebasingOrigin) -> Vec3 {
    (inp - coord_rebasing_origin.0).as_vec3()
}
