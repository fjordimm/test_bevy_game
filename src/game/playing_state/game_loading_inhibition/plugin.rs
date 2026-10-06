use bevy::prelude::*;

use crate::game::{
    core::states::OverallState,
    playing_state::{
        game_loading_inhibition::resources::GameLoadingInhibition,
        sets::{DuringPlaying, OnEnterPlaying},
        states::GameLoadingState,
    },
};

pub struct GameLoadingInhibitionPlugin;

impl Plugin for GameLoadingInhibitionPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(OnEnterPlaying::ResourceSetup)
            )
            .add_systems(Update,
                update_game_loading_state
                    .in_set(DuringPlaying::Final)
            )
        ;
    }
}

fn on_enter(mut commands: Commands) {
    commands.insert_resource(GameLoadingInhibition::new());
}

// TODO: This could be optimized. It is changing GameLoadingState every frame.
fn update_game_loading_state(
    game_loading_inhibition: Res<GameLoadingInhibition>,
    mut next_game_loading_state: ResMut<NextState<GameLoadingState>>,
) {
    if game_loading_inhibition.num_inhibitors() == 0 {
        next_game_loading_state.set(GameLoadingState::NotLoading);
    } else {
        next_game_loading_state.set(GameLoadingState::Loading);
    }
}
