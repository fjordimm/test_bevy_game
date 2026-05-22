use bevy::prelude::*;

use crate::game::{core::states::OverallState, playing_state::sets::DuringPlayingUnpaused};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(DuringPlayingUnpaused::General)
            );
    }
}

fn on_enter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        DirectionalLight {
            color: Color::hsv(0.0, 0.0, 1.0),
            shadows_enabled: false,
            ..default()
        },
        Transform::default().looking_at(Vec3::new(-0.1, -1.0, -0.2), Dir3::Y),
    ));
}
