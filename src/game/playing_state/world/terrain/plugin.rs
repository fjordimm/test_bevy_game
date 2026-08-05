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
            resources::TheTerrainFunc,
            terrain_func::TerrainFunc,
            terrain_mesh::{change_mesh_from_perim_lod_positions, create_terrain_meshes},
        },
    },
    util::{alrmo, alrms, alrrs, seed_from_u64},
};

const UPDATE_CHUNKS_INTERVAL: u64 = 800; // In milliseconds.

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

fn on_enter(mut commands: Commands) {
    // TODO: change the seed to not be this arbitrary number.
    commands.insert_resource(TheTerrainFunc(TerrainFunc::new(seed_from_u64(12345))));
    commands.insert_resource(L0ChunkDict(HashMap::new()));
}

pub(super) const CW: usize = 32; // Chunk Width (and height). Minimum value: 3 (because of the perimeter).
const MAX_LOD: i32 = 6;
const LL_CHUNK_SCALE: f32 = 3.;
const L0_CHUNK_SCALE: f32 = LL_CHUNK_SCALE * 2u32.pow(MAX_LOD as u32) as f32;
const L0_RENDER_DIST: i32 = 2;
const LOD_PROPORTION: f32 = 2.5;

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
            perimeter_entity: None,
            subchunk_tl: None,
            subchunk_tr: None,
            subchunk_bl: None,
            subchunk_br: None,
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
    perimeter_entity: Option<Entity>, // The child entity that has the perimeter mesh.
    subchunk_tl: Option<Entity>,
    subchunk_tr: Option<Entity>,
    subchunk_bl: Option<Entity>,
    subchunk_br: Option<Entity>,
}

