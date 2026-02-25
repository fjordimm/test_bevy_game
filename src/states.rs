use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    Playing,
}

#[derive(Resource, Default)]
pub struct PauseState {
    pub paused: bool,
}
