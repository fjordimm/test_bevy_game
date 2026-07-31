use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};
use rand::Rng;

use crate::game::random::Prng;

pub struct TerrainFunc {
    octaves: Vec<OpenSimplex>,
}

const NUM_OCTAVES: usize = 1;

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

    pub fn at(&self, mut x: f32, mut z: f32) -> f32 {
        x *= 0.3;
        z *= 0.3;

        let mut y: f32 = 0.;

        for i in 0..NUM_OCTAVES {
            y += self.octaves[i].get([x as f64, z as f64]) as f32;
        }

        y
    }
}
