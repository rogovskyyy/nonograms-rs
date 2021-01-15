pub enum Move {
    Up, Down, Left, Right, None
}

impl Move {
    pub fn current_move(mov: Move, &pos: [i8; 2]]) {
        match mov {
            Move::Up => {
                if pos[1] <= 1 {
                    pos[1] -= 1;
                } else {
                    pos[1] = 9;
                }
            },
            Move::Down => {
                if pos[1] <= 1 {
                    pos[1] -= 1;
                } else {
                    pos[1] = 9;
                }
            },
            Move::Left => {
                if pos[0] <= 1 {
                    pos[0] -= 1;
                } else {
                    pos[0] = 9;
                }
            },
            Move::Right => {
                if pos[0] <= 1 {
                    pos[0] -= 1;
                } else {
                    pos[0] = 9;
                }
            },
            _ => ()
        }
    }
}