use std::{collections::HashMap, hash::Hash, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};
use priority_queue::PriorityQueue;

use crate::game::{
    core::states::OverallState,
    playing_state::{
        coord_rebasing::{CoordRebasingOrigin, world_space_transf},
        player::tags::PlayerTransf,
        reusable_materials::ReusableMaterials,
        sets::{DuringPlaying, OnEnterPlaying},
        tags::PlayingStateEntity,
        terrain::{
            resources::{TerrainLodProportion, TheTerrainFunc},
            terrain_func::TerrainFunc,
            terrain_mesh::{change_mesh_from_perim_lod_vertices, create_terrain_mesh},
        },
    },
    util::{alrmo, alrms, alrrs, seed_from_u64},
};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Playing),
                (on_enter1, on_enter2)
                    .in_set(OnEnterPlaying::ResourceSetup)
            )
            .add_systems(Update,
                (inactivate_all_chunks, activate_chunks, update_chunk_perimeters)
                    .chain()
                    .in_set(DuringPlaying)
                    .run_if(on_timer(Duration::from_millis(UPDATE_CHUNKS_INTERVAL)))
            )
            .add_systems(Update,
                gen_next_mesh_in_queue
                    .in_set(DuringPlaying)
                    .after(update_chunk_perimeters)
            )
        ;
    }
}

const UPDATE_CHUNKS_INTERVAL: u64 = 800; // In milliseconds.

pub(super) const CW: usize = 16; // Chunk Width (and height). Minimum value: 3 (because of the perimeter).
const MAX_LOD: usize = 4;
const LL_CHUNK_SCALE: f32 = 6.; // LL stands for Last-LOD (the highest LOD).
const L0_CHUNK_SCALE: f32 = LL_CHUNK_SCALE * 2u32.pow(MAX_LOD as u32) as f32;
const L0_RENDER_DIST: i64 = 10;

fn on_enter1(world: &mut World) {
    // TODO: change the seed to not be this arbitrary number.
    world.insert_non_send_resource(TheTerrainFunc(TerrainFunc::new(seed_from_u64(12345))));
}

fn on_enter2(mut commands: Commands) {
    commands.insert_resource(TerrainLodProportion(0.15));
    commands.insert_resource(ChunkDicts(std::array::from_fn(|_| {
        ChunkDict(HashMap::new())
    })));
    commands.insert_resource(MeshGenQueue(PriorityQueue::new()));
}

#[derive(Resource)]
struct ChunkDicts([ChunkDict; MAX_LOD + 1]);

struct ChunkDict(HashMap<ChunkDictKey, Entity>);

#[derive(PartialEq, Eq, Hash)]
struct ChunkDictKey {
    x: i64,
    z: i64,
}

impl ChunkDictKey {
    fn new(x: i64, z: i64) -> Self {
        Self { x: x, z: z }
    }
}

fn chunk_bundle(
    reusable_materials: &ReusableMaterials,
    lod: usize,
    scale: f32,
    coord_rebasing_origin: &Vec3,
    off_x: i64,
    off_z: i64,
) -> impl Bundle {
    (
        PlayingStateEntity,
        Chunk {
            lod,
            scale: scale,
            off_x: off_x,
            off_z: off_z,
            has_been_queued_for_mesh: false,
            has_mesh: false,
            perimeter_entity: None,
        },
        world_space_transf(Transform::from_translation(
            Vec3::new(
                scale * CW as f32 * off_x as f32,
                0.,
                scale * CW as f32 * off_z as f32,
            ) - coord_rebasing_origin,
        )),
        MeshMaterial3d(reusable_materials.terrain.clone()),
        Visibility::Hidden,
    )
}

#[derive(Component)]
struct Chunk {
    lod: usize,
    scale: f32,
    off_x: i64,
    off_z: i64,
    has_been_queued_for_mesh: bool,
    has_mesh: bool,
    perimeter_entity: Option<Entity>, // The child entity that has the perimeter mesh.
}

impl Chunk {
    fn queue_for_mesh_nonredundantly(
        &mut self,
        mesh_gen_queue: &mut ResMut<MeshGenQueue>,
        entity: Entity,
    ) {
        if !self.has_been_queued_for_mesh {
            mesh_gen_queue.queue_chunk(entity, MAX_LOD - self.lod);
            self.has_been_queued_for_mesh = true;
        }
    }
}

#[derive(Component)]
struct ChunkPerimeter {
    perim_lod_verticies: Vec<Vec<[f32; 3]>>,
}

// Has a mesh and is visible, or is in queue for a mesh.
#[derive(Component)]
struct ActiveOrQueued;

