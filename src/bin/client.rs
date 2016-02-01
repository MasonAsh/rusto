#[macro_use]
extern crate rusto;

use rusto::*;

fn main() {
    let mut my_game = Game::new();

    my_game.run();
}
