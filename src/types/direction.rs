#[derive(Debug)]
pub enum Direction {
    In = 1,
    Out = 2,
    Unknown,
}

impl From<u8> for Direction {
    fn from(direction: u8) -> Direction {
        match direction {
            1 => Direction::In,
            2 => Direction::Out,
            _ => Direction::Unknown,
        }
    }
}
