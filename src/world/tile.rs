use crate::texture::TextureId;

#[derive(PartialEq)]
pub enum Tile {
    Wall(TextureId),
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
