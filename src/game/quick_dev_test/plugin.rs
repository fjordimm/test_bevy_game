use std::time::Duration;

use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    time::common_conditions::on_timer,
};
use bytemuck::cast_slice;

use crate::game::{
    core::states::OverallState,
    geometry::dodec::dodec_mesh,
    graphics::primary_shader::plugin::{
        PrimaryShaderMaterial, PrimaryShaderMaterialProps, primary_shader_material,
    },
    playing_state::{
        sets::DuringPlayingUnpaused,
        tags::PlayingStateEntity,
        world::{SeasonOfYear, TimeOfDay},
    },
    util::alrmo,
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
            .add_systems(Update,
                move_doodad
                    .in_set(DuringPlayingUnpaused::General)
            )
        ;
    }
}

fn after_a_sec(/* mut gui_scale: ResMut<GuiScale> */) {
    // gui_scale.0 = 5.;
}

fn rotate_sun(
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mut time_of_day: ResMut<TimeOfDay>,
    mut season_of_year: ResMut<SeasonOfYear>,
) {
    for mouse_wheel_msg in mouse_wheel_reader.read() {
        if keys.pressed(KeyCode::ControlLeft) {
            season_of_year.0 += 0.03 * mouse_wheel_msg.y;
        } else {
            time_of_day.0 += -0.03 * mouse_wheel_msg.y;
        }
    }
}

#[derive(Component)]
struct Doodad;

fn spawn_some_stuff(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PrimaryShaderMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn((
        PlayingStateEntity,
        Mesh3d(meshes.add(dodec_mesh())),
        MeshMaterial3d(
            materials.add(primary_shader_material(PrimaryShaderMaterialProps {
                test_tex: images.add(make_test_tex()),
            })),
        ),
        Transform::default(),
        Doodad,
    ));
}

fn move_doodad(/*mut doodad_q: Query<&mut Transform, With<Doodad>>*/) {
    // doodad_q.iter_mut().for_each(|mut transf| {
    //     transf.translation += vec3(0.0005, 0.0, 0.0);
    // });
}

fn make_test_tex() -> Image {
    const TEX_SIZE: u32 = 32; // IMPORTANT: This must match in the Python code that generates the binary file.
    const FILE_PATH: &str = "assets/generated/textures/test_tex.bin";

    if let Some(data) = alrmo!(std::fs::read(FILE_PATH)) {
        Image::new(
            Extent3d {
                width: TEX_SIZE,
                height: TEX_SIZE,
                depth_or_array_layers: TEX_SIZE,
            },
            TextureDimension::D3,
            cast_slice(&data).to_vec(),
            TextureFormat::Rgba16Float,
            default(),
        )
    } else {
        panic!()
    }
}
