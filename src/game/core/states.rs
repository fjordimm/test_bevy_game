use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum OverallState {
    #[default]
    Limbo,
    MainMenu,
    EnteringPlaying, // Most systems shouldn't run during this. It can be used by systems that need to set up some resource or something before OverallState::Playing begins, but you have to make sure not to use resources or things that might not exist yet.
    Playing, // Never directly set the state to this; instead set it to EnteringPlaying.
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum MouseMode {
    #[default]
    Free,
    Grabbed,
}
