use crate::{math::Point2, texture::TextureId};

#[derive(Clone)]
pub struct Sprite {
    pub pos: Point2,
    pub texture_id: TextureId,
}

impl Sprite {
    pub const fn new(pos: Point2, texture_id: TextureId) -> Self {
        Self { pos, texture_id }
    }
}
