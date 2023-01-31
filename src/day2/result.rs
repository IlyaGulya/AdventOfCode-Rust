use std::str::FromStr;

pub enum Result {
    WIN,
    DRAW,
    LOSE,
}

#[derive(Debug)]
pub struct UnknownStrategy(String);

impl FromStr for Result {
    type Err = UnknownStrategy;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let item = match s {
            "X" => Result::LOSE,
            "Y" => Result::DRAW,
            "Z" => Result::WIN,
            _ => return Err(UnknownStrategy(s.to_owned()))
        };

        Ok(item)
    }
}
