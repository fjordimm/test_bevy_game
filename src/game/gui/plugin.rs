use bevy::{prelude::*, ui::UiSystems};

use crate::game::gui::{GuiNode, gui_button};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                gui_button::update_style
                    .after(UiSystems::Focus)
            );
    }
}

pub struct CollectionOfGuiItems(pub Vec<Box<dyn GuiNode>>);

macro_rules! impl_tuple_to_collectionofguiitems {
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

        impl_tuple_to_collectionofguiitems!($($t),*);
    };
}

impl_tuple_to_collectionofguiitems!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
