use bevy::prelude::*;
use noise::{NoiseFn, SuperSimplex};
use rand::{Rng, SeedableRng};
use worley_noise::WorleyNoise;

use crate::game::{
    random::Prng,
    util::mathf64::{lerp_remap, sigmoid},
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

    pub fn elevation_at(&self, x: f32, z: f32) -> f32 {
        let x = x as f64;
        let z = z as f64;

        let mut h = 0.0;

        let mountains_placement = {
            let mut val = 0.0;

            val += sigmoid(20.0 * (self.mountains_placement_overall.sample(x, z) - 0.2))
                * (230f64.powf(1.0 - self.mountains_placement_detail.sample(x, z).abs()) / 230.0);

            val
        };
        let mountains_detail = {
            let mut val = 0.0;

            let rough = lerp_remap(self.mountains_rough.sample(x, z), -1.0, 1.0, 0.87, 1.0);

            let mut frq = 0.0007;
            let mut amp = 500.0;
            self.mountains_w.iter().for_each(|worley| {
                val += amp * rough * worley.value_2d(x * frq, z * frq);

                frq *= 1.6;
                amp *= 0.55;
            });

            val
        };
        let mountains = mountains_placement * mountains_detail;
        h += mountains;

        let rivers = {
            let mut val = 0.0;

            let skinniness = 21_000_000.0;
            let river_inp = self.rivers.sample(x, z) + 0.01 * mountains_placement;
            val += 1.0 - 1.0 / (1.0 + skinniness * river_inp.powi(4));

            val
        };
        h = rivers * (h + 35.0);

        h += 12.9 * self.ground_rough.sample(x, z);

        h as f32
    }

    pub fn color_at(&self, x: f32, z: f32) -> Color {
        let steepness = {
            let l = self.elevation_at(x - 0.1, z);
            let r = self.elevation_at(x + 0.1, z);
            let dx = r - l;
            let t = self.elevation_at(x, z - 0.1);
            let b = self.elevation_at(x, z + 0.1);
            let dz = b - t;

            ((dx * dx + dz * dz).sqrt() * 3.5).clamp(0.0, 1.0)
        };

        const GRASS: Srgba = Srgba::new(0.2, 0.6, 0.1, 1.0);
        const ROCK: Srgba = Srgba::new(0.75, 0.75, 0.75, 1.0);

        Color::srgb(
            (1.0 - steepness) * GRASS.red + (steepness) * ROCK.red,
            (1.0 - steepness) * GRASS.green + (steepness) * ROCK.green,
            (1.0 - steepness) * GRASS.blue + (steepness) * ROCK.blue,
        )
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
        let mut h = 0.0;
        let mut final_amp = 1.0;

        let mut frq = self.frequency;
        let mut amp = 1.0;
        for i in 0..self.octaves.len() {
            h += amp * self.octaves[i].get([frq * x, frq * z]);
            final_amp += amp;

            frq *= 2.0;
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