fn inactivate_all_chunks(
    mut commands: Commands,
    mut chunk_q: Query<(Entity, &mut Visibility), (With<Chunk>, With<ActiveOrQueued>)>,
) {
    chunk_q.iter_mut().for_each(|(entity, mut visibility)| {
        *visibility = Visibility::Hidden;
        commands.entity(entity).remove::<ActiveOrQueued>();
    });
}

fn activate_chunks(
    mut commands: Commands,
    coord_rebasing_origin: Res<CoordRebasingOrigin>,
    player_q: Option<Single<&Transform, With<PlayerTransf>>>,
    mut chunk_dicts: ResMut<ChunkDicts>,
    mut chunk_q: Query<(Entity, &mut Chunk, &mut Visibility)>,
    lod_proportion: Res<TerrainLodProportion>,
    reusable_materials: Res<ReusableMaterials>,
    mut mesh_gen_queue: ResMut<MeshGenQueue>,
) {
    let coord_rebasing_origin = &coord_rebasing_origin.0.as_vec3();

    let player_pos = alrrs!(player_q).translation + coord_rebasing_origin;
    let x_center = (player_pos.x / (L0_CHUNK_SCALE * CW as f32) - 0.5).round() as i64;
    let z_center = (player_pos.z / (L0_CHUNK_SCALE * CW as f32) - 0.5).round() as i64;

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
            if let Some(l0_chunk_entity) = chunk_dicts.0[0].0.get(&ChunkDictKey::new(x, z)) {
                let l0_chunk_entity = *l0_chunk_entity;

                if let Some((_, mut cc, _)) = alrmo!(chunk_q.get_mut(l0_chunk_entity)) {
                    cc.queue_for_mesh_nonredundantly(&mut mesh_gen_queue, l0_chunk_entity);
                    commands.entity(l0_chunk_entity).insert(ActiveOrQueued);

                    if cc.has_mesh {
                        activate_chunk_or_subchunks(
                            &mut commands,
                            &mut chunk_dicts,
                            &mut chunk_q,
                            &reusable_materials,
                            lod_proportion.0,
                            &player_pos,
                            &mut mesh_gen_queue,
                            coord_rebasing_origin,
                            l0_chunk_entity,
                        );
                    }
                }
            } else {
                let entity = commands
                    .spawn(chunk_bundle(
                        &reusable_materials,
                        0,
                        L0_CHUNK_SCALE,
                        coord_rebasing_origin,
                        x,
                        z,
                    ))
                    .id();

                chunk_dicts.0[0].0.insert(ChunkDictKey::new(x, z), entity);
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

// Should only be called on chunks who have a mesh.
fn activate_chunk_or_subchunks(
    commands: &mut Commands,
    chunk_dicts: &mut ChunkDicts,
    chunk_q: &mut Query<(Entity, &mut Chunk, &mut Visibility)>,
    reusable_materials: &Res<ReusableMaterials>,
    lod_proportion: f32,
    player_pos: &Vec3,
    mesh_gen_queue: &mut ResMut<MeshGenQueue>,
    coord_rebasing_origin: &Vec3,
    entity: Entity,
) {
    let mut not_doing_subchunks = true;

    if let Some((_, cc, _)) = alrmo!(chunk_q.get(entity)) {
        let sscale = cc.scale * 0.5;
        let sx = cc.off_x * 2;
        let sz = cc.off_z * 2;

        let should_do_subchunks = {
            let real_x = (cc.off_x as f32 + 0.5) * cc.scale * CW as f32;
            let real_z = (cc.off_z as f32 + 0.5) * cc.scale * CW as f32;
            let dist_to_player =
                ((player_pos.x - real_x).powi(2) + (player_pos.z - real_z).powi(2)).sqrt();

            cc.lod < MAX_LOD
                && dist_to_player < L0_RENDER_DIST as f32 * lod_proportion * cc.scale * CW as f32
        };

        if should_do_subchunks {
            if let Some(chunk_dict) = alrms!(chunk_dicts.0.get_mut(cc.lod + 1)) {
                let chunk_dict = &mut chunk_dict.0;

                let tl = {
                    let sx = sx;
                    let sz = sz;

                    if let Some(subchunk_entity) = chunk_dict.get(&ChunkDictKey::new(sx, sz)) {
                        *subchunk_entity
                    } else {
                        let subchunk_entity = commands
                            .spawn(chunk_bundle(
                                reusable_materials,
                                cc.lod + 1,
                                sscale,
                                coord_rebasing_origin,
                                sx,
                                sz,
                            ))
                            .id();

                        chunk_dict.insert(ChunkDictKey::new(sx, sz), subchunk_entity);

                        subchunk_entity
                    }
                };
                let tr = {
                    let sx = sx + 1;
                    let sz = sz;

                    if let Some(subchunk_entity) = chunk_dict.get(&ChunkDictKey::new(sx, sz)) {
                        *subchunk_entity
                    } else {
                        let subchunk_entity = commands
                            .spawn(chunk_bundle(
                                reusable_materials,
                                cc.lod + 1,
                                sscale,
                                coord_rebasing_origin,
                                sx,
                                sz,
                            ))
                            .id();

                        chunk_dict.insert(ChunkDictKey::new(sx, sz), subchunk_entity);

                        subchunk_entity
                    }
                };
                let bl = {
                    let sx = sx;
                    let sz = sz + 1;

                    if let Some(subchunk_entity) = chunk_dict.get(&ChunkDictKey::new(sx, sz)) {
                        *subchunk_entity
                    } else {
                        let subchunk_entity = commands
                            .spawn(chunk_bundle(
                                reusable_materials,
                                cc.lod + 1,
                                sscale,
                                coord_rebasing_origin,
                                sx,
                                sz,
                            ))
                            .id();

                        chunk_dict.insert(ChunkDictKey::new(sx, sz), subchunk_entity);

                        subchunk_entity
                    }
                };
                let br = {
                    let sx = sx + 1;
                    let sz = sz + 1;

                    if let Some(subchunk_entity) = chunk_dict.get(&ChunkDictKey::new(sx, sz)) {
                        *subchunk_entity
                    } else {
                        let subchunk_entity = commands
                            .spawn(chunk_bundle(
                                reusable_materials,
                                cc.lod + 1,
                                sscale,
                                coord_rebasing_origin,
                                sx,
                                sz,
                            ))
                            .id();

                        chunk_dict.insert(ChunkDictKey::new(sx, sz), subchunk_entity);

                        subchunk_entity
                    }
                };

                let tl_has_mesh = if let Ok((_, mut cc, _)) = chunk_q.get_mut(tl) {
                    cc.queue_for_mesh_nonredundantly(mesh_gen_queue, tl);
                    commands.entity(tl).insert(ActiveOrQueued);

                    cc.has_mesh
                } else {
                    false
                };
                let tr_has_mesh = if let Ok((_, mut cc, _)) = chunk_q.get_mut(tr) {
                    cc.queue_for_mesh_nonredundantly(mesh_gen_queue, tr);
                    commands.entity(tr).insert(ActiveOrQueued);

                    cc.has_mesh
                } else {
                    false
                };
                let bl_has_mesh = if let Ok((_, mut cc, _)) = chunk_q.get_mut(bl) {
                    cc.queue_for_mesh_nonredundantly(mesh_gen_queue, bl);
                    commands.entity(bl).insert(ActiveOrQueued);

                    cc.has_mesh
                } else {
                    false
                };
                let br_has_mesh = if let Ok((_, mut cc, _)) = chunk_q.get_mut(br) {
                    cc.queue_for_mesh_nonredundantly(mesh_gen_queue, br);
                    commands.entity(br).insert(ActiveOrQueued);

                    cc.has_mesh
                } else {
                    false
                };

                if tl_has_mesh && tr_has_mesh && bl_has_mesh && br_has_mesh {
                    activate_chunk_or_subchunks(
                        commands,
                        chunk_dicts,
                        chunk_q,
                        &reusable_materials,
                        lod_proportion,
                        &player_pos,
                        mesh_gen_queue,
                        coord_rebasing_origin,
                        tl,
                    );
                    activate_chunk_or_subchunks(
                        commands,
                        chunk_dicts,
                        chunk_q,
                        &reusable_materials,
                        lod_proportion,
                        &player_pos,
                        mesh_gen_queue,
                        coord_rebasing_origin,
                        tr,
                    );
                    activate_chunk_or_subchunks(
                        commands,
                        chunk_dicts,
                        chunk_q,
                        &reusable_materials,
                        lod_proportion,
                        &player_pos,
                        mesh_gen_queue,
                        coord_rebasing_origin,
                        bl,
                    );
                    activate_chunk_or_subchunks(
                        commands,
                        chunk_dicts,
                        chunk_q,
                        &reusable_materials,
                        lod_proportion,
                        &player_pos,
                        mesh_gen_queue,
                        coord_rebasing_origin,
                        br,
                    );

                    not_doing_subchunks = false;
                }
            }
        }
    }

    if not_doing_subchunks {
        if let Some((_, _, mut visibility)) = alrmo!(chunk_q.get_mut(entity)) {
            *visibility = Visibility::Visible;
        }
    } else {
        commands.entity(entity).remove::<ActiveOrQueued>();
    }
}

fn update_chunk_perimeters(
    chunk_dicts: Res<ChunkDicts>,
    chunk_q: Query<(&Chunk, &Visibility), With<ActiveOrQueued>>,
    chunk_perim_q: Query<(&ChunkPerimeter, &Mesh3d)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    chunk_q.iter().for_each(|(cc, visibility)| {
        if *visibility == Visibility::Visible {
            if let Some(perim_entity) = alrms!(cc.perimeter_entity) {
                let surrounding_lods =
                    get_surrounding_chunk_lods(&chunk_dicts, &chunk_q, cc.lod, cc.off_x, cc.off_z);
                let north_lod = match surrounding_lods.0 {
                    Some(lod) => lod,
                    None => cc.lod,
                };
                let east_lod = match surrounding_lods.1 {
                    Some(lod) => lod,
                    None => cc.lod,
                };
                let south_lod = match surrounding_lods.2 {
                    Some(lod) => lod,
                    None => cc.lod,
                };
                let west_lod = match surrounding_lods.3 {
                    Some(lod) => lod,
                    None => cc.lod,
                };

                if let Some((cpc, mesh3d)) = alrmo!(chunk_perim_q.get(perim_entity)) {
                    if let Some(mesh) = alrms!(meshes.get_mut(mesh3d.0.id())) {
                        change_mesh_from_perim_lod_vertices(
                            mesh,
                            &cpc.perim_lod_verticies,
                            north_lod,
                            east_lod,
                            south_lod,
                            west_lod,
                        );
                    }
                }
            }
        }
    });
}

fn get_surrounding_chunk_lods(
    chunk_dicts: &ChunkDicts,
    chunk_q: &Query<(&Chunk, &Visibility), With<ActiveOrQueued>>,
    coords_lod: usize,
    x: i64,
    z: i64,
) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
    let north = get_active_chunk_lod_at(chunk_dicts, chunk_q, coords_lod, x, z - 1);
    let east = get_active_chunk_lod_at(chunk_dicts, chunk_q, coords_lod, x + 1, z);
    let south = get_active_chunk_lod_at(chunk_dicts, chunk_q, coords_lod, x, z + 1);
    let west = get_active_chunk_lod_at(chunk_dicts, chunk_q, coords_lod, x - 1, z);

    (north, east, south, west)
}

fn get_active_chunk_lod_at(
    chunk_dicts: &ChunkDicts,
    chunk_q: &Query<(&Chunk, &Visibility), With<ActiveOrQueued>>,
    coords_lod: usize,
    x: i64,
    z: i64,
) -> Option<usize> {
    let mut lod = coords_lod;
    let mut x = x;
    let mut z = z;
    loop {
        if let Some(entity) = chunk_dicts.0[lod].0.get(&ChunkDictKey::new(x, z)) {
            if let Ok((cc, visibility)) = chunk_q.get(*entity) {
                if *visibility == Visibility::Visible {
                    return Some(cc.lod);
                }
            }
        }

        if lod == 0 {
            return None;
        }

        lod -= 1;
        x = if x.is_negative() {
            (x + 1) / 2 - 1
        } else {
            x / 2
        };
        z = if z.is_negative() {
            (z + 1) / 2 - 1
        } else {
            z / 2
        };
    }
}

#[derive(Resource)]
struct MeshGenQueue(PriorityQueue<Entity, usize>);

impl MeshGenQueue {
    fn queue_chunk(&mut self, entity: Entity, lod: usize) {
        self.0.push(entity, lod);
    }
}

fn gen_next_mesh_in_queue(
    mut commands: Commands,
    mut mesh_gen_queue: ResMut<MeshGenQueue>,
    mut chunk_q: Query<(&mut Chunk, Option<&ActiveOrQueued>)>,
    terrain_func: NonSend<TheTerrainFunc>,
    mut meshes: ResMut<Assets<Mesh>>,
    reusable_materials: Res<ReusableMaterials>,
) {
    if let Some((entity, _)) = mesh_gen_queue.0.pop() {
        if let Some((mut cc, is_active_chunk)) = alrmo!(chunk_q.get_mut(entity)) {
            if let Some(_) = is_active_chunk {
                let (main_mesh, perim_mesh, perim_lod_vertices) = create_terrain_mesh(
                    &terrain_func.0,
                    cc.scale,
                    cc.off_x,
                    cc.off_z,
                    cc.lod as usize,
                );

                commands
                    .entity(entity)
                    .insert(Mesh3d(meshes.add(main_mesh)));

                let perimeter = commands
                    .spawn((
                        PlayingStateEntity,
                        ChunkPerimeter {
                            perim_lod_verticies: perim_lod_vertices,
                        },
                        Transform::default(),
                        MeshMaterial3d(reusable_materials.terrain.clone()),
                        Mesh3d(meshes.add(perim_mesh)),
                        Visibility::Inherited,
                    ))
                    .id();
                commands.entity(entity).add_child(perimeter);
                cc.perimeter_entity = Some(perimeter);

                cc.has_mesh = true;
            } else {
                cc.has_been_queued_for_mesh = false;
            }
        }
    }
}
