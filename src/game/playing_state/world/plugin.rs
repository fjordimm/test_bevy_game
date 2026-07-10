use bevy::prelude::*;
use bevy_inspector_egui::egui::lerp;

use crate::game::{
    core::states::OverallState,
    playing_state::{
        player::tags::CameraForPlayer,
        sets::DuringPlayingUnpaused,
        skybox::ComputedSkyboxValues,
        tags::PlayingStateEntity,
        world::{SeasonOfYear, TimeOfDay},
    },
    util::alrms,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .insert_resource(TimeOfDay(0.))
            .insert_resource(SeasonOfYear(0.))
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

const ABOVE_AMBIENT_LIGHT_ILLUMINANCE: f32 = 300.;
const ABOVE_AMBIENT_LIGHT_COLOR: Color = Color::hsv(228., 0.18, 1.);
const BELOW_AMBIENT_LIGHT_ILLUMINANCE: f32 = 150.;
const BELOW_AMBIENT_LIGHT_COLOR: Color = Color::hsv(169., 0.05, 1.);

fn on_enter(
    mut commands: Commands,
    mut time_of_day: ResMut<TimeOfDay>,
    mut season_of_year: ResMut<SeasonOfYear>,
) {
    // Sunlight.

    commands.spawn((
        PlayingStateEntity,
        SunlightTag,
        DirectionalLight {
            color: Color::hsv(0., 0., 1.),
            shadows_enabled: false,
            ..default()
        },
        Transform::default().looking_at(vec3(0., -1., 0.), Dir3::Y),
    ));

    // "Ambient" light, coming from four directions (pointed towards the vertices of a tetrahedron).

    // Cool lighting from above:
    commands.spawn((
        PlayingStateEntity,
        DirectionalLight {
            color: ABOVE_AMBIENT_LIGHT_COLOR,
            shadows_enabled: false,
            illuminance: ABOVE_AMBIENT_LIGHT_ILLUMINANCE,
            ..default()
        },
        Transform::default().looking_at(vec3(1., -1., -1.), Dir3::Y),
    ));
    commands.spawn((
        PlayingStateEntity,
        DirectionalLight {
            color: ABOVE_AMBIENT_LIGHT_COLOR,
            shadows_enabled: false,
            illuminance: ABOVE_AMBIENT_LIGHT_ILLUMINANCE,
            ..default()
        },
        Transform::default().looking_at(vec3(-1., -1., 1.), Dir3::Y),
    ));
    // Warm(er) lighting from below:
    commands.spawn((
        PlayingStateEntity,
        DirectionalLight {
            color: BELOW_AMBIENT_LIGHT_COLOR,
            shadows_enabled: false,
            illuminance: BELOW_AMBIENT_LIGHT_ILLUMINANCE,
            ..default()
        },
        Transform::default().looking_at(vec3(-1., 1., -1.), Dir3::Y),
    ));
    commands.spawn((
        PlayingStateEntity,
        DirectionalLight {
            color: BELOW_AMBIENT_LIGHT_COLOR,
            shadows_enabled: false,
            illuminance: BELOW_AMBIENT_LIGHT_ILLUMINANCE,
            ..default()
        },
        Transform::default().looking_at(vec3(1., 1., 1.), Dir3::Y),
    ));

    // Reset sun position.

    time_of_day.0 = 0.5;
    season_of_year.0 = 0.0;
}

fn update_sunlight(
    sunlight_q: Option<Single<(&mut Transform, &mut DirectionalLight), With<SunlightTag>>>,
    computed_skybox_values: Res<ComputedSkyboxValues>,
) {
    if let Some(mut sunlight) = alrms!(sunlight_q) {
        sunlight
            .0
            .look_at(computed_skybox_values.sun_position, Vec3::Y);
        sunlight.0.rotate_local_y(180.0f32.to_radians());

        sunlight.1.illuminance = f32::max(0., computed_skybox_values.sun_position.y)
            * light_consts::lux::AMBIENT_DAYLIGHT;
    }
}
