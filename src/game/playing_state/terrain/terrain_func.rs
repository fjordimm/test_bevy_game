use bevy::prelude::*;
use noise::{NoiseFn, SuperSimplex, Worley, core::worley::ReturnType};
use rand::{Rng, SeedableRng};

use crate::game::{
    random::Prng,
    util::{lerp, smoothstep_in_bounds_only},
};

pub struct TerrainFunc {
    rough: OctavedNoiseSampler,
    mountains_worleys: [Worley; 1],
}

impl TerrainFunc {
    pub fn new(seed: [u8; 8]) -> Self {
        let mut prng = Prng::from_seed(seed);

        Self {
            rough: OctavedNoiseSampler::new(&mut prng, 5, 0.005),
            mountains_worleys: std::array::from_fn(|_| {
                Worley::new(prng.next_u32())
                    .set_return_type(ReturnType::Distance)
                    .set_distance_function(worley_distance_function)
            }),
        }
    }

    pub fn at(&self, x: f32, z: f32) -> f32 {
        let x = x as f64;
        let z = z as f64;

        let mut h = 0.;

        // let bruff = lerp(self.rough.sample(x, z), -1., 1., 0.9, 1.);

        let mut frq = 0.0005;
        let mut amp = 1500.;
        self.mountains_worleys.iter().for_each(|worley| {
            h += amp * worley.get([x * frq, z * frq]);

            frq *= 1.5;
            amp *= 0.3;
        });

        h as f32
    }
}

struct OctavedNoiseSampler {
    octaves: Vec<SuperSimplex>,
    frequency: f64,
}

impl OctavedNoiseSampler {
    fn new(prng: &mut Prng, num_octaves: usize, frequency: f64) -> Self {
        let mut octaves = Vec::with_capacity(num_octaves);
        for _ in 0..num_octaves {
            octaves.push(SuperSimplex::new(prng.next_u32()));
        }

        Self {
            octaves: octaves,
            frequency: frequency,
        }
    }

    fn sample(&self, x: f64, z: f64) -> f64 {
        let mut h = 0.;
        let mut final_amp = 1.;

        let mut frq = self.frequency;
        let mut amp = 1.;
        for i in 0..self.octaves.len() {
            h += amp * self.octaves[i].get([frq * x, frq * z]);
            final_amp += amp;

            frq *= 2.;
            amp *= 0.5;
        }

        h / final_amp
    }
}

fn worley_distance_function(a: &[f64], b: &[f64]) -> f64 {
    let mut ret = 0.;

    for i in 0..a.len() {
        ret += (b[i] - a[i]).powi(2);
    }

    ret = ret.sqrt();

    smoothstep_in_bounds_only(ret * 2.)
}
