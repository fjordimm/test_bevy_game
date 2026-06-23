use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GuiWidgetDuringAddFunctionalComponents;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GuiWidgetDuringUpdateFunctionalComponents;
