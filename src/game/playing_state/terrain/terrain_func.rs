use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};
use rand::{Rng, SeedableRng};
use rand_distr::num_traits::Pow;

use crate::game::random::Prng;

pub struct TerrainFunc {
    octaves: Vec<OpenSimplex>,
}

const NUM_OCTAVES: usize = 15;

impl TerrainFunc {
    pub fn new(seed: [u8; 8]) -> Self {
        let mut me = TerrainFunc {
            octaves: Vec::with_capacity(NUM_OCTAVES),
        };

        let mut prng = Prng::from_seed(seed);
        for _ in 0..NUM_OCTAVES {
            me.octaves.push(OpenSimplex::new(prng.next_u32()));
        }

        me
    }

    pub fn at(&self, x: f32, z: f32) -> f32 {
        let x = x as f64;
        let z = z as f64;

        let mut y = 0.;

        let mut frq = 0.0005;
        let mut amp = 6.;
        for i in 0..NUM_OCTAVES {
            y += amp * self.octaves[i].get([frq * x, frq * z]);

            frq *= 2.;
            amp *= 0.5;
        }

        y = 2.0.pow(y);

        y *= 15.;

        y as f32
    }
}
