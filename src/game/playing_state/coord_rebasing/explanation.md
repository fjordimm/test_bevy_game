# Explanation

## Mechanics

This is here mainly to combat a rendering issue with floating point imprecision from travelling out further, but it also helps with floating point imprecision in general.

Every few seconds or so, a rebase will be performed, wherein the player will be moved to the origin, and everything else (`WorldSpaceEntity`s, which almost all `Transform`s should be) will be moved relatively. 

All `WorldSpaceEntity`s, upon creation, will be assigned a world space position with double precision. This will be updated on every rebase.

## What It Means

You can use `Transform`s as you would regularly, as long as you don't rely on stationary objects to have a consistent position across frames. If you want the true world space position of something, then use `to_world_space` on the `Transform`'s translation.
