use bevy::prelude::*;

use crate::game::playing_state::world::terrain::terrain_func::TerrainFunc;

#[derive(Resource)]
pub struct TheTerrainFunc(pub TerrainFunc);
