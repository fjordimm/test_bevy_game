use bevy::prelude::*;

use crate::game::{
    core::{resources::GlobalGuiRoot, states::OverallState},
    gui::{
        gui_children,
        resources::GuiThemeComputed,
        widgets::{
            screen_div::{GuiScreenDivProps, gui_screen_div},
            text::gui_text_h1,
        },
    },
    playing_state::{
        game_loading::resources::GameLoadingInhibition,
        sets::{DuringPlaying, OnEnterPlaying},
        states::{GameLoadingState, PauseState},
    },
};

pub struct GameLoadingPlugin;

impl Plugin for GameLoadingPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(OnEnterPlaying::ResourceSetup)
            )
            .add_systems(Update,
                update_game_loading_state
                    .in_set(DuringPlaying::Final)
            )
            .add_systems(OnEnter(GameLoadingState::Loading), spawn_loading_overlay)
            .add_systems(OnEnter(PauseState::Unpaused), spawn_loading_overlay)
            .add_systems(OnExit(GameLoadingState::Loading), despawn_loading_overlay)
            .add_systems(OnEnter(PauseState::Paused), despawn_loading_overlay)
        ;
    }
}

fn on_enter(mut commands: Commands) {
    commands.insert_resource(GameLoadingInhibition::new());
}

// TODO: This could be optimized. It is changing GameLoadingState every frame.
fn update_game_loading_state(
    game_loading_inhibition: Res<GameLoadingInhibition>,
    game_loading_state: Res<State<GameLoadingState>>,
    mut next_game_loading_state: ResMut<NextState<GameLoadingState>>,
) {
    if game_loading_inhibition.num_inhibitors() == 0 {
        if *game_loading_state.get() != GameLoadingState::NotLoading {
            next_game_loading_state.set(GameLoadingState::NotLoading);
        }
    } else {
        if *game_loading_state.get() != GameLoadingState::Loading {
            next_game_loading_state.set(GameLoadingState::Loading);
        }
    }
}

#[derive(Component, FromTemplate)]
struct LoadingOverlayTag;

fn spawn_loading_overlay(
    mut commands: Commands,
    game_loading_state: Res<State<GameLoadingState>>,
    pause_state: Res<State<PauseState>>,
    gui_root: Res<GlobalGuiRoot>,
    theme: Res<GuiThemeComputed>,
) {
    if let GameLoadingState::Loading = game_loading_state.get()
        && let PauseState::Unpaused = pause_state.get()
    {
        let overlay = commands
            .spawn(gui_screen_div(GuiScreenDivProps {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                bg_color: theme.0.pause_menu_bg_color,
                ..default()
            }))
            .insert(gui_children(|p| {
                p.spawn(gui_text_h1("Loading..."));
            }))
            .insert(LoadingOverlayTag)
            .insert(ZIndex(3010))
            .id();

        commands.entity(gui_root.0).add_child(overlay);
    }
}

fn despawn_loading_overlay(
    mut commands: Commands,
    overlay_q: Query<Entity, With<LoadingOverlayTag>>,
) {
    overlay_q.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
}
