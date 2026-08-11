use bevy::{asset::RenderAssetUsages, prelude::*};
use bevy_mesh::{Indices, PrimitiveTopology, VertexAttributeValues};

use crate::game::{
    playing_state::terrain::{plugin::CW, terrain_func::TerrainFunc},
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

// Generates three things: 1) the inner mesh, 2) the outer mesh (perimeter), which together with the inner mesh make up a CWxCW grid of squares,
//   and 3) a vec of vecs of positions for the outermost vertices for connecting with different lods.
// The outer mesh is just the outermost squares, and the inner mesh is the full CWxCW grid minus the outer mesh squares.
// Each square has four corner vertices, plus one in the middle, and has four triangles connecting them all.
// The 3rd field of the return value (the vec of vecs of positions) has positions in the order: north edge, east edge, south edge, west edge,
//   where the north and south edges include the corners but the east and west don't.
pub(super) fn create_terrain_mesh(
    terrain_func: &TerrainFunc,
    scale: f32,
    off_x: i64,
    off_z: i64,
    lod: usize,
) -> (Mesh, Mesh, Vec<Vec<[f32; 3]>>) {
    let off_x_real = CW as f32 * scale * off_x as f32;
    let off_z_real = CW as f32 * scale * off_z as f32;

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

    // Corner vertices (inner mesh only).
    for c in 1..CW {
        for r in 1..CW {
            let cf = scale * c as f32;
            let rf = scale * r as f32;

            let h: f32 = terrain_func.at(cf + off_x_real, rf + off_z_real);

            inner_positions.push([cf, h, rf]);
            inner_colors.push(temp_vertex_color(scale, off_x_real, off_z_real));

            inner_indices_c[c][r] = inner_vc;
            inner_vc += 1;
        }
    }

    // Corner vertices (outer mesh only).
    // These are in the order that they are to make `change_mesh_from_perim_lod_vertices` easier to write.
    {
        // The outermost corner vertices.

        // North.
        for i in 0..=CW {
            let c = i;
            let r = 0;

            let cf = scale * c as f32;
            let rf = scale * r as f32;

            let h: f32 = terrain_func.at(cf + off_x_real, rf + off_z_real);

            outer_positions.push([cf, h, rf]);
            outer_colors.push(temp_vertex_color(scale, off_x_real, off_z_real));

            outer_indices_c[c][r] = outer_vc;
            outer_vc += 1;
        }

        // East.
        for i in 1..CW {
            let c = CW;
            let r = i;

            let cf = scale * c as f32;
            let rf = scale * r as f32;

            let h: f32 = terrain_func.at(cf + off_x_real, rf + off_z_real);

            outer_positions.push([cf, h, rf]);
            outer_colors.push(temp_vertex_color(scale, off_x_real, off_z_real));

            outer_indices_c[c][r] = outer_vc;
            outer_vc += 1;
        }

        // South.
        for i in 0..=CW {
            let c = i;
            let r = CW;

            let cf = scale * c as f32;
            let rf = scale * r as f32;

            let h: f32 = terrain_func.at(cf + off_x_real, rf + off_z_real);

            outer_positions.push([cf, h, rf]);
            outer_colors.push(temp_vertex_color(scale, off_x_real, off_z_real));

            outer_indices_c[c][r] = outer_vc;
            outer_vc += 1;
        }

        // West.
        for i in 1..CW {
            let c = 0;
            let r = i;

            let cf = scale * c as f32;
            let rf = scale * r as f32;

            let h: f32 = terrain_func.at(cf + off_x_real, rf + off_z_real);

            outer_positions.push([cf, h, rf]);
            outer_colors.push(temp_vertex_color(scale, off_x_real, off_z_real));

            outer_indices_c[c][r] = outer_vc;
            outer_vc += 1;
        }

        // Second-to-outermost corner vertices.

        // North.
        for i in 1..CW {
            let c = i;
            let r = 1;

            let cf = scale * c as f32;
            let rf = scale * r as f32;

            let h: f32 = terrain_func.at(cf + off_x_real, rf + off_z_real);

            outer_positions.push([cf, h, rf]);
            outer_colors.push(temp_vertex_color(scale, off_x_real, off_z_real));

            outer_indices_c[c][r] = outer_vc;
            outer_vc += 1;
        }

        // East.
        for i in 2..CW - 1 {
            let c = CW - 1;
            let r = i;

            let cf = scale * c as f32;
            let rf = scale * r as f32;

            let h: f32 = terrain_func.at(cf + off_x_real, rf + off_z_real);

            outer_positions.push([cf, h, rf]);
            outer_colors.push(temp_vertex_color(scale, off_x_real, off_z_real));

            outer_indices_c[c][r] = outer_vc;
            outer_vc += 1;
        }

        // South.
        for i in 1..CW {
            let c = i;
            let r = CW - 1;

            let cf = scale * c as f32;
            let rf = scale * r as f32;

            let h: f32 = terrain_func.at(cf + off_x_real, rf + off_z_real);

            outer_positions.push([cf, h, rf]);
            outer_colors.push(temp_vertex_color(scale, off_x_real, off_z_real));

            outer_indices_c[c][r] = outer_vc;
            outer_vc += 1;
        }

        // West.
        for i in 2..CW - 1 {
            let c = 1;
            let r = i;

            let cf = scale * c as f32;
            let rf = scale * r as f32;

            let h: f32 = terrain_func.at(cf + off_x_real, rf + off_z_real);

            outer_positions.push([cf, h, rf]);
            outer_colors.push(temp_vertex_color(scale, off_x_real, off_z_real));

            outer_indices_c[c][r] = outer_vc;
            outer_vc += 1;
        }
    }

    // Middle vertices.
    for c in 0..CW {
        for r in 0..CW {
            let cf = scale * (0.5 + c as f32);
            let rf = scale * (0.5 + r as f32);

            let h: f32 = terrain_func.at(cf + off_x_real, rf + off_z_real);

            // Inner vertices.
            if c > 0 && c < CW - 1 && r > 0 && r < CW - 1 {
                inner_positions.push([cf, h, rf]);
                inner_colors.push(temp_vertex_color(scale, off_x_real, off_z_real));

                inner_indices_m[c][r] = inner_vc;

                inner_vc += 1;
            }

            // Outer vertices.
            if c == 0 || c == CW - 1 || r == 0 || r == CW - 1 {
                outer_positions.push([cf, h, rf]);
                outer_colors.push(temp_vertex_color(scale, off_x_real, off_z_real));

                outer_indices_m[c][r] = outer_vc;

                outer_vc += 1;
            }
        }
    }

    for c in 0..CW {
        for r in 0..CW {
            // Inner triangles.
            if c > 0 && c < CW - 1 && r > 0 && r < CW - 1 {
                let center = inner_indices_m[c][r];
                let tl = inner_indices_c[c][r];
                let tr = inner_indices_c[c + 1][r];
                let bl = inner_indices_c[c][r + 1];
                let br = inner_indices_c[c + 1][r + 1];

                inner_triangles.extend_from_slice(&[tr, tl, center]);
                inner_triangles.extend_from_slice(&[br, tr, center]);
                inner_triangles.extend_from_slice(&[bl, br, center]);
                inner_triangles.extend_from_slice(&[tl, bl, center]);
            }

            // Outer triangles.
            if c == 0 || c == CW - 1 || r == 0 || r == CW - 1 {
                let center = outer_indices_m[c][r];
                let tl = outer_indices_c[c][r];
                let tr = outer_indices_c[c + 1][r];
                let bl = outer_indices_c[c][r + 1];
                let br = outer_indices_c[c + 1][r + 1];

                outer_triangles.extend_from_slice(&[tr, tl, center]);
                outer_triangles.extend_from_slice(&[br, tr, center]);
                outer_triangles.extend_from_slice(&[bl, br, center]);
                outer_triangles.extend_from_slice(&[tl, bl, center]);
            }
        }
    }

    let mut lod_connecting_perimeters =
        vec![Vec::<[f32; 3]>::with_capacity((CW + 1) + (CW - 1) + (CW + 1) + (CW - 1)); lod + 1];
    for i in (0..=lod).rev() {
        let quantization_factor = 2i64.pow((lod - i) as u32);

        // North.
        for j in 0..=CW {
            lod_connecting_perimeters[i].push(quantized_position(
                terrain_func,
                quantization_factor,
                scale,
                off_x,
                off_z,
                off_x_real,
                off_z_real,
                false,
                j,
                0,
            ));
        }

        // East.
        for j in 1..CW {
            lod_connecting_perimeters[i].push(quantized_position(
                terrain_func,
                quantization_factor,
                scale,
                off_x,
                off_z,
                off_x_real,
                off_z_real,
                true,
                CW,
                j,
            ));
        }

        // South.
        for j in 0..=CW {
            lod_connecting_perimeters[i].push(quantized_position(
                terrain_func,
                quantization_factor,
                scale,
                off_x,
                off_z,
                off_x_real,
                off_z_real,
                false,
                j,
                CW,
            ));
        }

        // West.
        for j in 1..CW {
            lod_connecting_perimeters[i].push(quantized_position(
                terrain_func,
                quantization_factor,
                scale,
                off_x,
                off_z,
                off_x_real,
                off_z_real,
                true,
                0,
                j,
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
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, outer_positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, outer_colors)
    .with_inserted_indices(Indices::U32(outer_triangles));

    (inner_mesh, outer_mesh, lod_connecting_perimeters)
}

fn quantized_position(
    terrain_func: &TerrainFunc,
    quantization_factor: i64,
    scale: f32,
    off_x: i64,
    off_z: i64,
    off_x_real: f32,
    off_z_real: f32,
    quantization_direction_is_ns: bool,
    c: usize,
    r: usize,
) -> [f32; 3] {
    // If North-South.
    if quantization_direction_is_ns {
        let ro = r as i64 + (off_z * CW as i64);
        let roq = if ro.is_negative() {
            ((ro + 1) / quantization_factor - 1) * quantization_factor
        } else {
            (ro / quantization_factor) * quantization_factor
        };
        let rq = roq - (off_z * CW as i64);

        let rf_q0 = scale * (rq) as f32;
        let rf_q1 = scale * (rq + quantization_factor) as f32;
        let lerp_pos = ((ro - roq) as f32) / (quantization_factor as f32);

        let cf = scale * c as f32;
        let rf = scale * r as f32;

        let h_q0 = terrain_func.at(cf + off_x_real, rf_q0 + off_z_real);
        let h_q1 = terrain_func.at(cf + off_x_real, rf_q1 + off_z_real);
        let h = (1. - lerp_pos) * h_q0 + (lerp_pos) * h_q1;

        [cf, h, rf]
    }
    // If East-West.
    else {
        let co = c as i64 + (off_x * CW as i64);
        let coq = if co.is_negative() {
            ((co + 1) / quantization_factor - 1) * quantization_factor
        } else {
            (co / quantization_factor) * quantization_factor
        };
        let cq = coq - (off_x * CW as i64);

        let cf_q0 = scale * (cq) as f32;
        let cf_q1 = scale * (cq + quantization_factor) as f32;
        let lerp_pos = ((co - coq) as f32) / (quantization_factor as f32);

        let cf = scale * c as f32;
        let rf = scale * r as f32;

        let h_q0 = terrain_func.at(cf_q0 + off_x_real, rf + off_z_real);
        let h_q1 = terrain_func.at(cf_q1 + off_x_real, rf + off_z_real);
        let h = (1. - lerp_pos) * h_q0 + (lerp_pos) * h_q1;

        [cf, h, rf]
    }
}

pub(super) fn change_mesh_from_perim_lod_vertices(
    mesh: &mut Mesh,
    perim_lod_vertices: &Vec<Vec<[f32; 3]>>,
    north_lod: usize,
    east_lod: usize,
    south_lod: usize,
    west_lod: usize,
) {
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        alrms!(mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION))
    {
        // North.
        for i in 0..=CW {
            positions[i] = perim_lod_vertices[north_lod][i];
        }

        // East.
        for i in 0..CW - 1 {
            let offset_index = (CW + 1) + i;
            positions[offset_index] = perim_lod_vertices[east_lod][offset_index];
        }

        // South.
        for i in 0..=CW {
            let offset_index = (CW + 1) + (CW - 1) + i;
            positions[offset_index] = perim_lod_vertices[south_lod][offset_index];
        }

        // West.
        for i in 0..CW - 1 {
            let offset_index = (CW + 1) + (CW - 1) + (CW + 1) + i;
            positions[offset_index] = perim_lod_vertices[west_lod][offset_index];
        }
    } else {
        error!("Positions attribute was not in an expected form.");
    }
}
