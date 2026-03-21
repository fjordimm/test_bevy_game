pub mod game;

pub fn build_app() -> bevy::app::App {
    game::build_bevy_app()
}
