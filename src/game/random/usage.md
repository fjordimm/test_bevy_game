# Usage

## Overview

`GlobalRng` is the only thing that is seeded from a "truly random" source (see https://docs.rs/bevy_rand/latest/bevy_rand/tutorial/ch04_seeding/index.html#where-to-get-seeds), which is declared at `.add_plugins(EntropyPlugin::<Prng>::default())` in `core::plugin`. `GlobalRng` should never be used directly.

Instead, there is a set of "Rand"s created during `Startup` (each with an attached `Prng` forked from `GlobalRng`) that you can use (see §Using Rands). You can use `GeneralRand` anywhere, but it's best to define another Rand (see §Defining New Rands) to fully utilize parallelism. You can even further fork from one of the Rands' `Prng` and attach it to individual entities (see https://docs.rs/bevy_rand/latest/bevy_rand/tutorial/ch03_components_forking/index.html).

## Defining New Rands

To define a new Rand, simply append the name of it to `random::list_of_rands`. It should have the suffix "Rand".

## Using Rands

Use the following parameter in your system: `Single<&mut Prng, With<___Rand>>`. You can use `.next_u32()`, `.next_u64()`, or functions from `rand::RngExt` from the Prng.

Common functions: `.random::<u32>()`, `.random_range(0.0..13.0)`, `.sample(alrro!(Normal::new(0.0, 100.0)))`.

You can fork the Prng by using `.fork_rng()`.

## Determinism

To give a fixed seed to `GlobalRng`, you can change the line `.add_plugins(EntropyPlugin::<Prng>::default())` to `.add_plugins(EntropyPlugin::<Prng>::with_seed([___; 8]))` in `core::plugin` (see https://docs.rs/bevy_rand/latest/bevy_rand/tutorial/ch04_seeding/index.html). However, although this ensures that all the seeds of each `Prng` will be deterministic, and that the sequences of values produced by each `Prng` will be deterministic, where each value is used might be non-deterministic if the order that the `Prng`s are created is non-deterministic (e.g. threading).
