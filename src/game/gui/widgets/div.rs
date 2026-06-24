use bevy::prelude::*;

use crate::game::gui::resources::GuiThemeComputed;

#[allow(unused)]
pub enum GuiDivStyle {
    None,
    Regular,
    Custom {
        padding: UiRect,
        gap: f32,
        bg_color: Color,
        box_shadow: bool,
    },
}

#[allow(unused)]
pub struct GuiDivProps {
    pub starts_active: bool,
    pub expand: bool,
    pub size: Option<(f32, f32)>,
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub div_style: GuiDivStyle,
}

impl Default for GuiDivProps {
    fn default() -> Self {
        Self {
            starts_active: true,
            expand: false,
            size: None,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            div_style: GuiDivStyle::Regular,
        }
    }
}

#[derive(Component)]
struct GuiDivAttribs {
    is_active: bool,
    expand: bool,
    size: Option<(f32, f32)>,
    flex_direction: FlexDirection,
    justify_content: JustifyContent,
    align_items: AlignItems,
    div_style: GuiDivStyle,
}

#[allow(unused)]
pub fn gui_div(props: GuiDivProps) -> impl Bundle {
    (
        GuiDivAttribs {
            is_active: props.starts_active,
            expand: props.expand,
            size: props.size,
            flex_direction: props.flex_direction,
            justify_content: props.justify_content,
            align_items: props.align_items,
            div_style: props.div_style,
        },
        Node::default(),
    )
}

pub struct GuiDivPlugin;

impl Plugin for GuiDivPlugin {
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
        (&GuiDivAttribs, Entity, &mut Node),
        Or<(Added<GuiDivAttribs>, Changed<GuiDivAttribs>)>,
    >,
) {
    entity_q.iter_mut().for_each(|(attribs, entity, mut node)| {
        set_style(&mut commands, &theme, &attribs, &entity, &mut node);
    });
}

fn update_style_on_theme_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<(&GuiDivAttribs, Entity, &mut Node)>,
) {
    entity_q.iter_mut().for_each(|(attribs, entity, mut node)| {
        set_style(&mut commands, &theme, &attribs, &entity, &mut node);
    });
}

fn set_style(
    commands: &mut Commands,
    theme: &GuiThemeComputed,
    attribs: &GuiDivAttribs,
    entity: &Entity,
    node: &mut Node,
) {
    node.display = match attribs.is_active {
        true => Display::Flex,
        false => Display::None,
    };
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
    node.border_radius = BorderRadius::all(px(theme.0.border_radius));
    node.flex_direction = attribs.flex_direction;
    node.justify_content = attribs.justify_content;
    node.align_items = attribs.align_items;

    match attribs.div_style {
        GuiDivStyle::None => {
            node.padding = UiRect::default();
            node.row_gap = Val::ZERO;
            commands
                .entity(*entity)
                .remove::<BackgroundColor>()
                .remove::<BoxShadow>();
        }
        GuiDivStyle::Regular => {
            node.padding = UiRect::all(px(theme.0.padding_main));
            node.row_gap = px(theme.0.padding_main);
            commands
                .entity(*entity)
                .insert(BackgroundColor(theme.0.bg_color_main))
                .insert(theme.0.box_shadow.clone());
        }
        GuiDivStyle::Custom {
            padding,
            gap,
            bg_color,
            box_shadow,
        } => {
            node.padding = padding;
            node.row_gap = px(gap);
            commands.entity(*entity).insert(BackgroundColor(bg_color));

            if box_shadow {
                commands.entity(*entity).insert(theme.0.box_shadow.clone());
            }
        }
    }
}
