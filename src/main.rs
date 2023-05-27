mod board;
mod field;
mod game;
mod test;

use crate::game::Game;

fn main() {
    let mut game = Game::new();

    match game.game_loop() {
        Ok(_) => {println!("Game closed!")}
        Err(e) => {println!("{}", e)}
    }

}
