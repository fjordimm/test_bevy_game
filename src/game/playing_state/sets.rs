use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayingStateOrdering {
    WorldFirst,
    WorldGeneral,
    WorldLast,
    Ui,
}

pub const PLAYING_STATE_ORDERING_ORDER: (
    PlayingStateOrdering,
    PlayingStateOrdering,
    PlayingStateOrdering,
    PlayingStateOrdering,
) = (
    PlayingStateOrdering::WorldFirst,
    PlayingStateOrdering::WorldGeneral,
    PlayingStateOrdering::WorldLast,
    PlayingStateOrdering::Ui,
);
