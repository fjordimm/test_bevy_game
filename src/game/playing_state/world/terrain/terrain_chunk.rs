use bevy::{asset::RenderAssetUsages, prelude::*};
use bevy_mesh::{Indices, PrimitiveTopology};

use crate::game::{
    graphics::primary_shader::plugin::{
        PrimaryShaderMaterial, PrimaryShaderMaterialProps, primary_shader_material,
    },
    playing_state::{
        sets::DuringPlayingUnpaused,
        tags::PlayingStateEntity,
        world::terrain::{plugin::TheTerrainFunc, terrain_func::TerrainFunc},
    },
};

pub struct TerrainChunkPlugin;

impl Plugin for TerrainChunkPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_message::<SpawnTerrainChunk>()
            .add_systems(Update,
                handle_spawn_terrain_chunk
                    .in_set(DuringPlayingUnpaused::General)
            )
        ;
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

#[derive(Message)]
pub struct SpawnTerrainChunk;

fn handle_spawn_terrain_chunk(
    mut commands: Commands,
    mut messages: MessageReader<SpawnTerrainChunk>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PrimaryShaderMaterial>>,
    terrain_func: Res<TheTerrainFunc>,
) {
    messages.read().for_each(|_| {
        commands.spawn((
            PlayingStateEntity,
            Mesh3d(meshes.add(generate_meshes(&terrain_func.0).1)),
            // TODO: reuse a material for all terrain chunks
            MeshMaterial3d(
                materials.add(primary_shader_material(PrimaryShaderMaterialProps {
                    texturing_scale: 1.,
                })),
            ),
            Transform::default(),
            TerrainChunk {},
        ));
    });
}

#[derive(Component)]
pub struct TerrainChunk {}

const CW: usize = 4; // Chunk Width (and height).

// Generates two meshes: 1) the inner mesh, 2) the outer mesh, which together make up a CWxCW grid of squares.
// The outer mesh is just the outermost squares, and the inner mesh is the full CWxCW grid minus the outer mesh squares.
// Each square has four corner vertices, plus one in the middle, and has four triangles connecting them all.
fn generate_meshes(terrain_func: &TerrainFunc) -> (Mesh, Mesh) {
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
            let rf = r as f32;
            let cf = c as f32;

            let h: f32 = terrain_func.at(rf, cf);

            // Inner vertices.
            if r > 0 && r < CW && c > 0 && c < CW {
                inner_positions.push([rf, h, cf]);
                inner_colors.push([1., 1., 1., 1.]);

                inner_indices_c[r][c] = inner_vc;
                inner_vc += 1;
            }

            // Outer vertices.
            if r <= 1 || r >= CW - 1 || c <= 1 || c >= CW - 1 {
                outer_positions.push([rf, h, cf]);
                outer_colors.push([1., 1., 1., 1.]);

                outer_indices_c[r][c] = outer_vc;
                outer_vc += 1;
            }
        }
    }

    // Middle vertices.
    for r in 0..CW {
        for c in 0..CW {
            let rf = 0.5 + r as f32;
            let cf = 0.5 + c as f32;

            let h: f32 = terrain_func.at(rf, cf);

            // Inner vertices.
            if r > 0 && r < CW - 1 && c > 0 && c < CW - 1 {
                inner_positions.push([rf, h, cf]);
                inner_colors.push([1., 1., 1., 1.]);

                inner_indices_m[r][c] = inner_vc;

                inner_vc += 1;
            }

            // Outer vertices.
            if r == 0 || r == CW - 1 || c == 0 || c == CW - 1 {
                outer_positions.push([rf, h, cf]);
                outer_colors.push([1., 1., 1., 1.]);

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
