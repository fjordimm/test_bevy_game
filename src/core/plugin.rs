use crate::core::{sets::GameSet, states::GameState};
use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .init_state::<GameState>()
            .configure_sets(
                Update,
                (
                    GameSet::Input,
                    GameSet::Movement,
                    GameSet::Physics,
                    GameSet::World,
                )
                    .chain(),
            );
    }
}
