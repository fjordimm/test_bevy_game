use std::{collections::HashMap, hash::Hash, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::game::{
    core::states::OverallState,
    graphics::primary_shader::plugin::PrimaryShaderMaterial,
    playing_state::{
        player::tags::PlayerBody,
        reusable_materials::ReusableMaterials,
        sets::{DuringPlaying, OnEnterPlaying},
        tags::PlayingStateEntity,
        world::terrain::{
            resources::TheTerrainFunc, terrain_func::TerrainFunc,
            terrain_mesh::create_terrain_meshes,
        },
    },
    random::{Prng, rands::GeneralRand},
    util::{alrmo, alrrs},
};

const UPDATE_CHUNKS_INTERVAL: u64 = 2000; // In milliseconds.

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_message::<GenerateMeshes>()
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(OnEnterPlaying::ResourceSetup)
            )
            .add_systems(Update,
                handle_generate_meshes
                    .in_set(DuringPlaying)
            )
            .add_systems(Update,
                update_chunks
                    .in_set(DuringPlaying)
                    .run_if(on_timer(Duration::from_millis(UPDATE_CHUNKS_INTERVAL)))
            )
        ;
    }
}

fn on_enter(mut commands: Commands, mut prng: Single<&mut Prng, With<GeneralRand>>) {
    commands.insert_resource(TheTerrainFunc(TerrainFunc::new(&mut prng)));
    commands.insert_resource(L0ChunkDict(HashMap::new()));
}

pub(super) const CW: usize = 16; // Chunk Width (and height). Minimum value: 3 (because of the perimeter).
const L0_CHUNK_SCALE: f32 = 25.;
const L0_RENDER_DIST: i32 = 2;
const MAX_LOD: i32 = 2;

fn chunk_bundle(
    material: Handle<PrimaryShaderMaterial>,
    lod: i32,
    scale: f32,
    off_x: i32,
    off_z: i32,
) -> impl Bundle {
    (
        PlayingStateEntity,
        MeshMaterial3d(material),
        Transform::from_xyz(
            scale * CW as f32 * off_x as f32,
            0.,
            scale * CW as f32 * off_z as f32,
        ),
        TerrainChunk {
            lod,
            scale: scale,
            off_x: off_x,
            off_z: off_z,
            has_generated_mesh: false,
            perimeter_entity: Entity::PLACEHOLDER,
            subchunk_tl: Entity::PLACEHOLDER,
            subchunk_tr: Entity::PLACEHOLDER,
            subchunk_bl: Entity::PLACEHOLDER,
            subchunk_br: Entity::PLACEHOLDER,
        },
        Visibility::Visible,
    )
}

#[derive(Component)]
struct TerrainChunk {
    lod: i32,
    scale: f32,
    off_x: i32,
    off_z: i32,
    has_generated_mesh: bool,
    perimeter_entity: Entity, // The child entity that has the perimeter mesh.
    subchunk_tl: Entity,
    subchunk_tr: Entity,
    subchunk_bl: Entity,
    subchunk_br: Entity,
}

#[derive(Component)]
struct TerrainChunkPerimeter;

impl TerrainChunk {
    fn generate_mesh_nonredundantly(
        &mut self,
        gm_messages: &mut MessageWriter<GenerateMeshes>,
        entity: &Entity,
    ) {
        if !self.has_generated_mesh {
            gm_messages.write(GenerateMeshes(*entity));
        }
    }
}

#[derive(Message)]
struct GenerateMeshes(Entity);

fn handle_generate_meshes(
    mut commands: Commands,
    mut messages: MessageReader<GenerateMeshes>,
    mut chunk_q: Query<&mut TerrainChunk>,
    terrain_func: Res<TheTerrainFunc>,
    mut meshes: ResMut<Assets<Mesh>>,
    reusable_materials: Res<ReusableMaterials>,
) {
    messages.read().for_each(|msg| {
        if let Some(mut chunk) = alrmo!(chunk_q.get_mut(msg.0)) {
            let (prim_mesh, perim_mesh) = create_terrain_meshes(
                &terrain_func.0,
                chunk.scale,
                chunk.scale * CW as f32 * chunk.off_x as f32,
                chunk.scale * CW as f32 * chunk.off_z as f32,
            );

            commands.entity(msg.0).insert(Mesh3d(meshes.add(prim_mesh)));

            let perimeter = commands
                .spawn((
                    PlayingStateEntity,
                    TerrainChunkPerimeter,
                    MeshMaterial3d(reusable_materials.terrain.clone()),
                    Mesh3d(meshes.add(perim_mesh)),
                ))
                .id();
            commands.entity(msg.0).add_child(perimeter);
            chunk.perimeter_entity = perimeter;

            chunk.has_generated_mesh = true;
        }
    });
}

#[derive(PartialEq, Eq, Hash)]
struct ChunkKey {
    x: i32,
    z: i32,
}

impl ChunkKey {
    fn new(x: i32, z: i32) -> Self {
        Self { x: x, z: z }
    }
}

#[derive(Resource)]
struct L0ChunkDict(HashMap<ChunkKey, Entity>);

