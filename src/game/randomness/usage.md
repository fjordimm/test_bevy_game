
## Overview

`GlobalRng` is the only thing that is seeded from a "truly random" source (see https://docs.rs/bevy_rand/latest/bevy_rand/tutorial/ch04_seeding/index.html#where-to-get-seeds), from `.add_plugins(EntropyPlugin::<WyRand>::default())` in core::plugin.rs.

`GlobalRng` should never be used directly outside of randomness::plugin::startup. In this system, which runs during `Startup`, `GlobalRng` is forked into different `bevy_prng::WyRand` global resources. You can use `GeneralRand`, but it's best to create new Rands (`WyRand` resources forked from `GlobalRng`) as much as possible to fully utilize parallelism (see §Adding New Rands). You can even fork again and attach the `WyRand` to singular Entities (see https://docs.rs/bevy_rand/latest/bevy_rand/tutorial/ch03_components_forking/index.html).

When doing sequential pseudorandom things like procedural generation, you can use the `rand` library if you want more than just `.next_u32()` or `.next_u64()` from the Rands; just use a single `.next_u64()` from some Rand as the seed of a `rand::Rng`.

## Adding New Rands

To add a new Rand that can be accessed as a global resource, // TODO

## Determinism

In core::plugin.rs, you can change the `.add_plugins(EntropyPlugin::<WyRand>::default())` line (see https://docs.rs/bevy_rand/latest/bevy_rand/tutorial/ch04_seeding/index.html). However, although it ensures all the seeds of each Rand will be deterministic, and each sequence of numbers produced by each Rand will be deterministic, where each number is used might be non-deterministic due to threading.
