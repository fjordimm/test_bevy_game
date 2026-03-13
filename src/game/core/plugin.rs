use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};

use crate::game::core::{global_resources::KeyBindings, states::OverallState};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .init_resource::<KeyBindings>()
            .init_state::<OverallState>();
    }
}
