use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DuringPlaying;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DuringPlayingUnpaused {
    General,
    Ui,
}

pub(super) const DURING_PLAYING_UNPAUSED_LIST: (DuringPlayingUnpaused, DuringPlayingUnpaused) =
    (DuringPlayingUnpaused::General, DuringPlayingUnpaused::Ui);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum OnEnterPlaying {
    PlayingStatePluginUseOnly,
    Setup, // Any systems in this set should not assume any resources have been inserted yet.
    General,
}

pub(super) const ON_ENTER_PLAYING_LIST: (OnEnterPlaying, OnEnterPlaying, OnEnterPlaying) = (
    OnEnterPlaying::PlayingStatePluginUseOnly,
    OnEnterPlaying::Setup,
    OnEnterPlaying::General,
);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum OnExitPlaying {
    General,
    PlayingStatePluginUseOnly,
}

pub(super) const ON_EXIT_PLAYING_LIST: (OnExitPlaying, OnExitPlaying) = (
    OnExitPlaying::General,
    OnExitPlaying::PlayingStatePluginUseOnly,
);
