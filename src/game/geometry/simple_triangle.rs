use bevy::{asset::RenderAssetUsages, prelude::*};
use bevy_mesh::{Indices, PrimitiveTopology};

use crate::game::playing_state::primary_shader::plugin::ATTRIBUTE_EDGE_NEARNESS_UV;

#[allow(unused)]
pub fn create_simple_triangle_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD, // TODO: is this optimal?
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]],
    )
    .with_inserted_attribute(
        ATTRIBUTE_EDGE_NEARNESS_UV,
        vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]],
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2]))
}
