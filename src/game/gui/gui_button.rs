use bevy::prelude::*;

use crate::game::gui::{GuiNode, GuiText, constants::*, plugin::CollectionOfGuiItems};

pub struct GuiButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    event_supplier: Option<fn() -> E>,
    children: Vec<Box<dyn GuiNode>>,
}

impl<E> GuiButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    pub fn new<C: Into<CollectionOfGuiItems>>(event_supplier: fn() -> E, children: C) -> Self {
        Self {
            event_supplier: Some(event_supplier),
            children: children.into().0,
        }
    }

    pub fn new_regular(event_supplier: fn() -> E, text: impl Into<String>) -> Self {
        Self {
            event_supplier: Some(event_supplier),
            children: vec![Box::new(GuiText::regular(text))],
        }
    }
}

#[doc(hidden)]
#[derive(Event)]
pub struct _GuiButtonDummyGeneric;

impl GuiButton<_GuiButtonDummyGeneric> {
    pub fn new_no_event<C: Into<CollectionOfGuiItems>>(children: C) -> Self {
        Self {
            event_supplier: None,
            children: children.into().0,
        }
    }

    pub fn new_regular_no_event(text: impl Into<String>) -> Self {
        Self {
            event_supplier: None,
            children: vec![Box::new(GuiText::regular(text))],
        }
    }
}

impl<E> GuiNode for GuiButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    fn spawn(&self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = commands
            .spawn((
                GuiButtonTag,
                Button,
                Node {
                    border_radius: BorderRadius::all(px(BORDER_RADIUS)),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(px(MAIN_PADDING)),
                    ..default()
                },
                main_box_shadow(),
                BackgroundColor(BUTTON_COLOR_MAIN),
            ))
            .id();
        if let Some(par) = parent {
            commands.entity(par).add_child(entity);
        }

        for child in &self.children {
            let child_entity = child.spawn(commands, None);
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
}

#[derive(Component)]
pub struct GuiButtonTag;

pub fn update(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<GuiButtonTag>, Changed<Interaction>),
    >,
) {
    for (interaction, mut color) in &mut query {
        *color = what_style(interaction);
    }
}

fn what_style(interaction: &Interaction) -> BackgroundColor {
    match interaction {
        Interaction::None => BackgroundColor(BUTTON_COLOR_MAIN),
        Interaction::Hovered => BackgroundColor(BUTTON_COLOR_HOVER),
        Interaction::Pressed => BackgroundColor(BUTTON_COLOR_PRESSED),
    }
}
