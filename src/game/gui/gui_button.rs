use bevy::prelude::*;

use crate::game::gui::{
    GuiNode, GuiText,
    constants::{DIV_BORDER_COLOR, DIV_BORDER_SIZE, DIV_MAIN_COLOR, MAIN_PADDING},
};

pub struct GuiButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    text: String,
    event_supplier: Option<fn() -> E>,
}

impl<E> GuiButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    pub fn new(text: impl Into<String>, event_supplier: Option<fn() -> E>) -> Self {
        Self {
            text: text.into(),
            event_supplier: event_supplier,
        }
    }
}

impl<E> GuiNode for GuiButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    fn spawn(&self, commands: &mut Commands) -> Entity {
        let entity = commands
            .spawn((
                Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(px(MAIN_PADDING)),
                    border: UiRect::all(px(DIV_BORDER_SIZE)),
                    ..default()
                },
                BackgroundColor(DIV_MAIN_COLOR),
                BorderColor::all(DIV_BORDER_COLOR),
            ))
            .id();

        {
            let child_entity = GuiText::new(&self.text).spawn(commands);
            commands.entity(entity).add_child(child_entity);
        }

        if let Some(event_supplier) = &self.event_supplier {
            let es = event_supplier.clone();
            commands
                .entity(entity)
                .observe(move |_: On<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger(es());
                });
        }

        // match &self.on_click_event {
        //     Some(p_event) => {
        //         let event = Arc::clone(p_event);
        //         commands
        //             .entity(entity)
        //             .observe(move |_: On<Pointer<Click>>, mut commands: Commands| {
        //                 commands.trigger((*event).clone());
        //             });
        //     }
        //     _ => {}
        // }

        entity
    }
}
