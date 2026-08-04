use bevy::{asset::RenderAssetUsages, prelude::*};
use bevy_mesh::{Indices, PrimitiveTopology, VertexAttributeValues};

use crate::game::{
    playing_state::world::terrain::{plugin::CW, terrain_func::TerrainFunc},
    util::alrms,
};

// TODOr
fn temp_vertex_color(scale: f32, off_x: f32, off_z: f32) -> [f32; 4] {
    let pre_checkerboard_color = Color::hsv((scale.log2().abs() * 222.) % 360., 1., 1.);

    let mut off_x_i = (off_x / (scale * CW as f32) - 0.5).round() as i32;
    let mut off_z_i = (off_z / (scale * CW as f32) - 0.5).round() as i32;

    if off_x_i < 0 {
        off_x_i += 1;
    }
    if off_z_i < 0 {
        off_z_i += 1;
    }

    let color = if ((off_x_i + off_z_i) % 2) == 0 {
        Color::hsv(pre_checkerboard_color.hue(), 0.9, 1.)
    } else {
        Color::hsv(pre_checkerboard_color.hue(), 1., 0.75)
    };

    let color = color.to_srgba();
    [color.red, color.green, color.blue, 1.]
}

// Generates two meshes: 1) the inner mesh, 2) the outer mesh (perimeter), which together make up a CWxCW grid of squares,
//   and 3) a vec of vecs of positions for the outermost vertices for connecting with different lods.
// The outer mesh is just the outermost squares, and the inner mesh is the full CWxCW grid minus the outer mesh squares.
// Each square has four corner vertices, plus one in the middle, and has four triangles connecting them all.
// The 3rd field of the return value (the vec of vecs of positions) has positions in the order: north edge, east edge, south edge, west edge,
//   where the north and south edges include the corners but the east and west don't.
pub(super) fn create_terrain_meshes(
    terrain_func: &TerrainFunc,
    scale: f32,
    off_x: f32,
    off_z: f32,
    base_lod: usize,
) -> (Mesh, Mesh, Vec<Vec<[f32; 3]>>) {
    let mut inner_positions =
        Vec::<[f32; 3]>::with_capacity((CW - 1) * (CW - 1) + (CW - 2) * (CW - 2));
    let mut inner_colors =
        Vec::<[f32; 4]>::with_capacity((CW - 1) * (CW - 1) + (CW - 2) * (CW - 2));
    let mut inner_triangles = Vec::<u32>::with_capacity(4 * (CW - 2));

    let mut outer_positions = Vec::<[f32; 3]>::with_capacity(4 * CW + 4 * (CW - 2) + 4 * (CW - 1));
    let mut outer_colors = Vec::<[f32; 4]>::with_capacity(4 * CW + 4 * (CW - 2) + 4 * (CW - 1));
    let mut outer_triangles = Vec::<u32>::with_capacity(4 * 4 * (CW - 1));

    // To keep track of the index of (Corner) vertices given a 2D index.
    let mut inner_indices_c = [[0u32; CW + 1]; CW + 1];
    let mut outer_indices_c = [[0u32; CW + 1]; CW + 1];

    // To keep track of the index of (Middle) vertices given a 2D index.
    let mut inner_indices_m = [[0u32; CW]; CW];
    let mut outer_indices_m = [[0u32; CW]; CW];

    // Vertex counters.
    let mut inner_vc: u32 = 0;
    let mut outer_vc: u32 = 0;

    // Corner vertices.
    for r in 0..=CW {
        for c in 0..=CW {
            let rf = scale * r as f32;
            let cf = scale * c as f32;

            let h: f32 = terrain_func.at(rf + off_x, cf + off_z);

            // Inner vertices.
            if r > 0 && r < CW && c > 0 && c < CW {
                inner_positions.push([rf, h, cf]);
                inner_colors.push(temp_vertex_color(scale, off_x, off_z));

                inner_indices_c[r][c] = inner_vc;
                inner_vc += 1;
            }

            // Outer vertices.
            if r <= 1 || r >= CW - 1 || c <= 1 || c >= CW - 1 {
                outer_positions.push([rf, h, cf]);
                outer_colors.push(temp_vertex_color(scale, off_x, off_z));

                outer_indices_c[r][c] = outer_vc;
                outer_vc += 1;
            }
        }
    }

    // Middle vertices.
    for r in 0..CW {
        for c in 0..CW {
            let rf = scale * (0.5 + r as f32);
            let cf = scale * (0.5 + c as f32);

            let h: f32 = terrain_func.at(rf + off_x, cf + off_z);

            // Inner vertices.
            if r > 0 && r < CW - 1 && c > 0 && c < CW - 1 {
                inner_positions.push([rf, h, cf]);
                inner_colors.push(temp_vertex_color(scale, off_x, off_z));

                inner_indices_m[r][c] = inner_vc;

                inner_vc += 1;
            }

            // Outer vertices.
            if r == 0 || r == CW - 1 || c == 0 || c == CW - 1 {
                outer_positions.push([rf, h, cf]);
                outer_colors.push(temp_vertex_color(scale, off_x, off_z));

                outer_indices_m[r][c] = outer_vc;

                outer_vc += 1;
            }
        }
    }

    for r in 0..CW {
        for c in 0..CW {
            // Inner triangles.
            if r > 0 && r < CW - 1 && c > 0 && c < CW - 1 {
                let center = inner_indices_m[r][c];
                let tl = inner_indices_c[r][c];
                let tr = inner_indices_c[r + 1][c];
                let bl = inner_indices_c[r][c + 1];
                let br = inner_indices_c[r + 1][c + 1];

                inner_triangles.extend_from_slice(&[tr, tl, center]);
                inner_triangles.extend_from_slice(&[br, tr, center]);
                inner_triangles.extend_from_slice(&[bl, br, center]);
                inner_triangles.extend_from_slice(&[tl, bl, center]);
            }

            // Outer triangles.
            if r == 0 || r == CW - 1 || c == 0 || c == CW - 1 {
                let center = outer_indices_m[r][c];
                let tl = outer_indices_c[r][c];
                let tr = outer_indices_c[r + 1][c];
                let bl = outer_indices_c[r][c + 1];
                let br = outer_indices_c[r + 1][c + 1];

                outer_triangles.extend_from_slice(&[tr, tl, center]);
                outer_triangles.extend_from_slice(&[br, tr, center]);
                outer_triangles.extend_from_slice(&[bl, br, center]);
                outer_triangles.extend_from_slice(&[tl, bl, center]);
            }
        }
    }

    let mut lod_connecting_perimeters =
        vec![
            Vec::<[f32; 3]>::with_capacity((CW + 1) + (CW - 1) + (CW + 1) + (CW - 1));
            base_lod + 1
        ];
    for i in (0..=base_lod).rev() {
        // North.
        for j in 0..=CW {
            lod_connecting_perimeters[i].push(quantized_position(
                terrain_func,
                scale,
                off_x,
                off_z,
                j,
                0,
                true,
                0,
            ));
        }

        // East.
        for j in 1..CW {
            lod_connecting_perimeters[i].push(quantized_position(
                terrain_func,
                scale,
                off_x,
                off_z,
                CW,
                j,
                true,
                0,
            ));
        }

        // South.
        for j in 0..=CW {
            lod_connecting_perimeters[i].push(quantized_position(
                terrain_func,
                scale,
                off_x,
                off_z,
                j,
                CW,
                true,
                0,
            ));
        }

        // West.
        for j in 1..CW {
            lod_connecting_perimeters[i].push(quantized_position(
                terrain_func,
                scale,
                off_x,
                off_z,
                0,
                j,
                true,
                0,
            ));
        }
    }

    let inner_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, inner_positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, inner_colors)
    .with_inserted_indices(Indices::U32(inner_triangles));

    let outer_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, outer_positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, outer_colors)
    .with_inserted_indices(Indices::U32(outer_triangles));

    (inner_mesh, outer_mesh, lod_connecting_perimeters)
}