fn update_chunks(
    mut commands: Commands,
    player_q: Option<Single<&Transform, With<PlayerBody>>>,
    mut l0_chunk_dict: ResMut<L0ChunkDict>,
    mut chunk_q: Query<(Entity, &mut TerrainChunk, &mut Visibility)>,
    reusable_materials: Res<ReusableMaterials>,
    mut gm_messages: MessageWriter<GenerateMeshes>,
) {
    chunk_q
        .iter_mut()
        .for_each(|(entity, mut tc, mut visibility)| {
            // Generate meshes for chunks that don't already have them (but only if they were left visible).
            if let Visibility::Visible = *visibility {
                tc.generate_mesh_nonredundantly(&mut gm_messages, &entity);
            }

            // Set all chunks to be hidden. The rest of this function will set the appropriate ones to be visible.
            *visibility = Visibility::Hidden;
        });

    let player_tran = alrrs!(player_q);
    let x_center: i32 =
        (player_tran.translation.x / (L0_CHUNK_SCALE * CW as f32) - 0.5).round() as i32;
    let z_center: i32 =
        (player_tran.translation.z / (L0_CHUNK_SCALE * CW as f32) - 0.5).round() as i32;

    // Spirals out from (x_center, z_center), covering a square which goes L0_RENDER_DIST in each direction.
    let mut x = x_center;
    let mut z = z_center;
    let mut dx = 0;
    let mut dz = 1;
    let mut side_len = 1;
    let mut side_countdown = 1;
    'spiral: loop {
        if x < x_center - L0_RENDER_DIST
            || x > x_center + L0_RENDER_DIST
            || z < z_center - L0_RENDER_DIST
            || z > z_center + L0_RENDER_DIST
        {
            break 'spiral;
        }

        // Actual code using (x, z) outside of the spiral logic.
        {
            if let Some(l0_chunk_entity) = l0_chunk_dict.0.get(&ChunkKey::new(x, z)) {
                update_chunk_and_subchunks(
                    &mut commands,
                    &mut chunk_q,
                    &reusable_materials,
                    *l0_chunk_entity,
                );
            } else {
                let entity = commands
                    .spawn(chunk_bundle(
                        reusable_materials.terrain.clone(),
                        0,
                        L0_CHUNK_SCALE,
                        x,
                        z,
                    ))
                    .id();

                l0_chunk_dict.0.insert(ChunkKey::new(x, z), entity);
            }
        }

        x += dx;
        z += dz;
        side_countdown -= 1;

        if side_countdown == 0 {
            match (dx, dz) {
                (0, 1) => {
                    (dx, dz) = (-1, 0);
                }
                (-1, 0) => {
                    (dx, dz) = (0, -1);
                    side_len += 1;
                }
                (0, -1) => {
                    (dx, dz) = (1, 0);
                }
                (1, 0) => {
                    (dx, dz) = (0, 1);
                    side_len += 1;
                }
                _ => {
                    error!("The terrain generation spiral looping logic is incorrect.");
                    break 'spiral;
                }
            }

            side_countdown = side_len;
        }
    }
}

fn update_chunk_and_subchunks(
    commands: &mut Commands,
    chunk_q: &mut Query<(Entity, &mut TerrainChunk, &mut Visibility)>,
    reusable_materials: &ReusableMaterials,
    entity: Entity,
) {
    if let Some((_, mut tc, mut visibility)) = alrmo!(chunk_q.get_mut(entity)) {
        let sscale = tc.scale * 0.5;
        let sx = tc.off_x * 2;
        let sz = tc.off_z * 2;

        if tc.lod < MAX_LOD
            && tc.subchunk_tl != Entity::PLACEHOLDER
            && tc.subchunk_tr != Entity::PLACEHOLDER
            && tc.subchunk_bl != Entity::PLACEHOLDER
            && tc.subchunk_br != Entity::PLACEHOLDER
        {
            // This is the case when all subchunks exist.

            let tl = tc.subchunk_tl;
            let tr = tc.subchunk_tr;
            let bl = tc.subchunk_bl;
            let br = tc.subchunk_br;
            update_chunk_and_subchunks(commands, chunk_q, reusable_materials, tl);
            update_chunk_and_subchunks(commands, chunk_q, reusable_materials, tr);
            update_chunk_and_subchunks(commands, chunk_q, reusable_materials, bl);
            update_chunk_and_subchunks(commands, chunk_q, reusable_materials, br);
        } else {
            // This is the case when its at the max lod or not all subchunks exist.

            *visibility = Visibility::Visible;

            // Create the subchunks that don't exist.

            if tc.lod < MAX_LOD {
                if tc.subchunk_tl == Entity::PLACEHOLDER {
                    tc.subchunk_tl = commands
                        .spawn(chunk_bundle(
                            reusable_materials.terrain.clone(),
                            tc.lod + 1,
                            sscale,
                            sx,
                            sz,
                        ))
                        .id();
                }

                if tc.subchunk_tr == Entity::PLACEHOLDER {
                    tc.subchunk_tr = commands
                        .spawn(chunk_bundle(
                            reusable_materials.terrain.clone(),
                            tc.lod + 1,
                            sscale,
                            sx + 1,
                            sz,
                        ))
                        .id();
                }

                if tc.subchunk_bl == Entity::PLACEHOLDER {
                    tc.subchunk_bl = commands
                        .spawn(chunk_bundle(
                            reusable_materials.terrain.clone(),
                            tc.lod + 1,
                            sscale,
                            sx,
                            sz + 1,
                        ))
                        .id();
                }

                if tc.subchunk_br == Entity::PLACEHOLDER {
                    tc.subchunk_br = commands
                        .spawn(chunk_bundle(
                            reusable_materials.terrain.clone(),
                            tc.lod + 1,
                            sscale,
                            sx + 1,
                            sz + 1,
                        ))
                        .id();
                }
            }
        }
    }
}
