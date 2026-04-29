- Add TabGroups to gui stuff
- Mark all the gui stuff with proper tags (MainMenuState or PlayingState)
    - Make some debug tool to make warnings when there are objects without one of these tags
    - Maybe make a thing that automatically marks any newly created objects?
- Maybe make the children argument of gui elements take a vec of plain things instead of a vec of boxes
- Add VSync as an option in user settings. Right now it is disabled (see present_mode in build_bevy_app.rs)
- Switch to the better way of using randomization. See: https://docs.rs/bevy_rand/latest/bevy_rand/. And actually more imporantly: https://docs.rs/bevy_rand/latest/bevy_rand/tutorial/index.html

Currently on:
- Making a debug menu
    - Make an X button for it.
- Instead of Single or Query::single for things that are only supposed to have one instance, use a resource.
- Naming convention change: instead of `thing_query`, use `thing_q`.