#[derive(Component)]
struct TerrainChunkPerimeter {
    perim_lod_positions: Vec<Vec<[f32; 3]>>,
}

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
        if let Some(mut tc) = alrmo!(chunk_q.get_mut(msg.0)) {
            let (prim_mesh, perim_mesh, perim_lod_positions) = create_terrain_meshes(
                &terrain_func.0,
                tc.scale,
                tc.scale * CW as f32 * tc.off_x as f32,
                tc.scale * CW as f32 * tc.off_z as f32,
                tc.lod as usize,
            );

            commands.entity(msg.0).insert(Mesh3d(meshes.add(prim_mesh)));

            let perimeter = commands
                .spawn((
                    PlayingStateEntity,
                    TerrainChunkPerimeter {
                        perim_lod_positions: perim_lod_positions,
                    },
                    MeshMaterial3d(reusable_materials.terrain.clone()),
                    Mesh3d(meshes.add(perim_mesh)),
                ))
                .id();
            commands.entity(msg.0).add_child(perimeter);
            tc.perimeter_entity = Some(perimeter);

            tc.has_generated_mesh = true;
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
    chunk_perim_q: Query<(&TerrainChunkPerimeter, &Mesh3d)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Update the perimeters to match surrounding chunks with different lods.
    chunk_q.iter().for_each(|(_, tc, _)| {
        if let Some(perim_entity) = tc.perimeter_entity {
            let tc_lod = tc.lod;
            let tc_off_x = tc.off_x;
            let tc_off_z = tc.off_z;
            let surrounding_lods =
                get_surrounding_chunk_lods(&*l0_chunk_dict, &chunk_q, tc_lod, tc_off_x, tc_off_z);
            let north_lod = match surrounding_lods.0 {
                Some(lod) => lod as usize,
                None => tc_lod as usize,
            };
            let east_lod = match surrounding_lods.1 {
                Some(lod) => lod as usize,
                None => tc_lod as usize,
            };
            let south_lod = match surrounding_lods.2 {
                Some(lod) => lod as usize,
                None => tc_lod as usize,
            };
            let west_lod = match surrounding_lods.3 {
                Some(lod) => lod as usize,
                None => tc_lod as usize,
            };

            if let Some((tcp, mesh3d)) = alrmo!(chunk_perim_q.get(perim_entity)) {
                if let Some(mesh) = alrms!(meshes.get_mut(mesh3d.0.id())) {
                    change_mesh_from_perim_lod_positions(
                        mesh,
                        &tcp.perim_lod_positions,
                        north_lod,
                        east_lod,
                        south_lod,
                        west_lod,
                    );
                }
            }
        }
    });

    chunk_q
        .iter_mut()
        .for_each(|(entity, mut tc, mut visibility)| {
            // Generate meshes for chunks that don't already have them (but only if they were left visible).
            if *visibility == Visibility::Visible {
                tc.generate_mesh_nonredundantly(&mut gm_messages, &entity);
            }

            // Set all chunks to be hidden. The rest of this function will set the appropriate ones to be visible.
            *visibility = Visibility::Hidden;
        });

    let player_transf = alrrs!(player_q);
    let x_center: i32 =
        (player_transf.translation.x / (L0_CHUNK_SCALE * CW as f32) - 0.5).round() as i32;
    let z_center: i32 =
        (player_transf.translation.z / (L0_CHUNK_SCALE * CW as f32) - 0.5).round() as i32;

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
                    &l0_chunk_dict,
                    &mut chunk_q,
                    &reusable_materials,
                    &player_transf,
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
    l0_chunk_dict: &L0ChunkDict,
    chunk_q: &mut Query<(Entity, &mut TerrainChunk, &mut Visibility)>,
    reusable_materials: &ReusableMaterials,
    player_transf: &Transform,
    entity: Entity,
) {
    if let Some((_, mut tc, mut visibility)) = alrmo!(chunk_q.get_mut(entity)) {
        let sscale = tc.scale * 0.5;
        let sx = tc.off_x * 2;
        let sz = tc.off_z * 2;

        let should_do_subchunks = {
            let real_x = (tc.off_x as f32 + 0.5) * tc.scale * CW as f32;
            let real_z = (tc.off_z as f32 + 0.5) * tc.scale * CW as f32;
            let dist_to_player = ((player_transf.translation.x - real_x).powi(2)
                + (player_transf.translation.z - real_z).powi(2))
            .sqrt();

            tc.lod < MAX_LOD && dist_to_player < LOD_PROPORTION * tc.scale * CW as f32
        };

        let tl = tc.subchunk_tl;
        let tr = tc.subchunk_tr;
        let bl = tc.subchunk_bl;
        let br = tc.subchunk_br;
        if should_do_subchunks
            && let Some(tl) = tl
            && let Some(tr) = tr
            && let Some(bl) = bl
            && let Some(br) = br
        {
            // This is the case when all subchunks exist.

            update_chunk_and_subchunks(
                commands,
                l0_chunk_dict,
                chunk_q,
                reusable_materials,
                player_transf,
                tl,
            );
            update_chunk_and_subchunks(
                commands,
                l0_chunk_dict,
                chunk_q,
                reusable_materials,
                player_transf,
                tr,
            );
            update_chunk_and_subchunks(
                commands,
                l0_chunk_dict,
                chunk_q,
                reusable_materials,
                player_transf,
                bl,
            );
            update_chunk_and_subchunks(
                commands,
                l0_chunk_dict,
                chunk_q,
                reusable_materials,
                player_transf,
                br,
            );
        } else {
            // This is the case when its at the max lod or not all subchunks exist.

            // Create the subchunks that don't exist.

            if should_do_subchunks {
                if tc.subchunk_tl.is_none() {
                    tc.subchunk_tl = Some(
                        commands
                            .spawn(chunk_bundle(
                                reusable_materials.terrain.clone(),
                                tc.lod + 1,
                                sscale,
                                sx,
                                sz,
                            ))
                            .id(),
                    );
                }

                if tc.subchunk_tr.is_none() {
                    tc.subchunk_tr = Some(
                        commands
                            .spawn(chunk_bundle(
                                reusable_materials.terrain.clone(),
                                tc.lod + 1,
                                sscale,
                                sx + 1,
                                sz,
                            ))
                            .id(),
                    );
                }

                if tc.subchunk_bl.is_none() {
                    tc.subchunk_bl = Some(
                        commands
                            .spawn(chunk_bundle(
                                reusable_materials.terrain.clone(),
                                tc.lod + 1,
                                sscale,
                                sx,
                                sz + 1,
                            ))
                            .id(),
                    );
                }

                if tc.subchunk_br.is_none() {
                    tc.subchunk_br = Some(
                        commands
                            .spawn(chunk_bundle(
                                reusable_materials.terrain.clone(),
                                tc.lod + 1,
                                sscale,
                                sx + 1,
                                sz + 1,
                            ))
                            .id(),
                    );
                }
            }

            // Stuff for this chunk.

            *visibility = Visibility::Visible;
        }
    }
}

