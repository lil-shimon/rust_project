pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new (value: i32) -> Guess {
        if value < 1  {
            panic!("Guess value must be smaller than one, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be greater than 100, got {}.", value);
        }
        Guess{value}
    }
}

