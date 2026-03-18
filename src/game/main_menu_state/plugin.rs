use bevy::{input_focus::tab_navigation::TabGroup, prelude::*};

use crate::game::{
    core::states::{MouseMode, OverallState},
    main_menu_state::tags::MainMenuStateEntity,
};

pub struct MainMenuStatePlugin;

impl Plugin for MainMenuStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::MainMenu), on_enter)
            .add_systems(OnExit(OverallState::MainMenu), on_exit);
            // .add_systems(Update,
            //     play_button_on_press
            //         .run_if(in_state(OverallState::MainMenu))
            // );
    }
}

#[derive(Component)]
struct PlayButton;

fn play_button_on_press(
    interactions: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
) {
    for thing in &interactions {
        debug!("{:?}", thing);
    }
}

fn on_enter(mut commands: Commands, mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Free);

    commands.spawn((MainMenuStateEntity, Camera2d::default()));

    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: px(10),
            ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.0, 0.1)),
        TabGroup::default(),
        children![(
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: px(10),
                padding: UiRect::all(px(10)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.0, 0.0, 0.3)),
            TabGroup::default(),
            children![(
                PlayButton,
                Button,
                Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(px(10)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.0, 0.0, 0.5)),
                children![(Text::new("Play"))]
            )],
        )],
    )).observe(|on_click: On<Pointer<Click>>| {
        debug!("{:?}", on_click);
    });
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<MainMenuStateEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
