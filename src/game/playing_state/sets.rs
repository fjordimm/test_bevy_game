use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DuringPlaying {
    General,
    CoordRebasing,
}

pub(super) const DURING_PLAYING_LIST: (DuringPlaying, DuringPlaying) =
    (DuringPlaying::General, DuringPlaying::CoordRebasing);

/// PauseState is Unpaused and GameLoadingState is NotLoading.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DuringPlayingUnpaused;

/// GameLoadingState is NotLoading, but PauseState may be Paused or Unpaused.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DuringPlayingNotLoading;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum OnEnterPlaying {
    PlayingStatePluginUseOnly,
    ResourceSetup, // Any systems running here or before should not assume any resources have been inserted yet.
    General,
}

pub(super) const ON_ENTER_PLAYING_LIST: (OnEnterPlaying, OnEnterPlaying, OnEnterPlaying) = (
    OnEnterPlaying::PlayingStatePluginUseOnly,
    OnEnterPlaying::ResourceSetup,
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
