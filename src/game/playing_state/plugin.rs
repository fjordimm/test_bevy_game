use bevy::prelude::*;

use crate::game::{
    core::{resources::KeyBindings, states::OverallState},
    playing_state::{
        pause_menu::plugin::PauseMenuPlugin,
        player::{plugin::PlayerPlugin, tags::CameraForPlayer},
        sets::{
            DURING_PLAYING_UNPAUSED_LIST, DuringPlaying, DuringPlayingUnpaused,
            DuringPlayingUnpausedW,
        },
        skybox::plugin::SkyboxPlugin,
        states::PauseState,
        tags::PlayingStateEntity,
        world::plugin::WorldPlugin,
    },
};

pub struct PlayingStatePlugin;

impl Plugin for PlayingStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .configure_sets(Update, (
                DuringPlaying
                    .run_if(in_state(OverallState::Playing)),
                DuringPlayingUnpausedW
                    .in_set(DuringPlaying)
                    .run_if(in_state(PauseState::Unpaused)),
                DURING_PLAYING_UNPAUSED_LIST
                    .in_set(DuringPlayingUnpausedW),
                DURING_PLAYING_UNPAUSED_LIST.chain(),
            ))
            .init_state::<PauseState>()
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(DuringPlayingUnpausedW)
                    .before(<[DuringPlayingUnpaused; _]>::from(DURING_PLAYING_UNPAUSED_LIST).first().unwrap().clone())
            )
            .add_systems(OnExit(OverallState::Playing),
                on_exit
                    .in_set(DuringPlayingUnpausedW)
                    .after(<[DuringPlayingUnpaused; _]>::from(DURING_PLAYING_UNPAUSED_LIST).last().unwrap().clone())
            )
            .add_systems(Update,
                toggle_pause
                    .in_set(DuringPlaying)
            )
            .add_plugins(PauseMenuPlugin)
            .add_plugins(SkyboxPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin);
    }
}

fn on_enter(mut commands: Commands, mut next_pause_state: ResMut<NextState<PauseState>>) {
    next_pause_state.set(PauseState::Unpaused);

    commands.spawn((
        PlayingStateEntity,
        CameraForPlayer,
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 7.0).looking_at(-Vec3::Z, Vec3::Y),
    ));
}

fn on_exit(mut commands: Commands, all_entities_q: Query<Entity, With<PlayingStateEntity>>) {
    all_entities_q.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    pause_state: Res<State<PauseState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
) {
    if keys.just_pressed(key_bindings.pause) {
        next_pause_state.set(match pause_state.get() {
            PauseState::Unpaused => PauseState::Paused,
            PauseState::Paused => PauseState::Unpaused,
        });
    }
}
