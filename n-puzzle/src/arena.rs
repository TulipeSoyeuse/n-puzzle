pub struct Puzzle {
    pub puzzle: Vec<Vec<u8>>,
    pub mouv_count: usize,
    //dim: u8,
}

impl Puzzle {
    pub fn new(dimension: usize) -> Self {
        Puzzle {
            puzzle: Vec::<Vec<u8>>::with_capacity(dimension)
                .iter()
                .map(|_| Vec::with_capacity(dimension))
                .collect(),
            mouv_count: 0,
            //dim: dimension as u8,
        }
    }
}
