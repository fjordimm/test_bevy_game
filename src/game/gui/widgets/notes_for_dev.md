# Notes For Development

- Anything permanent (until the gui widget is despawned) should be stored in the widget instance's `_Attribs` and `_State` components.
    - Fields of `_Attribs` should be things that are only set at creation, or are updated rarely.
    - Fields of `_State` can be things that are updated more commonly.
- An exception to this is that the entity hierarchy should be preserved (so basically no adding/deleting entities), so that after the client adds sub-widgets (using `gui::gui_children` or `gui::gui_child`), those sub-widgets will stay children to the same entity.