use bevy::prelude::*;

use crate::game::{
    core::{quit_game, resources::GlobalGuiRoot, states::OverallState},
    gui::{
        resources::GuiThemeComputed,
        widgets::{
            button::gui_button,
            div::{GuiDivProps, GuiDivStyle, gui_div},
            screen_div::{GuiScreenDivProps, gui_screen_div},
            text::{gui_text_h1, gui_text_h2, gui_text_p},
        },
    },
};

pub struct MainMenuGuiPlugin;

impl Plugin for MainMenuGuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::MainMenu), spawn_main_menu_gui)
            .add_systems(OnExit(OverallState::MainMenu), despawn_main_menu_gui)
        ;
    }
}

#[derive(Component)]
struct MainMenuGuiTag;

fn spawn_main_menu_gui(
    mut commands: Commands,
    gui_root: Res<GlobalGuiRoot>,
    theme: Res<GuiThemeComputed>,
) {
    let main_menu_gui = commands
        .spawn((gui_screen_div(GuiScreenDivProps {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            bg_color: theme.0.bg_color_main,
            ..default()
        }),))
        .with_children(|p| {
            p.spawn(gui_div(GuiDivProps {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                div_style: GuiDivStyle::Regular,
                with_children: Some(Box::new(|p: &mut ChildSpawner| {
                    p.spawn(gui_text_p("title or something"));
                    // p.spawn(gui_div(GuiDivProps {
                    //     flex_direction: FlexDirection::Column,
                    //     justify_content: JustifyContent::Center,
                    //     align_items: AlignItems::Center,
                    //     div_style: GuiDivStyle::None,
                    //     ..default()
                    // }))
                    // .with_child(gui_text_h1("Main Menu"));
                    // p.spawn(gui_button(default()))
                    //     .with_child(gui_text_h2("Play"))
                    //     .observe(|_: On<Pointer<Click>>, mut commands: Commands| {
                    //         commands.set_state(OverallState::Playing);
                    //     });
                    // p.spawn(gui_button(default()))
                    //     .with_child(gui_text_h2("Quit"))
                    //     .observe(|_: On<Pointer<Click>>| {
                    //         quit_game();
                    //     });
                })),
                ..default()
            }));
        })
        .insert(MainMenuGuiTag)
        .insert(ZIndex(3001))
        .id();

    commands.entity(gui_root.0).add_child(main_menu_gui);
}

fn despawn_main_menu_gui(
    mut commands: Commands,
    main_menu_gui_q: Query<Entity, With<MainMenuGuiTag>>,
) {
    main_menu_gui_q.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
}
