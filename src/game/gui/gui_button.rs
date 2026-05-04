use bevy::prelude::*;

use crate::game::gui::{GuiNode, GuiText, constants::*, plugin::CollectionOfGuiItems};

pub enum GuiButtonStyle {
    Regular,
    XButton,
}

#[derive(Component)]
pub struct GuiButtonTag {
    pub style: GuiButtonStyle,
}

pub struct GuiButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    style: GuiButtonStyle,
    event_supplier: Option<fn() -> E>,
    children: Vec<Box<dyn GuiNode>>,
}

impl<E> GuiButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    pub fn new<C: Into<CollectionOfGuiItems>>(
        style: GuiButtonStyle,
        event_supplier: fn() -> E,
        children: C,
    ) -> Self {
        Self {
            style: style,
            event_supplier: Some(event_supplier),
            children: children.into().0,
        }
    }

    pub fn new_regular(event_supplier: fn() -> E, text: impl Into<String>) -> Self {
        Self {
            style: GuiButtonStyle::Regular,
            event_supplier: Some(event_supplier),
            children: vec![Box::new(GuiText::new_regular(text))],
        }
    }
}

#[doc(hidden)]
#[derive(Event)]
pub struct _GuiButtonDummyGeneric;

impl GuiButton<_GuiButtonDummyGeneric> {
    pub fn new_eventless<C: Into<CollectionOfGuiItems>>(
        style: GuiButtonStyle,
        children: C,
    ) -> Self {
        Self {
            style: style,
            event_supplier: None,
            children: children.into().0,
        }
    }

    pub fn new_regular_eventless(text: impl Into<String>) -> Self {
        Self {
            style: GuiButtonStyle::Regular,
            event_supplier: None,
            children: vec![Box::new(GuiText::new_regular(text))],
        }
    }
}

impl<E> GuiNode for GuiButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    fn spawn(self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = match self.style {
            GuiButtonStyle::Regular => commands.spawn((
                GuiButtonTag { style: self.style },
                Button,
                Node {
                    border_radius: BorderRadius::all(px(BORDER_RADIUS)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(px(MAIN_PADDING)),
                    ..default()
                },
                main_box_shadow(),
                BackgroundColor(BUTTON_COLOR_MAIN),
            )),
            GuiButtonStyle::XButton => commands.spawn((
                GuiButtonTag { style: self.style },
                Button,
                Node {
                    border_radius: BorderRadius::all(px(BORDER_RADIUS)),
                    min_width: px(X_BUTTON_SIZE),
                    max_width: px(X_BUTTON_SIZE),
                    min_height: px(X_BUTTON_SIZE),
                    max_height: px(X_BUTTON_SIZE),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                main_box_shadow(),
                BackgroundColor(BUTTON_COLOR_MAIN),
            )),
        }
        .id();
        if let Some(par) = parent {
            commands.entity(par).add_child(entity);
        }

        for child in self.children {
            let child_entity = child.spawn_dyn(commands, None);
            commands.entity(entity).add_child(child_entity);
        }

        if let Some(event_supplier_) = &self.event_supplier {
            let event_supplier = event_supplier_.clone();
            commands.entity(entity).observe(
                move |_: On<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger(event_supplier());
                },
            );
        }

        entity
    }

    fn spawn_dyn(self: Box<Self>, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        self.spawn(commands, parent)
    }
}

pub fn update(
    mut query: Query<(&Interaction, &GuiButtonTag, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, tag, mut color) in &mut query {
        *color = what_style(interaction, tag);
    }
}

fn what_style(interaction: &Interaction, tag: &GuiButtonTag) -> BackgroundColor {
    match tag.style {
        GuiButtonStyle::Regular => match interaction {
            Interaction::None => BackgroundColor(BUTTON_COLOR_MAIN),
            Interaction::Hovered => BackgroundColor(BUTTON_COLOR_HOVER),
            Interaction::Pressed => BackgroundColor(BUTTON_COLOR_PRESSED),
        },
        GuiButtonStyle::XButton => match interaction {
            Interaction::None => BackgroundColor(X_BUTTON_COLOR_MAIN),
            Interaction::Hovered => BackgroundColor(X_BUTTON_COLOR_HOVER),
            Interaction::Pressed => BackgroundColor(X_BUTTON_COLOR_PRESSED),
        },
    }
}
