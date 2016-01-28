#[macro_use]
extern crate rusto;

use rusto::game;

fn main() {
    let mut my_game = game::Game::new();

    my_game.run();
}
