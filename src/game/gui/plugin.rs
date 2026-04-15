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

pub trait CollectionOfGuiItems {
    fn foreach(&self, f: impl FnMut(&dyn GuiNode));
}

macro_rules! impl_collectionofguiitems {
    () => {};

    ($h:ident $(,$t:ident)*) => {
        #[allow(non_snake_case)]
        impl<$($t: GuiNode),*> CollectionOfGuiItems for ($($t,)*) {
            #[allow(unused_variables, unused_mut)]
            fn foreach(&self, mut f: impl FnMut(&dyn GuiNode)) {
                let ($($t,)*) = self;
                $(f($t);)*
            }
        }


    };
}

impl_collectionofguiitems!();
impl_collectionofguiitems!(A);
impl_collectionofguiitems!(A, B);
