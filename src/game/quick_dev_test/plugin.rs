use std::time::Duration;

use bevy::{input::mouse::MouseWheel, prelude::*, time::common_conditions::on_timer};

use crate::game::{
    core::states::OverallState,
    geometry::dodec::dodec_mesh,
    graphics::primary_shader::plugin::{
        PrimaryShaderMaterial, PrimaryShaderMaterialProps, primary_shader_material,
    },
    playing_state::{
        sets::DuringPlayingUnpaused,
        tags::PlayingStateEntity,
        world::{SeasonOfYear, TimeOfDay, terrain::plugin::SpawnTerrainChunk},
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

fn spawn_some_stuff(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PrimaryShaderMaterial>>,
    mut stc_messages: MessageWriter<SpawnTerrainChunk>,
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

    stc_messages.write(SpawnTerrainChunk::new(1., 0, 0));
}
