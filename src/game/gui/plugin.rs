use bevy::{prelude::*, ui::UiSystems};

use crate::game::gui::{GuiNode, gui_button, gui_div, gui_floating_panel, gui_screen_div};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_observer(gui_floating_panel::minimize_button_observer)
            .add_observer(gui_floating_panel::x_button_observer)
            .add_systems(Update,
                (
                    gui_button::update,
                    gui_div::update_is_active,
                    gui_screen_div::update_is_active,
                    gui_floating_panel::update_panel_dragged,
                    gui_floating_panel::update_content_from_is_minimized,
                    gui_floating_panel::update_title_bar_from_is_minimized,
                    gui_floating_panel::update_panel_from_is_active,
                )
                    .after(UiSystems::Focus),
            );
    }
}

pub struct CollectionOfGuiItems(pub Vec<Box<dyn GuiNode>>);

macro_rules! impl_tuple_into_collectionofguiitems {
    () => {
        impl Into<CollectionOfGuiItems> for () {
            fn into(self) -> CollectionOfGuiItems {
                CollectionOfGuiItems(vec![])
            }
        }
    };

    ($h:ident $(,$t:ident)*) => {
        #[allow(non_snake_case, unused_variables, unused_mut)]
        impl<$h: GuiNode + 'static $(, $t: GuiNode + 'static)*> Into<CollectionOfGuiItems> for ($h, $($t,)*) {
            fn into(self) -> CollectionOfGuiItems {
                let ($h, $($t,)*) = self;
                CollectionOfGuiItems(vec![
                    Box::new($h),
                    $(Box::new($t),)*
                ])
            }
        }

        impl_tuple_into_collectionofguiitems!($($t),*);
    };
}

impl_tuple_into_collectionofguiitems!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
