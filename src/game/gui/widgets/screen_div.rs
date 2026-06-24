use bevy::prelude::*;

use crate::game::gui::resources::GuiThemeComputed;

#[allow(unused)]
pub struct GuiScreenDivProps {
    pub starts_active: bool,
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub bg_color: Color,
    pub padding: UiRect,
    pub gap: f32,
}

impl Default for GuiScreenDivProps {
    fn default() -> Self {
        Self {
            starts_active: true,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            bg_color: Color::BLACK,
            padding: UiRect::ZERO,
            gap: 0.,
        }
    }
}

#[derive(Component)]
struct GuiScreenDivAttribs {
    is_active: bool,
    flex_direction: FlexDirection,
    justify_content: JustifyContent,
    align_items: AlignItems,
    bg_color: Color,
    padding: UiRect,
    gap: f32,
}

#[allow(unused)]
pub fn gui_screen_div(props: GuiScreenDivProps) -> impl Bundle {
    (
        GuiScreenDivAttribs {
            is_active: props.starts_active,
            flex_direction: props.flex_direction,
            justify_content: props.justify_content,
            align_items: props.align_items,
            bg_color: props.bg_color,
            padding: props.padding,
            gap: props.gap,
        },
        Node::default(),
    )
}

pub struct GuiScreenDivPlugin;

impl Plugin for GuiScreenDivPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                update_style_on_attrib_change
            )
            .add_systems(Update,
                update_style_on_theme_change
                    .run_if(resource_changed::<GuiThemeComputed>)
            )
        ;
    }
}

fn update_style_on_attrib_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<
        (&GuiScreenDivAttribs, Entity, &mut Node),
        Or<(Added<GuiScreenDivAttribs>, Changed<GuiScreenDivAttribs>)>,
    >,
) {
    entity_q.iter_mut().for_each(|(attribs, entity, mut node)| {
        set_style(&mut commands, &theme, &attribs, &entity, &mut node);
    });
}

fn update_style_on_theme_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<(&GuiScreenDivAttribs, Entity, &mut Node)>,
) {
    entity_q.iter_mut().for_each(|(attribs, entity, mut node)| {
        set_style(&mut commands, &theme, &attribs, &entity, &mut node);
    });
}

fn set_style(
    commands: &mut Commands,
    _theme: &GuiThemeComputed,
    attribs: &GuiScreenDivAttribs,
    entity: &Entity,
    node: &mut Node,
) {
    node.display = match attribs.is_active {
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
