use bevy::prelude::*;

use crate::game::{
    core::states::OverallState,
    gui::{self, GuiButton, GuiDiv, GuiNode, GuiScreenDiv, GuiText},
    playing_state::{sets::PlayingStateOrdering, states::PauseState, tags::PlayingStateEntity},
};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(PauseState::Paused),
                spawn_pause_menu
                    .run_if(in_state(OverallState::Playing))
                    .in_set(PlayingStateOrdering::Ui)
            )
            .add_systems(OnExit(PauseState::Paused),
                despawn_pause_menu
                    .run_if(in_state(OverallState::Playing))
                    .in_set(PlayingStateOrdering::Ui)
            )
            .add_observer(exit_button_observer)
            .add_observer(continue_button_observer);
    }
}

#[derive(Component)]
struct PauseMenuTag;

fn spawn_pause_menu(mut commands: Commands) {
    let pause_menu = GuiScreenDiv::new(
        gui::constants::PAUSE_MENU_BG_COLOR,
        FlexDirection::Column,
        vec![Box::new(GuiDiv::new(
            FlexDirection::Column,
            vec![
                Box::new(GuiText::h1("Pause Menu")),
                Box::new(GuiButton::new(
                    "Continue",
                    Some(|| interactions::ContinueButtonEv),
                )),
                Box::new(GuiButton::new("Exit", Some(|| interactions::ExitButtonEv))),
            ],
        ))],
    )
    .spawn(&mut commands);
    commands.entity(pause_menu).insert(PlayingStateEntity);
    commands.entity(pause_menu).insert(PauseMenuTag);
}

fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuTag>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub mod interactions {
    use bevy_ecs::event::Event;

    #[derive(Event)]
    pub struct ExitButtonEv;

    #[derive(Event)]
    pub struct ContinueButtonEv;
}

fn continue_button_observer(_: On<interactions::ContinueButtonEv>, mut commands: Commands) {
    commands.set_state(PauseState::Unpaused);
}

fn exit_button_observer(_: On<interactions::ExitButtonEv>, mut commands: Commands) {
    commands.set_state(OverallState::MainMenu);
}
