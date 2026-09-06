use bevy::{
    prelude::*,
    render::{extract_resource::ExtractResourcePlugin, storage::ShaderStorageBuffer},
};

use crate::game::{
    graphics::global_render_data::resources::{GlobalRenderData, GlobalRenderDataHandle},
    playing_state::sets::DuringPlayingUnpaused,
    util::alrrs,
};

pub struct GlobalRenderDataPlugin;

impl Plugin for GlobalRenderDataPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(ExtractResourcePlugin::<GlobalRenderDataHandle>::default())
            .add_systems(Startup,
                create_global_render_data_resources
            )
            .add_systems(Update,
                update_global_render_data_handle
                    .run_if(resource_exists_and_changed::<GlobalRenderData>)
            )
            .add_systems(Update,
                update_some_data
                    .in_set(DuringPlayingUnpaused::General)
            )
        ;
    }
}

fn create_global_render_data_resources(
    mut commands: Commands,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    let data = GlobalRenderData::default();

    commands.insert_resource(data.clone());
    commands.insert_resource(GlobalRenderDataHandle(
        buffers.add(ShaderStorageBuffer::from(data.clone())),
    ));
}

fn update_global_render_data_handle(
    data: Res<GlobalRenderData>,
    handle: Res<GlobalRenderDataHandle>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    alrrs!(buffers.get_mut(handle.0.id())).set_data((*data).clone());
}

// This won't update all the data in GlobalRenderData.
// For example, `playing_state::skybox::plugin` is the one who updates `sun_position` and `sky_rotation_inv`.
fn update_some_data(mut data: ResMut<GlobalRenderData>, time: Res<Time>) {
    data.time_elapsed += time.delta_secs();
}
