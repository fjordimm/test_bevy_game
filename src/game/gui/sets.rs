use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum GuiSystemsOrdering {
    SetupRelations,
    HandleGuiChildren,
    PreUpdateState,
    UpdateState,
    PostUpdateState,
    UpdateStyle,
    PostUpdateStyle,
}

pub(super) const GUI_SYSTEMS_ORDERING_ORDER: (
    GuiSystemsOrdering,
    GuiSystemsOrdering,
    GuiSystemsOrdering,
    GuiSystemsOrdering,
    GuiSystemsOrdering,
    GuiSystemsOrdering,
    GuiSystemsOrdering,
) = (
    GuiSystemsOrdering::SetupRelations,
    GuiSystemsOrdering::HandleGuiChildren,
    GuiSystemsOrdering::PreUpdateState,
    GuiSystemsOrdering::UpdateState,
    GuiSystemsOrdering::PostUpdateState,
    GuiSystemsOrdering::UpdateStyle,
    GuiSystemsOrdering::PostUpdateStyle,
);
