use bevy::prelude::*;
use bevy_inspector_egui::egui::lerp;

use crate::game::{
    core::states::OverallState,
    playing_state::{
        player::tags::CameraForPlayer, sets::DuringPlayingUnpaused, skybox::ComputedSkyboxValues,
        tags::PlayingStateEntity, world::TimeOfDay,
    },
    util::alrms,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .insert_resource(TimeOfDay(0.0))
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(DuringPlayingUnpaused::General)
            )
            .add_systems(Update,
                update_sunlight
                    .in_set(DuringPlayingUnpaused::General)
            )
        ;
    }
}

#[derive(Component)]
pub struct SunlightTag;

fn on_enter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut time_of_day: ResMut<TimeOfDay>,
) {
    commands.spawn((
        PlayingStateEntity,
        SunlightTag,
        DirectionalLight {
            color: Color::hsv(0.0, 0.0, 1.0),
            shadows_enabled: false,
            ..default()
        },
        Transform::default().looking_at(Vec3::new(-0.1, -1.0, -0.2), Dir3::Y),
    ));
    commands.spawn((
        PlayingStateEntity,
        Mesh3d(meshes.add(Cuboid::new(10_000.0, 1.0, 10_000.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::hsv(113.0, 0.57, 0.55),
            perceptual_roughness: 0.0,
            reflectance: 0.0,
            ..default()
        })),
        Transform::from_xyz(0.0, -1.0, 0.0),
    ));
    commands.spawn((
        PlayingStateEntity,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::hsv(0.0, 1.0, 1.0))),
        Transform::default(),
    ));

    time_of_day.0 = 0.5;
}

fn update_sunlight(
    sunlight_q: Option<Single<(&mut Transform, &mut DirectionalLight), With<SunlightTag>>>,
    ambient_light_q: Option<Single<&mut AmbientLight, With<CameraForPlayer>>>,
    computed_skybox_values: Res<ComputedSkyboxValues>,
) {
    if let Some(mut sunlight) = alrms!(sunlight_q) {
        sunlight
            .0
            .look_at(computed_skybox_values.sun_position, Vec3::Y);
        sunlight.0.rotate_local_y(180.0f32.to_radians());

        sunlight.1.illuminance = f32::max(0.0, computed_skybox_values.sun_position.y)
            * light_consts::lux::AMBIENT_DAYLIGHT;
    }

    if let Some(mut ambient_light) = alrms!(ambient_light_q) {
        ambient_light.brightness = lerp(
            30.0..=80.0,
            f32::max(0.0, computed_skybox_values.sun_position.y),
        );
    }
}
