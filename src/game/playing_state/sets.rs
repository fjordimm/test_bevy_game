use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayingStateOrdering {
    WorldOnEnter,
    WorldPlayer,
    World,
    WorldOnExit,
    Physics,
    Ui,
}

pub const PLAYING_STATE_SET_ORDER: (
    PlayingStateOrdering,
    PlayingStateOrdering,
    PlayingStateOrdering,
    PlayingStateOrdering,
    PlayingStateOrdering,
    // PlayingStateSet,
) = (
    PlayingStateOrdering::WorldOnEnter,
    PlayingStateOrdering::WorldPlayer,
    PlayingStateOrdering::World,
    PlayingStateOrdering::WorldOnExit,
    // PlayingStateSet::Physics,
    PlayingStateOrdering::Ui,
);
