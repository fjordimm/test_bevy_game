use bevy_prng::WyRand;
use rand::Rng;

pub mod list_of_rands;
pub mod plugin;
pub mod rands;

pub type Prng = WyRand;

pub fn get_rng_seed(prng: &mut Prng) -> u64 {
    prng.next_u64()
}
