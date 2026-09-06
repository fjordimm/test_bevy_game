use bevy::{asset::RenderAssetUsages, prelude::*};
use bevy_mesh::{Indices, PrimitiveTopology};

#[allow(unused)]
#[rustfmt::skip]
pub fn cube_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![
        // Bottom
        2, 0, 3,
        1, 3, 0,
        // Top
        4, 6, 5,
        7, 5, 6,
        // Front
        6, 2, 7,
        3, 7, 2,
        // Back
        5, 1, 4,
        0, 4, 1,
        // Left
        4, 0, 6,
        2, 6, 0,
        // Right
        7, 3, 5,
        1, 5, 3,
    ]))
}
