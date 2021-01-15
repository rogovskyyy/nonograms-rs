use rand::Rng;

pub struct Board {
    pub map: [[i32; 10]; 10],
    pub horizontal_vec: Vec<Vec<i32>>,
    pub vertical_vec: Vec<Vec<i32>>,
    pub current_position: [[i32; 10]; 10]
}

impl Board {
    pub fn init() -> Board {
        let map = Self::generate();
        Board {
            horizontal_vec: Self::horizontal_vec(&map),
            vertical_vec: Self::vertical_vec(&map),
            current_position: map.clone(),
            map,
        }
    }

    fn generate() -> [[i32; 10]; 10] {
        let mut t_map: [[i32; 10]; 10] = [[0; 10]; 10];
        for j in 0..10 {
            for i in 0..10 {
                let num = rand::thread_rng().gen_range(0..2);
                t_map[i][j] = num;
            }
        }
        t_map
    }

    fn horizontal_vec(map: &[[i32; 10]; 10]) -> Vec<Vec<i32>> {
        let mut vectors: Vec<Vec<i32>> = Vec::new();
        let mut counter = 0;
        for i in 0..10 {
            let mut subvector: Vec<i32> = Vec::new();
            for j in 0..10 {
                if map[i][j] != 0 {
                    counter += 1;
                    if j == 9 {
                        subvector.push(counter);
                        counter = 0;
                    }
                } else {
                    if counter != 0 {
                        subvector.push(counter);
                        counter = 0;
                    }
                }
            }
            vectors.push(subvector);
            counter = 0;
        }
        vectors
    }

    fn vertical_vec(map: &[[i32; 10]; 10]) -> Vec<Vec<i32>> {
        let mut vectors: Vec<Vec<i32>> = Vec::new();
        let mut counter = 0;
        for i in 0..10 {
            let mut subvector: Vec<i32> = Vec::new();
            for j in 0..10 {
                if map[j][i] != 0 {
                    counter += 1;
                    if j == 9 {
                        subvector.push(counter);
                        counter = 0;
                    }
                } else {
                    if counter != 0 {
                        subvector.push(counter);
                        counter = 0;
                    }
                }
            }
            vectors.push(subvector);
            counter = 0;
        }
        vectors
    }
}