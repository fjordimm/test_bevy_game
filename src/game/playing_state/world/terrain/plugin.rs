use std::{collections::HashMap, hash::Hash};

use bevy::prelude::*;

use crate::game::{
    core::states::OverallState,
    playing_state::{
        reusable_materials::ReusableMaterials,
        sets::{DuringPlaying, OnEnterPlaying},
        world::terrain::{
            resources::TheTerrainFunc,
            terrain_chunk::{GenerateMeshes, TerrainChunk, TerrainChunkPlugin, chunk_bundle},
            terrain_func::TerrainFunc,
        },
    },
    random::{Prng, rands::GeneralRand},
    util::alrmo,
};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_message::<SpawnTerrainChunk>()
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(OnEnterPlaying::Setup)
            )
            .add_systems(Update,
                handle_spawn_terrain_chunk
                    .in_set(DuringPlaying)
            )
            .add_systems(Update,
                update_chunks
                    .in_set(DuringPlaying)
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

fn update_chunks(
    chunk_dict: Res<ChunkDict>,
    mut chunk_q: Query<&mut TerrainChunk>,
    mut gm_messages: MessageWriter<GenerateMeshes>,
) {
    // TODOc: only do ones near the player.
    chunk_dict.0.values().for_each(|entity| {
        if let Some(mut chunk) = alrmo!(chunk_q.get_mut(*entity)) {
            chunk.generate_mesh_nonredundantly(&mut gm_messages, entity);
        }
    });
}
