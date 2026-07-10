use crate::texture::TexturedId;

#[derive(PartialEq)]
pub enum Tile {
    Wall(TexturedId),
    Empty,
}

impl Tile {
    pub fn new(tile_type: usize) -> Self {
        if tile_type == 0 {
            Self::Empty
        } else {
            Self::Wall(tile_type - 1)
        }
    }
}
