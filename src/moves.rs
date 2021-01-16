pub enum Moves {
    Up, Down, Left, Right
}

impl Moves {
    pub fn current_move(mov: Moves, pos: &mut [i8; 2]) {
        match mov {
            Moves::Up => {
                if pos[1] >= 1 {
                    pos[1] -= 1;
                } else {
                    pos[1] = 9;
                }
            },
            Moves::Down => {
                if pos[1] <= 8 {
                    pos[1] += 1;
                } else {
                    pos[1] = 0;
                }
            },
            Moves::Left => {
                if pos[0] >= 1 {
                    pos[0] -= 1;
                } else {
                    pos[0] = 9;
                }
            },
            Moves::Right => {
                if pos[0] <= 8 {
                    pos[0] += 1;
                } else {
                    pos[0] = 0;
                }
            }
        }
    }
}