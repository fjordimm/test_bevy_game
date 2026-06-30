use bevy::prelude::*;

use crate::game::gui::{GuiChildren, resources::GuiThemeComputed, sets::GuiSystemsOrdering};

#[allow(unused)]
pub struct GuiScreenDivProps {
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub bg_color: Color,
    pub padding: UiRect,
    pub gap: f32,
    pub starts_active: bool,
}

impl Default for GuiScreenDivProps {
    fn default() -> Self {
        Self {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            bg_color: Color::BLACK,
            padding: UiRect::ZERO,
            gap: 0.,
            starts_active: true,
        }
    }
}

#[derive(Component)]
struct GuiScreenDivAttribs {
    flex_direction: FlexDirection,
    justify_content: JustifyContent,
    align_items: AlignItems,
    bg_color: Color,
    padding: UiRect,
    gap: f32,
}

#[derive(Component)]
struct GuiScreenDivState {
    is_active: bool,
}

#[allow(unused)]
pub fn gui_screen_div(props: GuiScreenDivProps) -> impl Bundle {
    (
        GuiScreenDivAttribs {
            flex_direction: props.flex_direction,
            justify_content: props.justify_content,
            align_items: props.align_items,
            bg_color: props.bg_color,
            padding: props.padding,
            gap: props.gap,
        },
        GuiScreenDivState {
            is_active: props.starts_active,
        },
        Node::default(),
    )
}

fn apply_style(
    commands: &mut Commands,
    _theme: &GuiThemeComputed,
    attribs: &GuiScreenDivAttribs,
    state: &GuiScreenDivState,
    entity: &Entity,
    node: &mut Node,
) {
    node.display = what_display(&state);
    node.position_type = PositionType::Absolute;
    node.left = Val::ZERO;
    node.top = Val::ZERO;
    node.width = vw(100);
    node.height = vh(100);
    node.flex_direction = attribs.flex_direction;
    node.justify_content = attribs.justify_content;
    node.align_items = attribs.align_items;
    node.padding = attribs.padding;
    node.row_gap = px(attribs.gap);
    commands
        .entity(*entity)
        .insert(BackgroundColor(attribs.bg_color));
}

fn modify_style_from_state(
    _commands: &mut Commands,
    _theme: &GuiThemeComputed,
    state: &GuiScreenDivState,
    _entity: &Entity,
    node: &mut Node,
) {
    node.display = what_display(&state);
}

fn what_display(state: &GuiScreenDivState) -> Display {
    match state.is_active {
        true => Display::Flex,
        false => Display::None,
    }
}

pub struct GuiScreenDivPlugin;

impl Plugin for GuiScreenDivPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                handle_gui_children
                    .in_set(GuiSystemsOrdering::HandleGuiChildren)
            )
            .add_systems(Update,
                update_style_on_init_or_attrib_change
                    .in_set(GuiSystemsOrdering::UpdateStyle)
            )
            .add_systems(Update,
                update_style_on_theme_change
                    .run_if(resource_changed::<GuiThemeComputed>)
                    .in_set(GuiSystemsOrdering::UpdateStyle)
            )
            .add_systems(Update,
                update_style_on_state_change
                    .in_set(GuiSystemsOrdering::UpdateStyle)
            )
        ;
    }
}

fn handle_gui_children(
    world: &mut World,
    mut entity_q: Local<QueryState<Entity, (With<GuiScreenDivAttribs>, With<GuiChildren>)>>,
) {
    let entities: Vec<_> = entity_q.iter(world).map(|e| e).collect();

    entities.iter().for_each(|entity| {
        let mut entity_mut = world.entity_mut(*entity);
        if let Some(gui_children) = entity_mut.take::<GuiChildren>() {
            entity_mut.with_children(gui_children.0);
        }
    });
}

fn update_style_on_init_or_attrib_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<
        (&GuiScreenDivAttribs, &GuiScreenDivState, Entity, &mut Node),
        Or<(Added<GuiScreenDivAttribs>, Changed<GuiScreenDivAttribs>)>,
    >,
) {
    entity_q
        .iter_mut()
        .for_each(|(attribs, state, entity, mut node)| {
            apply_style(&mut commands, &theme, &attribs, &state, &entity, &mut node);
        });
}

fn update_style_on_theme_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<(&GuiScreenDivAttribs, &GuiScreenDivState, Entity, &mut Node)>,
) {
    entity_q
        .iter_mut()
        .for_each(|(attribs, state, entity, mut node)| {
            apply_style(&mut commands, &theme, &attribs, &state, &entity, &mut node);
        });
}

fn update_style_on_state_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<(&GuiScreenDivState, Entity, &mut Node), Changed<GuiScreenDivState>>,
) {
    entity_q.iter_mut().for_each(|(state, entity, mut node)| {
        modify_style_from_state(&mut commands, &theme, &state, &entity, &mut node);
    });
}
