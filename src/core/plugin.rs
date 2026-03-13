use bevy::prelude::*;

use crate::core::{global_resources::KeyBindings, states::OverallState};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .init_resource::<KeyBindings>()
            .init_state::<OverallState>();
    }
}
