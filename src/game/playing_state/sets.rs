use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DuringPlaying;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DuringPlayingUnpausedW;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DuringPlayingUnpaused {
    General,
    Ui,
}

pub const DURING_PLAYING_UNPAUSED_LIST: (DuringPlayingUnpaused, DuringPlayingUnpaused) =
    (DuringPlayingUnpaused::General, DuringPlayingUnpaused::Ui);
