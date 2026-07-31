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
            Mesh3d(meshes.add(generate_mesh(&terrain_func.0))),
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

const N: u32 = 16; // TODOr

fn generate_mesh(terrain_func: &TerrainFunc) -> Mesh {
    let mut positions = Vec::<[f32; 3]>::with_capacity(((N + 1) * (N + 1) + N * N) as usize);
    let mut colors = Vec::<[f32; 4]>::with_capacity(((N + 1) * (N + 1) + N * N) as usize);
    let mut indices = Vec::<u32>::with_capacity((4 * N) as usize);

    for r in 0..=N {
        for c in 0..=N {
            let h: f32 = terrain_func.at(r as f32, c as f32);

            positions.push([r as f32, h, c as f32]);
            colors.push([1., 1., 1., 1.]);
        }
    }

    for r in 0..N {
        for c in 0..N {
            let h: f32 = terrain_func.at(0.5 + r as f32, 0.5 + c as f32);

            positions.push([0.5 + r as f32, h, 0.5 + c as f32]);
            colors.push([1., 1., 1., 1.]);
        }
    }

    for r in 0..N {
        for c in 0..N {
            let center = index_from_coords_c(r, c);
            let tl = index_from_coords_m(r, c);
            let tr = index_from_coords_m(r, c + 1);
            let bl = index_from_coords_m(r + 1, c);
            let br = index_from_coords_m(r + 1, c + 1);

            indices.extend_from_slice(&[tr, tl, center]);
            indices.extend_from_slice(&[br, tr, center]);
            indices.extend_from_slice(&[bl, br, center]);
            indices.extend_from_slice(&[tl, bl, center]);
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_indices(Indices::U32(indices))
}

// For the main vertices.
fn index_from_coords_m(r: u32, c: u32) -> u32 {
    c * (N + 1) + r
}

// For the center vertices.
fn index_from_coords_c(r: u32, c: u32) -> u32 {
    (N + 1) * (N + 1) + c * N + r
}
