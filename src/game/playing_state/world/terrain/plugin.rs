use bevy::prelude::*;

use crate::game::{
    core::states::OverallState,
    playing_state::world::terrain::{terrain_chunk::TerrainChunkPlugin, terrain_func::TerrainFunc},
    random::{Prng, rands::GeneralRand},
};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::EnteringPlaying),
                on_enter
            )
            .add_plugins(TerrainChunkPlugin)
        ;
    }
}

#[derive(Resource)]
pub struct TheTerrainFunc(pub TerrainFunc);

fn on_enter(mut commands: Commands, mut prng: Single<&mut Prng, With<GeneralRand>>) {
    commands.insert_resource(TheTerrainFunc(TerrainFunc::new(&mut prng)));
}
