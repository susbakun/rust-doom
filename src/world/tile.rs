#[derive(PartialEq)]
pub enum Tile {
    Wall,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Wall,
            _ => Tile::Empty,
        }
    }
}
