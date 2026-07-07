use crate::math::Point2;

#[derive(Default, PartialEq)]
pub enum Side {
    #[default]
    X,
    Y,
}

#[derive(Default)]
pub struct HitRecord {
    pub pos: Point2,
    pub ray_distance: f64,
    pub side: Side,
}

impl HitRecord {
    pub fn new(pos: Point2, ray_distance: f64) -> Self {
        Self {
            pos,
            ray_distance,
            side: Side::X,
        }
    }
}
