use bevy::prelude::*;

use crate::{
    game::{
        core::{
            global_resources::GlobalFonts,
            states::{MouseMode, OverallState},
        },
        main_menu_state::tags::MainMenuStateEntity,
    },
    gui::{GuiDiv, GuiParent, GuiScreenDiv, GuiText},
};

pub struct MainMenuStatePlugin;

impl Plugin for MainMenuStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::MainMenu), on_enter)
            .add_systems(OnExit(OverallState::MainMenu), on_exit);
    }
}

fn on_enter(
    mut commands: Commands,
    mut next_mouse_mode: ResMut<NextState<MouseMode>>,
    global_fonts: Res<GlobalFonts>,
) {
    next_mouse_mode.set(MouseMode::Free);

    commands.spawn((MainMenuStateEntity, Camera2d::default()));

    debug!("{:?}", global_fonts);

    GuiScreenDiv::new(
        Color::srgb(0.0, 0.0, 0.1),
        FlexDirection::Column,
        vec![Box::new(GuiDiv::new(
            FlexDirection::Column,
            vec![
                Box::new(GuiText::new("thingy")),
                Box::new(GuiDiv::new(
                    FlexDirection::Column,
                    vec![Box::new(GuiText::new("one")), Box::new(GuiText::new("two"))],
                )),
            ],
        ))],
    )
    .spawn(&mut commands);

    // let maindiv = commands
    //     .spawn((
    //         Node {
    //             width: percent(100),
    //             height: percent(100),
    //             display: Display::Flex,
    //             justify_content: JustifyContent::Center,
    //             align_items: AlignItems::Center,
    //             row_gap: px(10),
    //             ..default()
    //         },
    //         BackgroundColor(Color::srgb(0.0, 0.0, 0.1)),
    //         TabGroup::default(),
    //     ))
    //     .id();

    // let menudiv = commands
    //     .spawn((
    //         Node {
    //             display: Display::Flex,
    //             justify_content: JustifyContent::Center,
    //             align_items: AlignItems::Center,
    //             row_gap: px(10),
    //             padding: UiRect::all(px(10)),
    //             ..default()
    //         },
    //         BackgroundColor(Color::srgb(0.0, 0.0, 0.3)),
    //         TabGroup::default(),
    //     ))
    //     .id();
    // commands.entity(maindiv).add_child(menudiv);

    // let playbutton = commands
    //     .spawn((
    //         Button,
    //         Node {
    //             display: Display::Flex,
    //             justify_content: JustifyContent::Center,
    //             align_items: AlignItems::Center,
    //             padding: UiRect::all(px(10)),
    //             ..default()
    //         },
    //         BackgroundColor(Color::srgb(0.0, 0.0, 0.5)),
    //         children![(Text::new("Play"))],
    //     ))
    //     .id();
    // commands.entity(menudiv).add_child(playbutton);

    // commands.entity(playbutton).observe(
    //     |on_click: On<Pointer<Click>>, qs: Query<&Transform, With<Camera2d>>| {
    //         debug!("{:?}", on_click);
    //         for q in &qs {
    //             debug!("{:?}", q);
    //         }
    //     },
    // );
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<MainMenuStateEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
