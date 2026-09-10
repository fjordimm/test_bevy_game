/* Has the same shape as a `dodec`, but has 5 triangles per face to allow vertex color to work better. */

use bevy::{asset::RenderAssetUsages, prelude::*};
use bevy_mesh::{Indices, PrimitiveTopology};

use crate::game::util::{col_to_array4, lerp_colors};

#[allow(unused)]
pub enum CDodecMeshColors {
    All(Color),
    Layers {
        first: Color,
        second: Color,
        third: Color,
        fourth: Color,
        fifth: Color,
        sixth: Color,
    },
    Gradient {
        bottom: Color,
        top: Color,
    },
}

impl Default for CDodecMeshColors {
    fn default() -> Self {
        CDodecMeshColors::All(Color::WHITE)
    }
}

#[allow(unused)]
#[rustfmt::skip]
pub fn cdodec_mesh(colors: CDodecMeshColors) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // Bottom central vertex
            [0.0, -0.5, 0.0],
            // Bottom pentagon
            [-0.2245139794039736, -0.5, 0.30901704613980713],
            [0.2245139794039736, -0.5, 0.30901704613980713],
            [0.36327124301247765, -0.5, -0.11803409227961423],
            [0.0, -0.5, -0.38196590772038574],
            [-0.36327124301247765, -0.5, -0.11803409227961423],
            // First middle ring of central vertices
            [0.0, -0.22360681845592284, 0.44721363691184574],
            [0.42532538205236625, -0.22360681845592284, 0.13819659077203855],
            [0.2628655416882813, -0.22360681845592284, -0.3618034092279614],
            [-0.2628655416882813, -0.22360681845592284, -0.36180340922796145],
            [-0.42532538205236625, -0.22360681845592284, 0.13819659077203855],
            // First middle ring of corner vertices
            [-0.36327124301247765, -0.11803409227961423, 0.5],
            [0.36327124301247765, -0.11803409227961423, 0.5],
            [0.5877852224164513, -0.11803409227961423, -0.19098295386019287],
            [0.0, -0.11803409227961423, -0.6180340922796143],
            [-0.5877852224164513, -0.11803409227961423, -0.19098295386019287],
            // Second middle ring of corner vertices
            [0.0, 0.11803409227961423, 0.6180340922796143],
            [0.5877852224164513, 0.11803409227961423, 0.19098295386019287],
            [0.36327124301247765, 0.11803409227961423, -0.5],
            [-0.36327124301247765, 0.11803409227961423, -0.5],
            [-0.5877852224164513, 0.11803409227961423, 0.19098295386019287],
            // Second middle ring of central vertices
            [-0.2628655416882813, 0.22360681845592287, 0.3618034092279614],
            [0.2628655416882813, 0.22360681845592287, 0.36180340922796145],
            [0.42532538205236625, 0.22360681845592287, -0.13819659077203855],
            [0.0, 0.22360681845592287, -0.44721363691184574],
            [-0.4253253820523663, 0.22360681845592287, -0.13819659077203855],
            // Top pentagon
            [0.0, 0.5, 0.38196590772038574],
            [0.36327124301247765, 0.5, 0.11803409227961423],
            [0.2245139794039736, 0.5, -0.30901704613980713],
            [-0.2245139794039736, 0.5, -0.30901704613980713],
            [-0.36327124301247765, 0.5, 0.11803409227961423],
            // Top central vertex
            [0.0, 0.5, 0.0]
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_COLOR,
        color_data(colors),
    )
    .with_inserted_indices(Indices::U32(vec![
        // Bottom pentagon
        0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4, 0, 1, 5,
        // First sideways pentagon #1
        6, 1, 2, 6, 11, 1, 6, 16, 11, 6, 12, 16, 6, 2, 12,
        // First sideways pentagon #2
        7, 2, 3, 7, 12, 2, 7, 17, 12, 7, 13, 17, 7, 3, 13,
        // First sideways pentagon #3
        8, 3, 4, 8, 13, 3, 8, 18, 13, 8, 14, 18, 8, 4, 14,
        // First sideways pentagon #4
        9, 4, 5, 9, 14, 4, 9, 19, 14, 9, 15, 19, 9, 5, 15,
        // First sideways pentagon #5
        10, 5, 1, 10, 15, 5, 10, 20, 15, 10, 11, 20, 10, 1, 11,
        // Second sideways pentagon #1
        21, 20, 11, 21, 30, 20, 21, 26, 30, 21, 16, 26, 21, 11, 16,
        // Second sideways pentagon #2
        22, 16, 12, 22, 26, 16, 22, 27, 26, 22, 17, 27, 22, 12, 17,
        // Second sideways pentagon #3
        23, 17, 13, 23, 27, 17, 23, 28, 27, 23, 18, 28, 23, 13, 18,
        // Second sideways pentagon #4
        24, 18, 14, 24, 28, 18, 24, 29, 28, 24, 19, 29, 24, 14, 19,
        // Second sideways pentagon #5
        25, 19, 15, 25, 29, 19, 25, 30, 29, 25, 20, 30, 25, 15, 20,
        // Top pentagon
        31, 30, 26, 31, 26, 27, 31, 27, 28, 31, 28, 29, 31, 29, 30,
    ]))
}

