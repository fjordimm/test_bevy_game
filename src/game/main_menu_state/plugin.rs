use bevy::prelude::*;

use crate::game::{
    core::{
        global_resources::GlobalGuiRoot,
        states::{MouseMode, OverallState},
    },
    gui::GuiNode,
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

fn on_enter(
    mut commands: Commands,
    mut next_mouse_mode: ResMut<NextState<MouseMode>>,
    gui_root: Res<GlobalGuiRoot>,
) {
    next_mouse_mode.set(MouseMode::Free);

    commands.spawn((MainMenuStateEntity, Camera2d::default()));

    let menu_gui = main_menu_gui::make_main_menu_gui().spawn(&mut commands, Some(gui_root.0));
    commands.entity(menu_gui).insert(ZIndex(0));
    commands.entity(menu_gui).insert(MainMenuStateEntity);
}

fn on_exit(mut commands: Commands, all_entities: Query<Entity, With<MainMenuStateEntity>>) {
    for entity in &all_entities {
        commands.entity(entity).despawn();
    }
}
