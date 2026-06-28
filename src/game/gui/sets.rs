use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum GuiSystemsOrdering {
    SetupRelations,
    HandleGuiChildren,
    UpdateState,
    UpdateStyle,
}

pub(super) const GUI_SYSTEMS_ORDERING_ORDER: (
    GuiSystemsOrdering,
    GuiSystemsOrdering,
    GuiSystemsOrdering,
    GuiSystemsOrdering,
) = (
    GuiSystemsOrdering::SetupRelations,
    GuiSystemsOrdering::HandleGuiChildren,
    GuiSystemsOrdering::UpdateState,
    GuiSystemsOrdering::UpdateStyle,
);
