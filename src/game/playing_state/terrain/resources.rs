use bevy::prelude::*;

use crate::game::playing_state::terrain::terrain_func::TerrainFunc;

pub struct TheTerrainFunc(pub TerrainFunc);

// Higher means more high-LOD chunks and less low-LOD chunks.
// Has to be at least 0.0, but should be at least big enough to ensure the player is always standing on a highest-LOD chunk.
// Can be higher than 1.0, meaning L0 chunks might not ever render (which isn't a problem).
#[derive(Resource)]
pub struct TerrainLodProportion(pub f32);
