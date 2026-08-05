use bevy::prelude::*;

use crate::game::playing_state::terrain::terrain_func::TerrainFunc;

#[derive(Resource)]
pub struct TheTerrainFunc(pub TerrainFunc);
