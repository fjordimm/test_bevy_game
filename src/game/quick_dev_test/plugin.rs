use std::time::Duration;

use bevy::{input::mouse::MouseWheel, prelude::*, time::common_conditions::on_timer};

use crate::game::{
    core::states::OverallState,
    geometry::cuboid::create_cuboid_mesh,
    playing_state::{
        primary_shader::plugin::{
            PrimaryShaderMaterial, PrimaryShaderMaterialProps, primary_shader_material,
        },
        sets::DuringPlayingUnpaused,
        tags::PlayingStateEntity,
        world::TimeOfDay,
    },
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
                rotate_sun
                    .in_set(DuringPlayingUnpaused::General)
            )
            .add_systems(OnEnter(OverallState::Playing),
                spawn_some_stuff
                    .in_set(DuringPlayingUnpaused::General)
            )
        ;
    }
}

fn after_a_sec(/* mut gui_scale: ResMut<GuiScale> */) {
    // gui_scale.0 = 5.;
}

fn rotate_sun(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mut time_of_day: ResMut<TimeOfDay>,
) {
    for mouse_wheel_msg in mouse_wheel_reader.read() {
        time_of_day.0 += -0.03 * mouse_wheel_msg.y;
    }
}

fn spawn_some_stuff(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PrimaryShaderMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let test_texture: Handle<Image> = asset_server.load("misc/thing.png");

    commands.spawn((
        PlayingStateEntity,
        Mesh3d(meshes.add(create_cuboid_mesh())),
        MeshMaterial3d(
            materials.add(primary_shader_material(PrimaryShaderMaterialProps {
                base_color: Color::hsv(0., 0., 1.),
                edge_color: Color::hsv(0., 0., 1.),
                texture: Some(test_texture),
            })),
        ),
        Transform::default(),
    ));

    // commands.spawn((
    //     PlayingStateEntity,
    //     Mesh3d(meshes.add(Tetrahedron::default())),
    //     MeshMaterial3d(materials.add(Color::hsv(270., 1., 1.))),
    //     Transform::from_xyz(0., 2., 0.),
    // ));
}
