
# Todo List

## Currently Working On
- Water shader.
- Switch back to `1.0` instead of `1.`.
- Terrain.
    - Add hysteresis for changing LODs.
- Texturing?? Texturing scale thing?
- Maybe: instead of using plain `Without<>`s when you need to resolve query conflicts, you should make a type alias for it.

## Reminders

### General

- Add systems using proper sets (DuringPlaying/DuringPlayingUnpaused, OnEnterPlaying, OnExitPlaying).
- Prefer using resources rather than consts for constant arbitrary values.
- Be consistent on when to use the 'Tag' suffix.
- Put more `#[allow(unused)]` where you need them.

### Tests

- Make sure there are no 'entity leaks', i.e. make sure everything gets despawned when exiting states, especially exiting `OverallState::Playing`. The Primary Debug Menu has an entity counter.
- Search project for 'todo'. And search project for 'debug!'.
- Test if everything works correctly when you change the `GuiScale`.
- Make sure the first time you enter `OverallState::Playing` works the exact same as when you exit to the menu and then go back.

## Should Do At Some Point
- Make dev vs release profiles in Cargo.toml.
- Get rid of the yellow background color.
- Test out when there are multiple floating panels and if you drag the window or drag the resizer over each other.
- Make floating panels move correctly when the window is resized.
- Make the camera cursor movement proportional to the window size.
- When dragging floating panels around, they should order themselves correctly (most recently dragged goes on top).
- Get rid of post-processor if not using it.
- Instead of having the Z-Indexing stuff in gui/usage.md, just have a few sub-divs under the gui root which you add things to.

## Optimizations
- If I don't need uv coords or anything for my primary shader, than try implementing meshes that don't use repeated vertices, while still keeping flat shading.
- Don't repeatedly use commands.entity(...) but use EntityCommands directly, wherever possible.
- When in release profile, the functions in util like `alrms`, `alrmo`, `alrrs`, and `alrro` should all just pass the input through without executing code.
- TerrainFunc::at uses f32 for both input and output, but the underlying noise functions use f64, so it's casting back and forth every time. Could you find an implementation of the noise functions that use f32?
- Just pass three f32s for vertex colors, not four. Actually, you should get rid of vertices being individually colored and have like 8 colors or something passed in as a uniform, and each vertex has an index to that color.
- The function that generates the mesh for terrain chunks could be optimized. Most notably, it makes a big 2d array that is then discarded.
- `terrain_mesh::quantized_position` could be optimized. Each call to `quantized_position` in `create_terrain_mesh` computes the same value multiple times.

## Features
- Add TabGroups to gui stuff
- Add VSync as an option in user settings. Right now it is disabled (see present_mode in build_bevy_app.rs)
- Shapes
    - Pentagon
    - Dodec
    - Indented Dodec
    - Pentagonal Prism
    - Augmented Pentagonal Prism
