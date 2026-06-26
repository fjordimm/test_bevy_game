use bevy::prelude::*;

use crate::game::gui::resources::GuiThemeComputed;

#[allow(unused)]
pub struct GuiScreenDivProps {
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub bg_color: Color,
    pub padding: UiRect,
    pub gap: f32,
    pub starts_active: bool,
    pub with_children: Option<Box<dyn FnOnce(&mut ChildSpawner) + Sync + Send>>,
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
            with_children: None,
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

#[derive(Component)]
struct ChildrenAdder(Option<Box<dyn FnOnce(&mut ChildSpawner) + Sync + Send>>);

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
        ChildrenAdder(props.with_children),
    )
}

pub struct GuiScreenDivPlugin;

impl Plugin for GuiScreenDivPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update, apply_children_adder)
            .add_systems(Update, update_style_on_attrib_change)
            .add_systems(Update,
                update_style_on_theme_change
                    .run_if(resource_changed::<GuiThemeComputed>)
            )
            .add_systems(Update, update_style_from_state_change)
        ;
    }
}

fn apply_children_adder(
    world: &mut World,
    mut entity_q: Local<QueryState<Entity, With<ChildrenAdder>>>,
) {
    let entities: Vec<_> = entity_q.iter(world).map(|e| e).collect();

    entities.iter().for_each(|entity| {
        let mut entity_mut = world.entity_mut(*entity);
        if let Some(children_adder) = entity_mut.take::<ChildrenAdder>() {
            if let Some(f) = children_adder.0 {
                entity_mut.with_children(f);
            }
        }
    });
}

fn update_style_on_attrib_change(
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
            set_style(&mut commands, &theme, &attribs, &state, &entity, &mut node);
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
            set_style(&mut commands, &theme, &attribs, &state, &entity, &mut node);
        });
}

fn update_style_from_state_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<(&GuiScreenDivState, Entity, &mut Node), Changed<GuiScreenDivState>>,
) {
    entity_q.iter_mut().for_each(|(state, entity, mut node)| {
        modify_style_from_state(&mut commands, &theme, &state, &entity, &mut node);
    });
}

fn set_style(
    commands: &mut Commands,
    _theme: &GuiThemeComputed,
    attribs: &GuiScreenDivAttribs,
    state: &GuiScreenDivState,
    entity: &Entity,
    node: &mut Node,
) {
    node.display = match state.is_active {
        true => Display::Flex,
        false => Display::None,
    };
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
    node.display = match state.is_active {
        true => Display::Flex,
        false => Display::None,
    };
}
