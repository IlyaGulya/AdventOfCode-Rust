pub enum Result {
    WIN,
    DRAW,
    LOSE,
}

impl Result {
    pub fn by_str(string: &str) -> Result {
        match string {
            "X" => Result::LOSE,
            "Y" => Result::DRAW,
            "Z" => Result::WIN,
            _ => panic!("Unknown strategy {}", string)
        }
    }
}
