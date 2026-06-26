use bevy::prelude::*;

use crate::game::gui::{GuiChildren, resources::GuiThemeComputed};

#[allow(unused)]
pub struct GuiButtonProps {}

impl Default for GuiButtonProps {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Component)]
struct GuiButtonAttribs {}

enum GuiButtonPressedState {
    None,
    Hovered,
    Pressed,
}

#[derive(Component)]
struct GuiButtonState {
    pressed_state: GuiButtonPressedState,
}

#[allow(unused)]
pub fn gui_button(props: GuiButtonProps) -> impl Bundle {
    (
        GuiButtonAttribs {},
        GuiButtonState {
            pressed_state: GuiButtonPressedState::None,
        },
        Button,
        Node::default(),
    )
}

fn set_style(
    commands: &mut Commands,
    theme: &GuiThemeComputed,
    _attribs: &GuiButtonAttribs,
    state: &GuiButtonState,
    entity: &Entity,
    node: &mut Node,
) {
    node.display = Display::Flex;
    node.width = Val::Auto;
    node.height = Val::Auto;
    node.border_radius = BorderRadius::all(px(theme.0.border_radius));
    node.flex_direction = FlexDirection::Column;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.padding = UiRect::all(px(theme.0.padding_main));
    node.row_gap = px(theme.0.padding_main);
    commands
        .entity(*entity)
        .insert(BackgroundColor(what_bg_color(&theme, &state)))
        .insert(theme.0.box_shadow.clone());
}

fn modify_style_from_state(
    commands: &mut Commands,
    theme: &GuiThemeComputed,
    state: &GuiButtonState,
    entity: &Entity,
) {
    commands
        .entity(*entity)
        .insert(BackgroundColor(what_bg_color(&theme, &state)));
}

fn what_bg_color(theme: &GuiThemeComputed, state: &GuiButtonState) -> Color {
    match state.pressed_state {
        GuiButtonPressedState::None => theme.0.button_color_normal,
        GuiButtonPressedState::Hovered => theme.0.button_color_hovered,
        GuiButtonPressedState::Pressed => theme.0.button_color_pressed,
    }
}

pub struct GuiButtonPlugin;

impl Plugin for GuiButtonPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update, handle_gui_children)
            .add_systems(Update, update_style_on_attrib_change)
            .add_systems(Update,
                update_style_on_theme_change
                    .run_if(resource_changed::<GuiThemeComputed>)
            )
            .add_systems(Update, update_state)
            .add_systems(Update, update_style_from_state_change)
        ;
    }
}

fn handle_gui_children(
    world: &mut World,
    mut entity_q: Local<QueryState<Entity, With<GuiChildren>>>,
) {
    let entities: Vec<_> = entity_q.iter(world).map(|e| e).collect();

    entities.iter().for_each(|entity| {
        let mut entity_mut = world.entity_mut(*entity);
        if let Some(gui_children) = entity_mut.take::<GuiChildren>() {
            entity_mut.with_children(gui_children.0);
        }
    });
}

fn update_style_on_attrib_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<
        (&GuiButtonAttribs, &GuiButtonState, Entity, &mut Node),
        Or<(Added<GuiButtonAttribs>, Changed<GuiButtonAttribs>)>,
    >,
) {
    entity_q
        .iter_mut()
        .for_each(|(attribs, state, entity, mut node)| {
            set_style(&mut commands, &theme, &attribs, &state, &entity, &mut node);
        });
}

fn update_style_on_theme_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<(&GuiButtonAttribs, &GuiButtonState, Entity, &mut Node)>,
) {
    entity_q
        .iter_mut()
        .for_each(|(attribs, state, entity, mut node)| {
            set_style(&mut commands, &theme, &attribs, &state, &entity, &mut node);
        });
}

fn update_state(mut entity_q: Query<(&mut GuiButtonState, &Interaction), Changed<Interaction>>) {
    entity_q.iter_mut().for_each(|(mut state, interaction)| {
        state.pressed_state = match interaction {
            Interaction::None => GuiButtonPressedState::None,
            Interaction::Hovered => GuiButtonPressedState::Hovered,
            Interaction::Pressed => GuiButtonPressedState::Pressed,
        }
    });
}

fn update_style_from_state_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    entity_q: Query<(&GuiButtonState, Entity), Changed<GuiButtonState>>,
) {
    entity_q.iter().for_each(|(state, entity)| {
        modify_style_from_state(&mut commands, &theme, &state, &entity);
    });
}
