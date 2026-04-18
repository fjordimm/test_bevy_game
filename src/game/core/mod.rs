mod plugin;
mod quit_game;

pub mod global_resources;
pub mod states;
pub use plugin::CorePlugin;
pub use plugin::GlobalGuiRoot;
pub use quit_game::quit_game;
