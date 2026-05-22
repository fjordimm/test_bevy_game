use bevy::prelude::*;

use crate::game::{core::states::OverallState, playing_state::sets::PlayingStateOrdering};

pub struct QuickDevTestPlugin;

impl Plugin for QuickDevTestPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::MainMenu), on_enter_main_menu_state)
            .add_systems(OnEnter(OverallState::Playing),
                on_enter_playing_state
                    .in_set(PlayingStateOrdering::WorldGeneral)
            );
    }
}

fn on_enter_main_menu_state() {}

fn on_enter_playing_state() {}
