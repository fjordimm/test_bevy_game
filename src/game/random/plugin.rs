use bevy::prelude::*;
use bevy_rand::global::GlobalRng;

use crate::game::{
    core::sets::GlobalStartupOrdering,
    random::{Prng, rands},
    util::alrms,
};

pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app.add_systems(
            Startup,
            startup.in_set(GlobalStartupOrdering::RandomUseOnly),
        );
    }
}

fn startup(mut commands: Commands, global_rng_q: Option<Single<&mut Prng, With<GlobalRng>>>) {
    if let Some(mut global_rng) = alrms!(global_rng_q) {
        rands::spawn_rands(&mut commands, &mut global_rng);
    }
}
