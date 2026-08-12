use bevy::prelude::*;
use noise::{NoiseFn, SuperSimplex};
use rand::{Rng, SeedableRng};
use worley_noise::WorleyNoise;

use crate::game::{
    playing_state::terrain::terrain_func_functions::{lerp_remap, sigmoid},
    random::Prng,
};

pub struct TerrainFunc {
    mountains_placement_overall: OctavedNoiseSampler,
    mountains_placement_detail: OctavedNoiseSampler,
    mountains_w: [WorleyNoise; 4],
    mountains_rough: OctavedNoiseSampler,
    rivers: OctavedNoiseSampler,
    ground_rough: OctavedNoiseSampler,
}

impl TerrainFunc {
    pub fn new(seed: [u8; 8]) -> Self {
        let mut prng = Prng::from_seed(seed);

        Self {
            mountains_placement_overall: OctavedNoiseSampler::new(&mut prng, 1, 0.000156),
            mountains_placement_detail: OctavedNoiseSampler::new(&mut prng, 2, 0.000247),
            mountains_w: make_worley_noise_array(&mut prng),
            mountains_rough: OctavedNoiseSampler::new(&mut prng, 5, 0.005),
            rivers: OctavedNoiseSampler::new(&mut prng, 6, 0.00007),
            ground_rough: OctavedNoiseSampler::new(&mut prng, 5, 0.002),
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
        let mountains_detail = {
            let mut val = 0.;

            let rough = lerp_remap(self.mountains_rough.sample(x, z), -1., 1., 0.87, 1.);

            let mut frq = 0.0007;
            let mut amp = 500.;
            self.mountains_w.iter().for_each(|worley| {
                val += amp * rough * worley.value_2d(x * frq, z * frq);

                frq *= 1.6;
                amp *= 0.55;
            });

            val
        };
        let mountains = mountains_placement * mountains_detail;
        h += mountains;

        h += 12.9 * self.ground_rough.sample(x, z);

        let rivers = {
            let mut val = 0.;

            let skinniness = 21_000_000.;
            let river_inp = self.rivers.sample(x, z) + 0.01 * mountains_placement;
            val += 1. - 1. / (1. + skinniness * river_inp.powi(4));

            val
        };
        h = rivers * (h + 35.);

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
