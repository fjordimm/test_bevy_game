use std::{collections::HashMap, hash::Hash, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::game::{
    core::states::OverallState,
    playing_state::{
        player::tags::PlayerBody,
        reusable_materials::ReusableMaterials,
        sets::{DuringPlaying, OnEnterPlaying},
        world::terrain::{
            resources::TheTerrainFunc,
            terrain_chunk::{GenerateMeshes, TerrainChunk, TerrainChunkPlugin, chunk_bundle},
            terrain_func::TerrainFunc,
        },
    },
    random::{Prng, rands::GeneralRand},
    util::{alrmo, alrrs},
};

const UPDATE_CHUNKS_INTERVAL: u64 = 100; // In milliseconds.

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_message::<SpawnTerrainChunk>()
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(OnEnterPlaying::ResourceSetup)
            )
            .add_systems(Update,
                handle_spawn_terrain_chunk
                    .in_set(DuringPlaying)
            )
            .add_systems(Update,
                update_chunks
                    .in_set(DuringPlaying)
                    .run_if(on_timer(Duration::from_millis(UPDATE_CHUNKS_INTERVAL)))
            )
            .add_plugins(TerrainChunkPlugin)
        ;
    }
}

fn on_enter(mut commands: Commands, mut prng: Single<&mut Prng, With<GeneralRand>>) {
    commands.insert_resource(TheTerrainFunc(TerrainFunc::new(&mut prng)));
    commands.insert_resource(ChunkDict(HashMap::new()));
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
struct ChunkDict(HashMap<ChunkKey, Entity>);

#[derive(Message)]
pub struct SpawnTerrainChunk {
    scale: f32,
    off_x: i32,
    off_z: i32,
}

impl SpawnTerrainChunk {
    pub fn new(scale: f32, off_x: i32, off_z: i32) -> Self {
        Self {
            scale: scale,
            off_x: off_x,
            off_z: off_z,
        }
    }
}

fn handle_spawn_terrain_chunk(
    mut commands: Commands,
    mut messages: MessageReader<SpawnTerrainChunk>,
    reusable_materials: Res<ReusableMaterials>,
    mut chunk_dict: ResMut<ChunkDict>,
) {
    messages.read().for_each(|msg| {
        let chunk = commands
            .spawn(chunk_bundle(
                reusable_materials.terrain.clone(),
                msg.scale,
                msg.off_x,
                msg.off_z,
            ))
            .id();

        chunk_dict
            .0
            .insert(ChunkKey::new(msg.off_x, msg.off_z), chunk);
    });
}

pub(super) const CW: usize = 8; // Chunk Width (and height).
const CSCALE: f32 = 1.;
const L0_RENDER_DIST: i32 = 2;

fn update_chunks(
    player_q: Option<Single<&Transform, With<PlayerBody>>>,
    chunk_dict: Res<ChunkDict>,
    mut chunk_q: Query<&mut TerrainChunk>,
    mut gm_messages: MessageWriter<GenerateMeshes>,
    mut stc_messages: MessageWriter<SpawnTerrainChunk>,
) {
    let player_tran = alrrs!(player_q);

    let x_center: i32 = (player_tran.translation.x / (CSCALE * CW as f32) - 0.5).round() as i32;
    let z_center: i32 = (player_tran.translation.z / (CSCALE * CW as f32) - 0.5).round() as i32;

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
            let entity = chunk_dict.0.get(&ChunkKey::new(x, z));
            if let Some(entity) = entity {
                if let Some(mut chunk) = alrmo!(chunk_q.get_mut(*entity)) {
                    chunk.generate_mesh_nonredundantly(&mut gm_messages, entity);
                }

                // TODO: change visibility.
            } else {
                // TODO: stop using magic values for scale
                stc_messages.write(SpawnTerrainChunk::new(CSCALE, x, z));
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
