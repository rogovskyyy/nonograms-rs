extern crate piston_window;

use crate::draw::{ Draw };
use crate::board::{ Board };

pub struct Game {
    pub window_size: [u32; 2],
}

impl Game {
    pub fn new() -> Game {
        Game {
            window_size: [700, 700]
        }
    }

    pub fn init(self, object: Board) {
        Draw::render(self, object);
    }
}