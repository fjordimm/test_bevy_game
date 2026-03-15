use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum OverallState {
    #[default]
    MainMenu,
    Playing,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum MouseMode {
    #[default]
    Free,
    Grabbed,
}
