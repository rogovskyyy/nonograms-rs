use rand::Rng;

pub struct Board {
    pub map: [[usize; 10]; 10],
    pub horizontal_vec: Vec<Vec<usize>>,
    pub vertical_vec: Vec<Vec<usize>>,
    pub current_position: [[usize; 10]; 10]
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

    fn generate() -> [[usize; 10]; 10] {
        let mut t_map: [[usize; 10]; 10] = [[0; 10]; 10];
        for j in 0..10 {
            for i in 0..10 {
                let num = rand::thread_rng().gen_range(0..2);
                t_map[i][j] = num;
            }
        }
        t_map
    }

    fn horizontal_vec(map: &[[usize; 10]; 10]) -> Vec<Vec<usize>> {
        let mut vectors: Vec<Vec<usize>> = Vec::new();
        let mut counter = 0;
        for i in 0..10 {
            let mut subvector: Vec<usize> = Vec::new();
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

    fn vertical_vec(map: &[[usize; 10]; 10]) -> Vec<Vec<usize>> {
        let mut vectors: Vec<Vec<usize>> = Vec::new();
        let mut counter = 0;
        for i in 0..10 {
            let mut subvector: Vec<usize> = Vec::new();
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