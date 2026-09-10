use bevy::{asset::RenderAssetUsages, prelude::*};
use bevy_mesh::{Indices, PrimitiveTopology};

use crate::game::util::col_to_array4;

#[allow(unused)]
pub enum CubeMeshColors {
    All(Color),
}

impl Default for CubeMeshColors {
    fn default() -> Self {
        CubeMeshColors::All(Color::WHITE)
    }
}

#[allow(unused)]
#[rustfmt::skip]
pub fn cube_mesh(colors: CubeMeshColors) -> Mesh {
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
        color_data(colors),
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

fn color_data(colors: CubeMeshColors) -> Vec<[f32; 4]> {
    match colors {
        #[rustfmt::skip]
        CubeMeshColors::All(color) => {
            let color_all = col_to_array4(color);

            vec![
                color_all,
                color_all,
                color_all,
                color_all,
                color_all,
                color_all,
                color_all,
                color_all,
            ]
        }
    }
}
