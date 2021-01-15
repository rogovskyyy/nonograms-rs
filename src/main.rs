mod game;
mod draw;
mod board;

fn main() {
    let play = board::Board::init();
    game::Game::new().init(play);
}