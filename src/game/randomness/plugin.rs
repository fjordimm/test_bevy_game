use bevy::prelude::*;
use bevy_rand::global::GlobalRng;

use crate::game::{
    core::sets::GlobalStartupOrdering,
    randomness::{Prng, rands},
    util::warned_ok,
};

pub struct RandomnessPlugin;

impl Plugin for RandomnessPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app.add_systems(
            Startup,
            startup.in_set(GlobalStartupOrdering::RandomnessUseOnly),
        );
    }
}

pub fn startup(mut commands: Commands, mut global_rng_q: Query<&mut Prng, With<GlobalRng>>) {
    if let Some(mut global_rng) = warned_ok!(global_rng_q.single_mut()) {
        rands::spawn_rands(&mut commands, &mut global_rng);
    }
}
