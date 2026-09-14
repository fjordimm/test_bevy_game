use std::time::Duration;

use avian3d::prelude::*;
use bevy::{input::mouse::MouseWheel, prelude::*, time::common_conditions::on_timer};
use rand::RngExt;
use rand_distr::num_traits::Pow;

use crate::game::{
    geometry::cube::cube_mesh,
    graphics::primary_material::plugin::PrimaryMaterial,
    playing_state::{
        coord_rebasing::world_space_transf,
        environment_light::resources::{SkyRotationS, SkyRotationT},
        player::{resources::PlayerMovementSettings, tags::PlayerBody},
        reusable_materials::ReusableMaterials,
        sets::{DuringPlaying, DuringPlayingUnpaused},
        tags::PlayingStateEntity,
    },
    random::{Prng, rands::GeneralRand},
    util::alrrs,
};

pub struct QuickDevTestPlugin;

impl Plugin for QuickDevTestPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                after_a_sec
                    .run_if(on_timer(Duration::from_secs(1)).and_then(run_once))
            )
            .add_systems(Update,
                scrolling
                    .in_set(DuringPlaying::General)
                    .in_set(DuringPlayingUnpaused)
            )
            .add_systems(Update,
                test_cubes
                    .in_set(DuringPlaying::General)
                    .in_set(DuringPlayingUnpaused)
            )
        ;
    }
}

fn after_a_sec(/* mut gui_scale: ResMut<GuiScale> */) {
    // gui_scale.0 = 5.0;
}

fn scrolling(
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mut time_of_day: ResMut<SkyRotationT>,
    mut season_of_year: ResMut<SkyRotationS>,
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

            movement_settings.freecam_speed = movement_settings
                .freecam_speed
                .pow(1.0 + 0.05 * mouse_wheel_msg.y);

            if movement_settings.freecam_speed < 0.05 {
                movement_settings.freecam_speed = 0.05;
            }
            if movement_settings.freecam_speed > 10_000.0 {
                movement_settings.freecam_speed = 10_000.0;
            }
        }
    }
}

fn test_cubes(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    player_q: Option<Single<&Transform, With<PlayerBody>>>,
    mut rand: Single<&mut Prng, With<GeneralRand>>,
    reusable_materials: Res<ReusableMaterials>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        let player_pos = alrrs!(player_q).translation;

        for _ in 0..100 {
            commands.spawn_scene(bsn! {
                PlayingStateEntity
                world_space_transf(Transform::from_xyz(
                    player_pos.x + rand.random_range(-20.0..20.0),
                    player_pos.y + 30.0 + rand.random_range(-20.0..20.0),
                    player_pos.z + rand.random_range(-20.0..20.0),
                ))
                Mesh3d(asset_value(cube_mesh(default())))
                MeshMaterial3d::<PrimaryMaterial>({ reusable_materials.primary_plain.clone() })
                template_value(RigidBody::Dynamic)
                Collider::cuboid(1.0, 1.0, 1.0)
            });
        }
    }
}
