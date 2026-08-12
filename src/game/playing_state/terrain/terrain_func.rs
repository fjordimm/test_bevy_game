use bevy::prelude::*;
use noise::{NoiseFn, SuperSimplex};
use rand::{Rng, SeedableRng};
use worley_noise::WorleyNoise;

use crate::game::{
    playing_state::terrain::terrain_func_functions::{lerp_remap, sigmoid},
    random::Prng,
};

pub struct TerrainFunc {
    rough: OctavedNoiseSampler,
    mountains_placement_overall: OctavedNoiseSampler,
    mountains_placement_detail: OctavedNoiseSampler,
    mountains_w: [WorleyNoise; 4],
}

impl TerrainFunc {
    pub fn new(seed: [u8; 8]) -> Self {
        let mut prng = Prng::from_seed(seed);

        Self {
            rough: OctavedNoiseSampler::new(&mut prng, 5, 0.005),
            mountains_placement_overall: OctavedNoiseSampler::new(&mut prng, 1, 0.000156),
            mountains_placement_detail: OctavedNoiseSampler::new(&mut prng, 2, 0.000247),
            mountains_w: make_worley_noise_array(&mut prng),
        }
    }

    pub fn at(&self, x: f32, z: f32) -> f32 {
        let x = x as f64;
        let z = z as f64;

        let mut h = 0.;

        let mountains_placement = {
            let mut val = 0.;

            val += sigmoid(20. * (self.mountains_placement_overall.sample(x, z) - 0.2))
                * (230f64.powf(1. - self.mountains_placement_detail.sample(x, z).abs()) / 230.);

            val
        };
        let mountains = {
            let mut val = 0.;

            let texture = lerp_remap(self.rough.sample(x, z), -1., 1., 0.92, 1.);

            let mut frq = 0.0007;
            let mut amp = 500.;
            self.mountains_w.iter().for_each(|worley| {
                val += amp * texture * worley.value_2d(x * frq, z * frq);

                frq *= 1.6;
                amp *= 0.55;
            });

            val
        };
        h += mountains_placement * mountains;

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

fn make_worley_noise_array<const N: usize>(prng: &mut Prng) -> [WorleyNoise; N] {
    std::array::from_fn(|_| {
        let mut worley = WorleyNoise::new();
        worley.permutate_seeded(
            WorleyNoise::DEFAULT_PERMUTATION_BITS,
            prng.next_u64() as u128,
        );
        worley
    })
}
