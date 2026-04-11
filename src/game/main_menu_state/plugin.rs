use bevy::prelude::*;

use crate::game::{
    core::states::{MouseMode, OverallState},
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

fn on_enter(mut commands: Commands, mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Free);

    commands.spawn((MainMenuStateEntity, Camera2d::default()));

    let menu_gui = main_menu_gui::make_main_menu_gui().spawn(&mut commands);
    commands.entity(menu_gui).insert(MainMenuStateEntity);
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<MainMenuStateEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