// Return value is ordered as (north, east, south, west).
fn get_surrounding_chunk_lods(
    l0_chunk_dict: &L0ChunkDict,
    chunk_q: &Query<(Entity, &mut TerrainChunk, &mut Visibility)>,
    tc_lod: i32,
    tc_off_x: i32,
    tc_off_z: i32,
) -> (Option<i32>, Option<i32>, Option<i32>, Option<i32>) {
    let north_entity = get_active_chunk_at(l0_chunk_dict, chunk_q, tc_lod, tc_off_x, tc_off_z - 1);
    let east_entity = get_active_chunk_at(l0_chunk_dict, chunk_q, tc_lod, tc_off_x + 1, tc_off_z);
    let south_entity = get_active_chunk_at(l0_chunk_dict, chunk_q, tc_lod, tc_off_x, tc_off_z + 1);
    let west_entity = get_active_chunk_at(l0_chunk_dict, chunk_q, tc_lod, tc_off_x - 1, tc_off_z);

    let north = match north_entity {
        Some(e) => match alrmo!(chunk_q.get(e)) {
            Some((_, tc, _)) => Some(tc.lod),
            None => None,
        },
        None => None,
    };
    let east = match east_entity {
        Some(e) => match alrmo!(chunk_q.get(e)) {
            Some((_, tc, _)) => Some(tc.lod),
            None => None,
        },
        None => None,
    };
    let south = match south_entity {
        Some(e) => match alrmo!(chunk_q.get(e)) {
            Some((_, tc, _)) => Some(tc.lod),
            None => None,
        },
        None => None,
    };
    let west = match west_entity {
        Some(e) => match alrmo!(chunk_q.get(e)) {
            Some((_, tc, _)) => Some(tc.lod),
            None => None,
        },
        None => None,
    };

    (north, east, south, west)
}

// Stops zeroing in at `coords_lod` (lod of the coords).
fn get_active_chunk_at(
    l0_chunk_dict: &L0ChunkDict,
    chunk_q: &Query<(Entity, &mut TerrainChunk, &mut Visibility)>,
    coords_lod: i32,
    x: i32,
    z: i32,
) -> Option<Entity> {
    let l0_x = x / 2i32.pow(coords_lod as u32);
    let l0_z = z / 2i32.pow(coords_lod as u32);

    if let Some(l0_chunk_entity) = l0_chunk_dict.0.get(&ChunkKey::new(l0_x, l0_z)) {
        if let Some((l0_entity, l0_tc, l0_visibility)) = alrmo!(chunk_q.get(*l0_chunk_entity)) {
            let (mut c_entity, mut c_tc, mut c_visibility) = (l0_entity, l0_tc, l0_visibility);

            loop {
                if c_tc.lod == coords_lod || *c_visibility == Visibility::Visible {
                    return Some(c_entity);
                }

                let subchunk_entity = match (
                    ((x / 2i32.pow((coords_lod - (c_tc.lod + 1)) as u32)) % 2).abs(),
                    ((z / 2i32.pow((coords_lod - (c_tc.lod + 1)) as u32)) % 2).abs(),
                ) {
                    (0, 0) => c_tc.subchunk_tl,
                    (1, 0) => c_tc.subchunk_tr,
                    (0, 1) => c_tc.subchunk_bl,
                    (1, 1) => c_tc.subchunk_br,
                    _ => {
                        error!("The terrain generation zeroing in logic is incorrect.");
                        return None;
                    }
                };

                if let Some(subchunk_entity) = subchunk_entity {
                    if let Some((new_entity, new_tc, new_visibility)) =
                        alrmo!(chunk_q.get(subchunk_entity))
                    {
                        (c_entity, c_tc, c_visibility) = (new_entity, new_tc, new_visibility);
                    }
                } else {
                    return Some(c_entity);
                }
            }
        } else {
            None
        }
    } else {
        None
    }
}
