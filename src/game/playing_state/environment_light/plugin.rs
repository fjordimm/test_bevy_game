use bevy::prelude::*;

use crate::game::{
    core::states::OverallState,
    graphics::global_render_data::resources::GlobalRenderData,
    playing_state::{
        environment_light::{SeasonOfYear, TimeOfDay},
        sets::{DuringPlayingUnpaused, OnEnterPlaying},
        tags::PlayingStateEntity,
    },
    util::{alrms, mathf32::lerp_remap},
};

pub struct EnvironmentLightPlugin;

impl Plugin for EnvironmentLightPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .insert_resource(TimeOfDay(0.))
            .insert_resource(SeasonOfYear(0.))
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(OnEnterPlaying::General)
            )
            .add_systems(Update,
                update_environment_lights
                    .in_set(DuringPlayingUnpaused::General)
            )
        ;
    }
}

const SUNLIGHT_ILLUMINANCE: f32 = 1000.;

const ABOVE_AMBIENT_LIGHT_ILLUMINANCE: f32 = 250.;
const ABOVE_AMBIENT_LIGHT_COLOR: Color = Color::hsv(228., 0.18, 1.);
const BELOW_AMBIENT_LIGHT_ILLUMINANCE: f32 = 100.;
const BELOW_AMBIENT_LIGHT_COLOR: Color = Color::hsv(169., 0.05, 1.);

const SUNLIGHT_Y_LEVEL_OF_MIN: f32 = -0.1;
const AMBIENT_LIGHT_MIN_FACTOR: f32 = 0.05;
const AMBIENT_LIGHT_Y_LEVEL_OF_MIN: f32 = -0.25;

#[derive(Component)]
pub struct SunlightTag;

#[derive(Component)]
pub struct AboveAmbientLightTag;

#[derive(Component)]
pub struct BelowAmbientLightTag;

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
            illuminance: SUNLIGHT_ILLUMINANCE,
            ..default()
        },
        Transform::default().looking_at(vec3(0., -1., 0.), Dir3::Y),
    ));

    // "Ambient" light, coming from four directions (pointed towards the vertices of a tetrahedron).

    // Cool lighting from above:
    commands.spawn((
        PlayingStateEntity,
        AboveAmbientLightTag,
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
        AboveAmbientLightTag,
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
        BelowAmbientLightTag,
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
        BelowAmbientLightTag,
        DirectionalLight {
            color: BELOW_AMBIENT_LIGHT_COLOR,
            shadows_enabled: false,
            illuminance: BELOW_AMBIENT_LIGHT_ILLUMINANCE,
            ..default()
        },
        Transform::default().looking_at(vec3(1., 1., 1.), Dir3::Y),
    ));

    // Reset sun position.

    time_of_day.0 = 0.25;
    season_of_year.0 = 0.0;
}

fn update_environment_lights(
    global_render_data: Res<GlobalRenderData>,
    sunlight_q: Option<Single<(&mut Transform, &mut DirectionalLight), With<SunlightTag>>>,
    mut above_ambient_light_q: Query<
        &mut DirectionalLight,
        (With<AboveAmbientLightTag>, Without<SunlightTag>),
    >,
    mut below_ambient_light_q: Query<
        &mut DirectionalLight,
        (
            With<BelowAmbientLightTag>,
            Without<SunlightTag>,
            Without<AboveAmbientLightTag>,
        ),
    >,
) {
    if let Some(mut sunlight) = alrms!(sunlight_q) {
        sunlight.0.look_at(global_render_data.sun_position, Vec3::Y);
        sunlight.0.rotate_local_y(180.0f32.to_radians());

        sunlight.1.illuminance = lerp_remap(
            global_render_data.sun_position.y,
            SUNLIGHT_Y_LEVEL_OF_MIN,
            1.,
            0.,
            1.,
        )
        .clamp(0., 1.)
            * SUNLIGHT_ILLUMINANCE;
    }

    let ambient_light_based_on_sun = lerp_remap(
        global_render_data
            .sun_position
            .y
            .clamp(AMBIENT_LIGHT_Y_LEVEL_OF_MIN, 1.),
        AMBIENT_LIGHT_Y_LEVEL_OF_MIN,
        1.,
        AMBIENT_LIGHT_MIN_FACTOR,
        1.,
    );

    above_ambient_light_q.iter_mut().for_each(|mut light| {
        light.illuminance = ambient_light_based_on_sun * ABOVE_AMBIENT_LIGHT_ILLUMINANCE;
    });
    below_ambient_light_q.iter_mut().for_each(|mut light| {
        light.illuminance = ambient_light_based_on_sun * BELOW_AMBIENT_LIGHT_ILLUMINANCE;
    });
}
