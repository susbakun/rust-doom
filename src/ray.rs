use crate::math::{Point2, Vec2};

pub struct Ray {
    origin: Point2,
    dir: Vec2,
}

impl Ray {
    pub fn new(origin: Point2, dir: Vec2) -> Self {
        Self { origin, dir }
    }

    pub fn origin(&self) -> Point2 {
        self.origin
    }

    pub fn dir(&self) -> Vec2 {
        self.dir
    }
}
