use bevy_prng::WyRand;

mod plugin;

pub mod list_of_rands;
pub mod rands;
pub use plugin::RandomnessPlugin;

pub type Prng = WyRand;
