pub enum Moves {
    Up, Down, Left, Right
}

impl Moves {
    pub fn current_move(mov: Moves, user_position: &mut [usize; 2]) {
        match mov {
            Moves::Up => { 
                if user_position[0] >= 1 { user_position[0] -= 1; } else { user_position[0] = 9; }
            },
            Moves::Down => {
                if user_position[0] <= 8 { user_position[0] += 1; } else { user_position[0] = 0; }
            },
            Moves::Left => {
                if user_position[1] >= 1 { user_position[1] -= 1; } else { user_position[1] = 9; }
            },
            Moves::Right => {
                if user_position[1] <= 8 { user_position[1] += 1; } else { user_position[1] = 0;}
            }
        }
    }

    pub fn set_current_position(user_position: &mut[usize; 2],  position: &mut[[usize; 10]; 10]) {
        if position[user_position[0]][user_position[1]] == 0 {
            position[user_position[0]][user_position[1]] = 1
        } else {
            position[user_position[0]][user_position[1]] = 0
        }
    }

    pub fn if_win(position: [[usize; 10]; 10], map: [[usize; 10]; 10]) {
        assert_ne!(position, map);
    }
}