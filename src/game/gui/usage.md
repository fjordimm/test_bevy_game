# Usage

## General Usage

All instances of GUI elements (that aren't children of other GUI elements) should be created once at the beginning of the game (Startup, in set GlobalStartupOrdering::GuiSpawning), should be set as a child of the resource GlobalGuiRoot, and should never be deleted. Instead of being deleted, they can be hidden; usually swapped betwen Display::Flex and Display::None.

## Z-Indexing GUI elements

- 1000-1999: In-game gui elements (hotbar, health, etc.)
    - The rest is reserved (for now)
- 2000-2999: In-game gui screens (inventory, chests, etc.)
    - The rest is reserved (for now)
- 3000-3999: Menus (pause menu, settings menu, main menu, etc.)
    - 3000: Pause menu
    - 3001: Main menu
    - The rest is reserved (for now)
- 4000-4999: Debug menus
    - 4000: Main debug menu
    - The rest is reserved (for now)