use bevy::prelude::*;

use crate::game::gui::{
    resources::GuiTheme,
    sets::{GuiWidgetDuringAddFunctionalComponents, GuiWidgetDuringUpdateFunctionalComponents},
};

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
    pub is_active: bool,
    pub expand: bool,
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub div_style: GuiDivStyle,
}

pub fn gui_div(props: GuiDivProps) -> impl Bundle {
    (
        GuiDivAttribs {
            is_active: props.starts_active,
            expand: props.expand,
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
                add_functional_components
                    .in_set(GuiWidgetDuringAddFunctionalComponents)
            )
            .add_systems(Update,
                update_functional_components_on_attrib_change
                    .in_set(GuiWidgetDuringUpdateFunctionalComponents)
            )
            .add_systems(Update,
                update_functional_components_on_theme_change
                    .in_set(GuiWidgetDuringUpdateFunctionalComponents)
                    .run_if(resource_changed::<GuiTheme>)
            )
        ;
    }
}

fn add_functional_components(
    mut commands: Commands,
    entity_q: Query<Entity, Added<GuiDivAttribs>>,
) {
    entity_q.iter().for_each(|entity| {
        commands.entity(entity).insert((Node::default(),));
    });
}

fn update_functional_components_on_attrib_change(
    mut commands: Commands,
    theme: Res<GuiTheme>,
    mut entity_q: Query<
        (&GuiDivAttribs, Entity, &mut Node),
        Or<(Added<GuiDivAttribs>, Changed<GuiDivAttribs>)>,
    >,
) {
    entity_q.iter_mut().for_each(|(attribs, entity, mut node)| {
        update_functional_components(&mut commands, &theme, &attribs, &entity, &mut node);
    });
}

fn update_functional_components_on_theme_change(
    mut commands: Commands,
    theme: Res<GuiTheme>,
    mut entity_q: Query<(&GuiDivAttribs, Entity, &mut Node)>,
) {
    entity_q.iter_mut().for_each(|(attribs, entity, mut node)| {
        update_functional_components(&mut commands, &theme, &attribs, &entity, &mut node);
    });
}

fn update_functional_components(
    commands: &mut Commands,
    theme: &GuiTheme,
    attribs: &GuiDivAttribs,
    entity: &Entity,
    node: &mut Node,
) {
    node.display = match attribs.is_active {
        false => Display::None,
        true => Display::Flex,
    };
    node.width = match attribs.expand {
        true => percent(100),
        false => Val::Auto,
    };
    node.height = match attribs.expand {
        true => percent(100),
        false => Val::Auto,
    };
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
            node.padding = UiRect::all(px(theme.main_padding));
            node.row_gap = px(theme.main_padding);
            commands
                .entity(*entity)
                .insert(BackgroundColor(theme.main_bg_color))
                .insert(theme.main_box_shadow.clone());
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
                commands
                    .entity(*entity)
                    .insert(theme.main_box_shadow.clone());
            }
        }
    }
}