fn color_data(colors: CDodecMeshColors) -> Vec<[f32; 4]> {
    match colors {
        #[rustfmt::skip]
        CDodecMeshColors::All(color) => {
            let color_all = col_to_array4(color);

            vec![
                // Bottom central vertex
                color_all,
                // Bottom pentagon
                color_all, color_all, color_all, color_all, color_all,
                // First middle ring of central vertices
                color_all, color_all, color_all, color_all, color_all,
                // First middle ring of corner vertices
                color_all, color_all, color_all, color_all, color_all,
                // Second middle ring of corner vertices
                color_all, color_all, color_all, color_all, color_all,
                // Second middle ring of central vertices
                color_all, color_all, color_all, color_all, color_all,
                // Top pentagon
                color_all, color_all, color_all, color_all, color_all,
                // Top central vertex
                color_all,
            ]
        }
        #[rustfmt::skip]
        CDodecMeshColors::Layers {
            first,
            second,
            third,
            fourth,
            fifth,
            sixth,
        } => {
            let color_first = col_to_array4(first);
            let color_second = col_to_array4(second);
            let color_third = col_to_array4(third);
            let color_fourth = col_to_array4(fourth);
            let color_fifth = col_to_array4(fifth);
            let color_sixth = col_to_array4(sixth);

            vec![
                // Bottom central vertex
                color_first,
                // Bottom pentagon
                color_first, color_first, color_first, color_first, color_first,
                // First middle ring of central vertices
                color_second, color_second, color_second, color_second, color_second,
                // First middle ring of corner vertices
                color_third, color_third, color_third, color_third, color_third,
                // Second middle ring of corner vertices
                color_fourth, color_fourth, color_fourth, color_fourth, color_fourth,
                // Second middle ring of central vertices
                color_fifth, color_fifth, color_fifth, color_fifth, color_fifth,
                // Top pentagon
                color_sixth, color_sixth, color_sixth, color_sixth, color_sixth,
                // Top central vertex
                color_sixth,
            ]
        }
        #[rustfmt::skip]
        CDodecMeshColors::Gradient { bottom, top } => {
            let color_first = col_to_array4(lerp_colors(bottom, top, 0.0));
            let color_second = col_to_array4(lerp_colors(bottom, top, 0.5 - 0.22360681845592284));
            let color_third = col_to_array4(lerp_colors(bottom, top, 0.5 - 0.11803409227961423));
            let color_fourth = col_to_array4(lerp_colors(bottom, top, 0.5 + 0.11803409227961423));
            let color_fifth = col_to_array4(lerp_colors(bottom, top, 0.5 + 0.22360681845592287));
            let color_sixth = col_to_array4(lerp_colors(bottom, top, 1.0));

            vec![
                // Bottom central vertex
                color_first,
                // Bottom pentagon
                color_first, color_first, color_first, color_first, color_first,
                // First middle ring of central vertices
                color_second, color_second, color_second, color_second, color_second,
                // First middle ring of corner vertices
                color_third, color_third, color_third, color_third, color_third,
                // Second middle ring of corner vertices
                color_fourth, color_fourth, color_fourth, color_fourth, color_fourth,
                // Second middle ring of central vertices
                color_fifth, color_fifth, color_fifth, color_fifth, color_fifth,
                // Top pentagon
                color_sixth, color_sixth, color_sixth, color_sixth, color_sixth,
                // Top central vertex
                color_sixth,
            ]
        }
    }
}
