use bevy::prelude::*;
use noise::OpenSimplex;
use rand::Rng;

use crate::game::random::Prng;

pub struct TerrainFunc {
    octaves: Vec<OpenSimplex>,
}

const NUM_OCTAVES: usize = 5;

impl TerrainFunc {
    pub fn new(prng: &mut Prng) -> Self {
        let mut me = TerrainFunc {
            octaves: Vec::with_capacity(NUM_OCTAVES),
        };

        for _ in 0..NUM_OCTAVES {
            me.octaves.push(OpenSimplex::new(prng.next_u32()));
        }

        me
    }
}
