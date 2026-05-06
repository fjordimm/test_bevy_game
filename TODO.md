- Add TabGroups to gui stuff
- Mark all the gui stuff with proper tags (MainMenuState or PlayingState)
    - Make some debug tool to make warnings when there are objects without one of these tags
    - Maybe make a thing that automatically marks any newly created objects?
    - Actually nvm?
- Maybe make the children argument of gui elements take a vec of plain things instead of a vec of boxes
- Add VSync as an option in user settings. Right now it is disabled (see present_mode in build_bevy_app.rs)
- Switch to the better way of using randomization. See: https://docs.rs/bevy_rand/latest/bevy_rand/. And actually more imporantly: https://docs.rs/bevy_rand/latest/bevy_rand/tutorial/index.html
- Make floating panels move correctly when the window is resized.
- Make the camera cursor movement proportional to the window size.

Currently on:
- Making a debug menu
    - Make an X button for it.
    - Make windows show up in the right order.
- Make gui children be able to take any Entity.
- Figure out / be consistent for when to use `for ___ in &mut ___` vs `for ___ in ___`, especially with queries.
- Figure out when to use `ref mut` vs `mut`.
