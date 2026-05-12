use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::{global::GlobalRng, traits::ForkableRng};

use crate::game::{core::sets::GlobalStartupOrdering, randomness::rands::{DumbRand, DumbRandTag, GeneralRand, GeneralRandTag}, util::warned_ok};

pub struct RandomnessPlugin;

impl Plugin for RandomnessPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Startup, 
                startup
                    .in_set(GlobalStartupOrdering::RandomnessUseOnly)
            );
    }
}

pub fn startup(mut commands: Commands, mut global_rng_q: Query<&mut WyRand, With<GlobalRng>>) {
    if let Some(mut global_rng) = warned_ok!(global_rng_q.single_mut()) {
        {
            let thing = commands.spawn((GeneralRandTag, global_rng.fork_rng())).id();
            commands.insert_resource(GeneralRand(thing));
        }

        {
            let thing = commands.spawn((DumbRandTag, global_rng.fork_rng())).id();
            commands.insert_resource(DumbRand(thing));
        }
    }
}
