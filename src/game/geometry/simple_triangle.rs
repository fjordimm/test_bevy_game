use bevy::{asset::RenderAssetUsages, prelude::*};
use bevy_mesh::{Indices, PrimitiveTopology};

use crate::game::playing_state::primary_shader::plugin::ATTRIBUTE_POLYGON_UV;

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
        ATTRIBUTE_POLYGON_UV,
        vec![
            [210.0f32.to_radians().cos(), 210.0f32.to_radians().sin()],
            [330.0f32.to_radians().cos(), 330.0f32.to_radians().sin()],
            [90.0f32.to_radians().cos(), 90.0f32.to_radians().sin()],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2]))
}
