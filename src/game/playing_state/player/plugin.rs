use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

use crate::game::{
    core::states::{MouseMode, OverallState},
    playing_state::{
        player::{resources::PlayerMovementSettings, tags::CameraForPlayer},
        sets::PlayingStateOrdering,
        states::PauseState,
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .init_resource::<PlayerMovementSettings>()
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(PlayingStateOrdering::WorldOnEnter)
            )
            .add_systems(OnExit(OverallState::Playing),
                on_exit
                    .in_set(PlayingStateOrdering::WorldOnExit)
            )
            .add_systems(OnEnter(PauseState::Unpaused),
                on_enter_unpaused
                    .run_if(in_state(OverallState::Playing))
                    .in_set(PlayingStateOrdering::Ui)
            )
            .add_systems(OnExit(PauseState::Unpaused),
                on_exit_unpaused
                    .run_if(in_state(OverallState::Playing))
                    .in_set(PlayingStateOrdering::Ui)
            )
            .add_systems(Update,
                cursor_controls_camera_look
                    .run_if(in_state(OverallState::Playing))
                    .in_set(PlayingStateOrdering::WorldPlayer)
                    .run_if(in_state(MouseMode::Grabbed))
            )
            .add_systems(Update,
                funny1
                    .run_if(in_state(OverallState::Playing))
                    .in_set(PlayingStateOrdering::Ui)
            );
    }
}

fn funny1(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_overall_state: ResMut<NextState<OverallState>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        next_overall_state.set(OverallState::MainMenu);
    }
}

fn on_enter() {}

fn on_exit() {}

fn on_enter_unpaused(mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Grabbed);
}

fn on_exit_unpaused(mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Free);
}

fn cursor_controls_camera_look(
    movement_settings: Res<PlayerMovementSettings>,
    window: Single<&mut Window, With<PrimaryWindow>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut camera_trans: Single<&mut Transform, With<CameraForPlayer>>,
) {
    for ev in mouse_motion.read() {
        let (mut yaw, mut pitch, _) = camera_trans.rotation.to_euler(EulerRot::YXZ);
        let window_scale = window.height().min(window.width());

        pitch -= (movement_settings.look_sensitivity * ev.delta.y * window_scale).to_radians();
        yaw -= (movement_settings.look_sensitivity * ev.delta.x * window_scale).to_radians();

        pitch = pitch.clamp(-1.54, 1.54);

        camera_trans.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }
}
