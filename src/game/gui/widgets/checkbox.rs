use bevy::prelude::*;
use bevy_ecs::query::QueryData;

use crate::game::gui::{
    resources::GuiThemeComputed,
    sets::GuiSystemsOrdering,
    widgets::icon::{GuiIconIcon, gui_icon},
};

#[allow(unused)]
pub struct GuiCheckboxProps {}

impl Default for GuiCheckboxProps {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Component)]
struct GuiCheckboxAttribs {}

enum GuiCheckboxPressedState {
    None,
    Hovered,
    Pressed,
}

#[derive(Component)]
pub struct GuiCheckboxState {
    checked: bool,
    pressed_state: GuiCheckboxPressedState,
}

#[allow(unused)]
pub fn gui_checkbox(props: GuiCheckboxProps) -> impl Bundle {
    (
        GuiCheckboxAttribs {},
        GuiCheckboxState {
            checked: false,
            pressed_state: GuiCheckboxPressedState::None,
        },
        Button,
        Node::default(),
    )
}

// Updates the checkbox state with the resource value automatically, and vice-versa.
// The first argument is a tag (component) that the actual checkbox entity has.
// The second argument, the resource to which the checkbox will be binded, must
//   be a tuple struct with one bool element, and must derive `Resource`.
macro_rules! bind_checkbox_with_resource {
    ($CheckboxTag:ident, $ResourceToBind:ident) => {
        (
            (|mut checkbox_q: Query<
                crate::game::gui::widgets::checkbox::GuiCheckboxInterface,
                With<$CheckboxTag>,
            >,
              res: Res<$ResourceToBind>| {
                checkbox_q.iter_mut().for_each(|mut checkbox| {
                    checkbox.set_checked(res.0);
                });
            })
            .run_if(resource_changed::<$ResourceToBind>),
            |checkbox_q: Query<
                crate::game::gui::widgets::checkbox::GuiCheckboxInterface,
                (
                    With<$CheckboxTag>,
                    Changed<crate::game::gui::widgets::checkbox::GuiCheckboxState>,
                ),
            >,
             mut res: ResMut<$ResourceToBind>| {
                checkbox_q.iter().for_each(|checkbox| {
                    res.0 = checkbox.checked();
                });
            },
        )
    };
}

pub(crate) use bind_checkbox_with_resource;

fn apply_style(
    commands: &mut Commands,
    theme: &GuiThemeComputed,
    _attribs: &GuiCheckboxAttribs,
    state: &GuiCheckboxState,
    entity: &Entity,
    node: &mut Node,
) {
    node.display = Display::Flex;
    node.width = Val::Auto;
    node.height = Val::Auto;
    node.flex_direction = FlexDirection::Column;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;

    node.border_radius = BorderRadius::all(px(theme.0.border_radius));
    node.min_width = px(theme.0.checkbox_icon_size + 2.0 * theme.0.checkbox_padding);
    node.min_height = px(theme.0.checkbox_icon_size + 2.0 * theme.0.checkbox_padding);
    node.padding = UiRect::all(px(theme.0.checkbox_padding));
    commands
        .entity(*entity)
        .insert(BackgroundColor(what_bg_color(&theme, &state)))
        .insert(theme.0.box_shadow.clone());
}

fn modify_style_from_state(
    commands: &mut Commands,
    theme: &GuiThemeComputed,
    state: &GuiCheckboxState,
    entity: &Entity,
) {
    commands
        .entity(*entity)
        .insert(BackgroundColor(what_bg_color(&theme, &state)));

    commands.entity(*entity).despawn_children();
    if state.checked {
        let icon = commands
            .spawn(gui_icon(
                GuiIconIcon::Check,
                theme.0.checkbox_icon_size,
                theme.0.checkbox_icon_size,
                default(),
            ))
            .id();
        commands.entity(*entity).add_child(icon);
    }
}

fn what_bg_color(theme: &GuiThemeComputed, state: &GuiCheckboxState) -> Color {
    match state.pressed_state {
        GuiCheckboxPressedState::None => theme.0.button_color_normal,
        GuiCheckboxPressedState::Hovered => theme.0.button_color_hovered,
        GuiCheckboxPressedState::Pressed => theme.0.button_color_pressed,
    }
}

pub struct GuiCheckboxPlugin;

impl Plugin for GuiCheckboxPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
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
                update_state
                    .in_set(GuiSystemsOrdering::UpdateState)
            )
            .add_systems(Update,
                update_style_on_state_change
                    .in_set(GuiSystemsOrdering::UpdateStyle)
            )
        ;
    }
}

fn update_style_on_init_or_attrib_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<
        (&GuiCheckboxAttribs, &GuiCheckboxState, Entity, &mut Node),
        Or<(Added<GuiCheckboxAttribs>, Changed<GuiCheckboxAttribs>)>,
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
    mut entity_q: Query<(&GuiCheckboxAttribs, &GuiCheckboxState, Entity, &mut Node)>,
) {
    entity_q
        .iter_mut()
        .for_each(|(attribs, state, entity, mut node)| {
            apply_style(&mut commands, &theme, &attribs, &state, &entity, &mut node);
        });
}

fn update_state(mut entity_q: Query<(&mut GuiCheckboxState, &Interaction), Changed<Interaction>>) {
    entity_q.iter_mut().for_each(|(mut state, interaction)| {
        state.pressed_state = match interaction {
            Interaction::None => GuiCheckboxPressedState::None,
            Interaction::Hovered => GuiCheckboxPressedState::Hovered,
            Interaction::Pressed => GuiCheckboxPressedState::Pressed,
        };

        if *interaction == Interaction::Pressed {
            state.checked = !state.checked;
        }
    });
}

fn update_style_on_state_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    entity_q: Query<(&GuiCheckboxState, Entity), Changed<GuiCheckboxState>>,
) {
    entity_q.iter().for_each(|(state, entity)| {
        modify_style_from_state(&mut commands, &theme, &state, &entity);
    });
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct GuiCheckboxInterface {
    state: &'static mut GuiCheckboxState,
}

impl<'w, 's> GuiCheckboxInterfaceItem<'w, 's> {
    #[allow(unused)]
    pub fn checked(&self) -> bool {
        self.state.checked
    }

    #[allow(unused)]
    pub fn set_checked(&mut self, val: bool) {
        self.state.checked = val;
    }
}

impl<'w, 's> GuiCheckboxInterfaceReadOnlyItem<'w, 's> {
    #[allow(unused)]
    pub fn checked(&self) -> bool {
        self.state.checked
    }
}
