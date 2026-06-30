use bevy::prelude::*;

use crate::game::gui::{GuiChildren, resources::GuiThemeComputed, sets::GuiSystemsOrdering};

#[allow(unused)]
pub enum GuiDivStyle {
    None,
    Regular,
    Custom {
        border_radius: BorderRadius,
        padding: UiRect,
        gap: f32,
        bg_color: Color,
        box_shadow: bool,
    },
}

#[allow(unused)]
pub struct GuiDivProps {
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub div_style: GuiDivStyle,
    pub size: Option<(f32, f32)>,
    pub expand: bool,
    pub starts_active: bool,
}

impl Default for GuiDivProps {
    fn default() -> Self {
        Self {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            div_style: GuiDivStyle::Regular,
            size: None,
            expand: false,
            starts_active: true,
        }
    }
}

#[derive(Component)]
struct GuiDivAttribs {
    flex_direction: FlexDirection,
    justify_content: JustifyContent,
    align_items: AlignItems,
    div_style: GuiDivStyle,
    size: Option<(f32, f32)>,
    expand: bool,
}

#[derive(Component)]
struct GuiDivState {
    is_active: bool,
}

#[allow(unused)]
pub fn gui_div(props: GuiDivProps) -> impl Bundle {
    (
        GuiDivAttribs {
            flex_direction: props.flex_direction,
            justify_content: props.justify_content,
            align_items: props.align_items,
            div_style: props.div_style,
            size: props.size,
            expand: props.expand,
        },
        GuiDivState {
            is_active: props.starts_active,
        },
        Node::default(),
    )
}

fn apply_style(
    commands: &mut Commands,
    theme: &GuiThemeComputed,
    attribs: &GuiDivAttribs,
    state: &GuiDivState,
    entity: &Entity,
    node: &mut Node,
) {
    node.display = what_display(&state);
    match attribs.size {
        Some((w, h)) => {
            node.width = px(w);
            node.height = px(h);
        }
        None => match attribs.expand {
            true => {
                node.width = percent(100);
                node.height = percent(100);
            }
            false => {
                node.width = Val::Auto;
                node.height = Val::Auto;
            }
        },
    }
    node.flex_direction = attribs.flex_direction;
    node.justify_content = attribs.justify_content;
    node.align_items = attribs.align_items;

    match attribs.div_style {
        GuiDivStyle::None => {
            node.border_radius = BorderRadius::ZERO;
            node.padding = UiRect::ZERO;
            node.row_gap = Val::ZERO;
            commands
                .entity(*entity)
                .remove::<BackgroundColor>()
                .remove::<BoxShadow>();
        }
        GuiDivStyle::Regular => {
            node.border_radius = BorderRadius::all(px(theme.0.border_radius));
            node.padding = UiRect::all(px(theme.0.padding_main));
            node.row_gap = px(theme.0.padding_main);
            commands
                .entity(*entity)
                .insert(BackgroundColor(theme.0.bg_color_main))
                .insert(theme.0.box_shadow.clone());
        }
        GuiDivStyle::Custom {
            border_radius,
            padding,
            gap,
            bg_color,
            box_shadow,
        } => {
            node.border_radius = border_radius;
            node.padding = padding;
            node.row_gap = px(gap);
            commands.entity(*entity).insert(BackgroundColor(bg_color));

            if box_shadow {
                commands.entity(*entity).insert(theme.0.box_shadow.clone());
            }
        }
    }
}

fn modify_style_from_state(
    _commands: &mut Commands,
    _theme: &GuiThemeComputed,
    state: &GuiDivState,
    _entity: &Entity,
    node: &mut Node,
) {
    node.display = what_display(&state);
}

fn what_display(state: &GuiDivState) -> Display {
    match state.is_active {
        true => Display::Flex,
        false => Display::None,
    }
}

pub struct GuiDivPlugin;

impl Plugin for GuiDivPlugin {
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

fn update_style_on_init_or_attrib_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<
        (&GuiDivAttribs, &GuiDivState, Entity, &mut Node),
        Or<(Added<GuiDivAttribs>, Changed<GuiDivAttribs>)>,
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
    mut entity_q: Query<(&GuiDivAttribs, &GuiDivState, Entity, &mut Node)>,
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
    mut entity_q: Query<(&GuiDivState, Entity, &mut Node), Changed<GuiDivState>>,
) {
    entity_q.iter_mut().for_each(|(state, entity, mut node)| {
        modify_style_from_state(&mut commands, &theme, &state, &entity, &mut node);
    });
}
