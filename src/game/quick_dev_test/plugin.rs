use std::time::Duration;

use bevy::{input::mouse::MouseWheel, prelude::*, time::common_conditions::on_timer};
use rand_distr::num_traits::Pow;

use crate::game::{
    core::states::OverallState,
    geometry::dodec::dodec_mesh,
    graphics::primary_shader::plugin::{
        PrimaryShaderMaterial, PrimaryShaderMaterialProps, primary_shader_material,
    },
    playing_state::{
        player::{
            resources::PlayerMovementSettings,
            tags::{CameraForPlayer, PlayerBody},
        },
        sets::{DuringPlayingUnpaused, OnEnterPlaying},
        tags::PlayingStateEntity,
        world::{SeasonOfYear, TimeOfDay},
    },
    util::alrms,
};

pub struct QuickDevTestPlugin;

impl Plugin for QuickDevTestPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                after_a_sec
                    .run_if(on_timer(Duration::from_secs(1)).and(run_once))
            )
            .add_systems(Update,
                scrolling
                    .in_set(DuringPlayingUnpaused::General)
            )
            .add_systems(Update,
                move_player_body_to_cam
                    .in_set(DuringPlayingUnpaused::General)
            )
            .add_systems(OnEnter(OverallState::Playing),
                spawn_some_stuff
                    .in_set(OnEnterPlaying::General)
            )
        ;
    }
}

fn after_a_sec(/* mut gui_scale: ResMut<GuiScale> */) {
    // gui_scale.0 = 5.;
}

fn scrolling(
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mut time_of_day: ResMut<TimeOfDay>,
    mut season_of_year: ResMut<SeasonOfYear>,
    mut movement_settings: ResMut<PlayerMovementSettings>,
) {
    for mouse_wheel_msg in mouse_wheel_reader.read() {
        if keys.pressed(KeyCode::ControlLeft) {
            // Move sun.

            if keys.pressed(KeyCode::AltLeft) {
                season_of_year.0 += 0.03 * mouse_wheel_msg.y;
            } else {
                time_of_day.0 += -0.03 * mouse_wheel_msg.y;
            }
        } else {
            // Change movement speed.

            movement_settings.speed = movement_settings.speed.pow(1. - 0.01 * mouse_wheel_msg.y);

            if movement_settings.speed < 0.75 {
                movement_settings.speed = 0.75;
            }
            if movement_settings.speed > 3072. {
                movement_settings.speed = 3072.;
            }
        }
    }
}

fn move_player_body_to_cam(
    keys: Res<ButtonInput<KeyCode>>,
    player_body_q: Option<Single<&mut Transform, With<PlayerBody>>>,
    camera_q: Option<Single<&Transform, (With<CameraForPlayer>, Without<PlayerBody>)>>,
) {
    if keys.pressed(KeyCode::BracketLeft) {
        if let Some(mut player_body) = alrms!(player_body_q) {
            if let Some(camera) = alrms!(camera_q) {
                player_body.translation.x = camera.translation.x;
                player_body.translation.z = camera.translation.z;
            }
        }
    }
}

fn spawn_some_stuff(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PrimaryShaderMaterial>>,
) {
    commands.spawn((
        PlayingStateEntity,
        Mesh3d(meshes.add(dodec_mesh())),
        MeshMaterial3d(
            materials.add(primary_shader_material(PrimaryShaderMaterialProps {
                texturing_scale: 1.,
            })),
        ),
        Transform::default(),
    ));
}
