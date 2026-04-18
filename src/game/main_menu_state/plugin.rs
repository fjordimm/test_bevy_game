use bevy::prelude::*;

use crate::game::{
    core::states::{MouseMode, OverallState},
    gui::{GuiNode, gui_root_template},
    main_menu_state::{
        main_menu_gui::{self, MainMenuGuiPlugin},
        tags::MainMenuStateEntity,
    },
};

pub struct MainMenuStatePlugin;

impl Plugin for MainMenuStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::MainMenu), on_enter)
            .add_systems(OnExit(OverallState::MainMenu), on_exit)
            .add_plugins(MainMenuGuiPlugin);
    }
}

#[derive(Resource)]
pub struct MainMenuStateGuiRoot(pub Entity);

fn on_enter(mut commands: Commands, mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Free);

    commands.spawn((MainMenuStateEntity, Camera2d::default()));

    let gui_root = commands.spawn(gui_root_template()).id();
    commands.insert_resource(MainMenuStateGuiRoot(gui_root));

    let menu_gui = main_menu_gui::make_main_menu_gui().spawn(&mut commands, Some(gui_root));
    commands.entity(menu_gui).insert(MainMenuStateEntity);
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<MainMenuStateEntity>>, gui_root: Res<MainMenuStateGuiRoot>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }

    commands.entity(gui_root.0).despawn();
}
