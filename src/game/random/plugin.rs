use bevy::prelude::*;
use bevy_rand::global::GlobalRng;

use crate::game::{
    core::sets::GlobalStartupOrdering,
    random::{Prng, rands},
    util::alrmo,
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

fn startup(mut commands: Commands, mut global_rng_q: Query<&mut Prng, With<GlobalRng>>) {
    if let Some(mut global_rng) = alrmo!(global_rng_q.single_mut()) {
        rands::spawn_rands(&mut commands, &mut global_rng);
    }
}
