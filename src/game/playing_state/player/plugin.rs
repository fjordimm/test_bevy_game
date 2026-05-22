use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

use crate::game::{
    core::states::{MouseMode, OverallState},
    playing_state::{
        player::{resources::PlayerMovementSettings, tags::CameraForPlayer},
        sets::{DuringPlaying, DuringPlayingUnpaused},
        states::PauseState,
    },
    util::alrms,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .init_resource::<PlayerMovementSettings>()
            .add_systems(OnEnter(PauseState::Unpaused),
                grab_cursor
                    .in_set(DuringPlaying)
            )
            .add_systems(OnExit(PauseState::Unpaused),
                free_cursor
                    .in_set(DuringPlaying)
            )
            .add_systems(OnExit(OverallState::Playing),
                free_cursor
            )
            .add_systems(Update,
                cursor_controls_camera_look
                    .in_set(DuringPlayingUnpaused::General)
                    .run_if(in_state(MouseMode::Grabbed))
            );
    }
}

fn grab_cursor(mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Grabbed);
}

fn free_cursor(mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Free);
}

fn cursor_controls_camera_look(
    window_q: Option<Single<&mut Window, With<PrimaryWindow>>>,
    camera_trans_q: Option<Single<&mut Transform, With<CameraForPlayer>>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    movement_settings: Res<PlayerMovementSettings>,
) {
    if let (Some(window), Some(mut camera_trans)) = (alrms!(window_q), alrms!(camera_trans_q)) {
        mouse_motion.read().for_each(|ev| {
            let (mut yaw, mut pitch, _) = camera_trans.rotation.to_euler(EulerRot::YXZ);
            let window_scale = window.height().min(window.width());

            pitch -= (movement_settings.look_sensitivity * ev.delta.y * window_scale).to_radians();
            yaw -= (movement_settings.look_sensitivity * ev.delta.x * window_scale).to_radians();

            pitch = pitch.clamp(-1.54, 1.54);

            camera_trans.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        });
    }
}
