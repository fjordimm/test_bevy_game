use crate::game::randomness::{Prng, list_of_rands::list_of_rands};
use bevy::prelude::*;
use bevy_rand::prelude::ForkableRng;

macro_rules! make_components {
    ( $( $name:ident ),* $(,)? ) => {
        $(
            #[derive(Component)]
            pub struct $name;
        )*
    };
}

list_of_rands!(make_components);

macro_rules! make_spawn_rands_fn {
    ( $( $name:ident ),* $(,)? ) => {
        pub(super) fn spawn_rands(commands: &mut Commands, global_rng: &mut Prng) {
            $(
                commands.spawn(($name, global_rng.fork_rng()));
            )*
        }
    };
}

list_of_rands!(make_spawn_rands_fn);
