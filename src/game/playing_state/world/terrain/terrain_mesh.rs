use bevy::{asset::RenderAssetUsages, prelude::*};
use bevy_mesh::{Indices, PrimitiveTopology};

use crate::game::playing_state::world::terrain::{plugin::CW, terrain_func::TerrainFunc};

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

// TODOr
const TEMP_VERTEX_COLOR1: [f32; 4] = [0., 0., 1., 1.];
const TEMP_VERTEX_COLOR2: [f32; 4] = [0., 0.5, 0., 1.];
fn temp_vertex_color(scale: f32, off_x: f32, off_z: f32) -> [f32; 4] {
    let mut off_x_i = (off_x / (scale * CW as f32) - 0.5).round() as i32;
    let mut off_z_i = (off_z / (scale * CW as f32) - 0.5).round() as i32;

    if off_x_i < 0 {
        off_x_i += 1;
    }
    if off_z_i < 0 {
        off_z_i += 1;
    }

    if ((off_x_i + off_z_i) % 2) == 0 {
        TEMP_VERTEX_COLOR1
    } else {
        TEMP_VERTEX_COLOR2
    }
}

// Generates two meshes: 1) the inner mesh, 2) the outer mesh (perimeter), which together make up a CWxCW grid of squares.
// The outer mesh is just the outermost squares, and the inner mesh is the full CWxCW grid minus the outer mesh squares.
// Each square has four corner vertices, plus one in the middle, and has four triangles connecting them all.
pub(super) fn create_terrain_meshes(
    terrain_func: &TerrainFunc,
    scale: f32,
    off_x: f32,
    off_z: f32,
) -> (Mesh, Mesh) {
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

    (inner_mesh, outer_mesh)
}
