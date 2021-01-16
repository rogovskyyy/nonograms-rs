mod game;
mod draw;
mod board;
mod moves;

fn main() {
    let play = board::Board::init();
    game::Game::new().init(play);
}