use bevy::prelude::*;

use crate::game::{core::states::OverallState, playing_state::sets::PlayingStateOrdering};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(PlayingStateOrdering::WorldOnEnter)
            )
            .add_systems(OnExit(OverallState::Playing),
                on_exit
                    .in_set(PlayingStateOrdering::WorldOnExit)
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
        Transform::from_xyz(0.0, 0.5, -5.0),
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn on_exit() {}
