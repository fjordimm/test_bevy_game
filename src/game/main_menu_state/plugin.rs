use bevy::{input_focus::tab_navigation::TabGroup, prelude::*};

use crate::game::{
    core::states::{MouseMode, OverallState},
    egui_setup::tags::CameraForEgui,
    main_menu_state::{
        main_menu_gui::main_menu_gui,
        tags::{MainMenuStateCameraForEgui, MainMenuStateEntity},
    },
};

pub struct MainMenuStatePlugin;

impl Plugin for MainMenuStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::MainMenu), on_enter)
            .add_systems(OnExit(OverallState::MainMenu), on_exit);
        // .add_systems(CameraForEgui,
        //     main_menu_gui
        //         .run_if(in_state(OverallState::MainMenu))
        // );
    }
}

fn on_enter(mut commands: Commands, mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Free);

    commands.spawn((
        MainMenuStateEntity,
        MainMenuStateCameraForEgui,
        Camera2d::default(),
    ));

    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(10),
            ..default()
        },
        TabGroup::default(),
        children![
            (
                Text::new("Bluh")
            ),
            (
                
            )
        ]
    ));
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<MainMenuStateEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
