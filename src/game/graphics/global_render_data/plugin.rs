use bevy::{prelude::*, render::storage::ShaderStorageBuffer};

use crate::game::{
    graphics::global_render_data::resources::{GlobalRenderData, GlobalRenderDataHandle},
    util::alrrs,
};

pub struct GlobalRenderDataPlugin;

impl Plugin for GlobalRenderDataPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Startup,
                create_global_render_data
            )
            .add_systems(Update,
                update_global_render_data
                    .run_if(resource_exists_and_changed::<GlobalRenderData>)
            )
        ;
    }
}

fn create_global_render_data(
    mut commands: Commands,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    let data = GlobalRenderData::default();

    commands.insert_resource(data.clone());
    commands.insert_resource(GlobalRenderDataHandle(
        buffers.add(ShaderStorageBuffer::from(data.clone())),
    ));
}

fn update_global_render_data(
    data: Res<GlobalRenderData>,
    handle: Res<GlobalRenderDataHandle>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    alrrs!(buffers.get_mut(handle.0.id())).set_data((*data).clone());
}
