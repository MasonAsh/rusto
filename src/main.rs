#[macro_use]
extern crate glium;

mod renderer;
mod game;

fn main() {
    use glium::DisplayBuild;

    let mut my_game = game::Game::new();

    my_game.run();
}