fn quantized_position(
    terrain_func: &TerrainFunc,
    scale: f32,
    off_x: f32,
    off_z: f32,
    r: usize,
    c: usize,
    quantization_direction_is_ns: bool,
    quantization_factor: usize,
) -> [f32; 3] {
    let rf = scale * r as f32;
    let cf = scale * c as f32;

    let h: f32 = terrain_func.at(rf + off_x, cf + off_z);

    [rf, h + 1., cf]

    // // If East-West.
    // if !quantization_direction_is_ns {

    // }
    // // If North-South.
    // else {
    // }

    // let rf_q0 = scale * ((r / quantization_factor) * quantization_factor) as f32;
    // let rf_q1 = scale * ((r / quantization_factor + 1) * quantization_factor) as f32;

    // let cf_q0 = scale * ((c / quantization_factor) * quantization_factor) as f32;
    // let cf_q1 = scale * ((c / quantization_factor + 1) * quantization_factor) as f32;

    // let rf = scale * r as f32;
    // let cf = scale * c as f32;

    // let h: f32 = terrain_func.at(rf + off_x, cf + off_z);
}

pub(super) fn change_mesh_from_perim_lod_positions(
    mesh: &mut Mesh,
    perim_lod_positions: &Vec<[f32; 3]>,
) {
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        alrms!(mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION))
    {
        positions[0][1] += 0.1;
    } else {
        error!("Positions attribute was not in an expected form.");
    }
}

// TODOr
// fn todor1(mesh_q: Query<&Mesh3d, With<TerrainChunk>>, mut meshes: ResMut<Assets<Mesh>>) {
//     mesh_q.iter().for_each(|mesh_handle| {
//         if let Some(mesh) = alrms!(meshes.get_mut(mesh_handle.0.id())) {
//             if let Some(VertexAttributeValues::Float32x3(positions)) =
//                 alrms!(mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION))
//             {
//                 positions[0][1] += 0.1;
//             } else {
//                 error!("Positions attribute was not in an expected form.");
//             }
//         }
//     });
// }
