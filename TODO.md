
# Todo List

## Currently Working On
- Rework the whole GUI system.
- Finish the debug menu.

## Reminders
- Prefer using resources rather than consts for constant arbitrary values.
- Be consistent on when to use the 'Tag' suffix.
- Mark everything with proper tags (MainMenuState or PlayingState)
    - Make some debug tool to make warnings when there are objects without one of these tags
    - Maybe make a thing that automatically marks any newly created objects?
    - Actually nvm?
    - Actually maybe use something like GameEntity just for PlayingState and nothing for MainMenuState.
- Add systems using proper sets (PlayingStateSet).
- Put more `#[allow(unused)]` where you need them.

## Should Do At Some Point
- Search project for 'TODO'.
- Make a better system for changing the cursor type (right now it's just done locally for the floating panel corner resizer).
- Test out when there are multiple floating panels and if you drag the window or drag the resizer over each other.
- Make floating panels move correctly when the window is resized.
- Make the camera cursor movement proportional to the window size.
- When dragging floating panels around, they should order themselves correctly (most recently dragged goes on top).
- Get rid of the yellow background color.
- Make different GUI sizes.
- Make dev vs release profiles in Cargo.toml.
- Maybe instead of creating all GUI stuff during Startup, you create the elements when they appear and actually delete them when they are gone.
- Check whether everything is despawned when you exit OverallState::Playing.

## Optimizations
- Right now I'm inserting a new CursorIcon to the window entity when changing the cursor type. Should I be removing the old ones?
- Don't use commands.entity(...) but use EntityCommands directly, wherever possible.
- Maybe the warned_ok!() and warned_some!() macros can cease to do anything in the release profile. Especially consider using Single<> instead of Query<>.

## Features
- Add TabGroups to gui stuff
- Add VSync as an option in user settings. Right now it is disabled (see present_mode in build_bevy_app.rs)




