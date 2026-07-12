
# Todo List

## Reminders

### General

- Prefer using resources rather than consts for constant arbitrary values.
- Be consistent on when to use the 'Tag' suffix.
- Add systems using proper sets (PlayingStateSet).
- Put more `#[allow(unused)]` where you need them.

### Tests

- Make sure there are no 'entity leaks', i.e. make sure everything gets despawned when exiting states, especially exiting `OverallState::Playing`. The Primary Debug Menu has an entity counter.
- Search project for 'todo'. And search project for 'debug!'.
- Test if everything works correctly when you change the `GuiScale`.

## Currently Working On
- My primary shader.
    - Edge highlighting.
        - Document what values to pass to my new vertex attribute.
- Terrain.

## Should Do At Some Point
- Make dev vs release profiles in Cargo.toml.
- Get rid of the yellow background color.
- Test out when there are multiple floating panels and if you drag the window or drag the resizer over each other.
- Make floating panels move correctly when the window is resized.
- Make the camera cursor movement proportional to the window size.
- When dragging floating panels around, they should order themselves correctly (most recently dragged goes on top).

## Optimizations
- If I don't need uv coords or anything for my primary shader, than try implementing meshes that don't use repeated vertices, while still keeping flat shading.
- Don't repeatedly use commands.entity(...) but use EntityCommands directly, wherever possible.
- When in release profile, the functions in util like `alrms`, `alrmo`, `alrrs`, and `alrro` should all just pass the input through without executing code.

## Features
- Add TabGroups to gui stuff
- Add VSync as an option in user settings. Right now it is disabled (see present_mode in build_bevy_app.rs)
- Shapes
    - Pentagon
    - Dodec
    - Indented Dodec
    - Pentagonal Prism
    - Augmented Pentagonal Prism
