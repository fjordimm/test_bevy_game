use bevy::prelude::*;

use crate::game::{
    core::states::OverallState,
    playing_state::{SunPosition, sets::DuringPlayingUnpaused, tags::PlayingStateEntity},
    util::alrms,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(DuringPlayingUnpaused::General)
            )
            .add_systems(Update,
                update_sunlight
                    .in_set(DuringPlayingUnpaused::General)
            );
    }
}

#[derive(Component)]
pub struct SunlightTag;

fn on_enter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
}

fn update_sunlight(
    sun_position: Res<SunPosition>,
    sunlight_transf_q: Option<Single<&mut Transform, With<SunlightTag>>>,
) {
    if let Some(mut sunlight_transf) = alrms!(sunlight_transf_q) {
        sunlight_transf.look_at(sun_position.0, Vec3::Y);
        sunlight_transf.rotate_local_y(180.0f32.to_radians());
    }
}
