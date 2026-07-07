use std::f64::INFINITY;

use crate::{
    hitrecord::{HitRecord, Side},
    math::Point2,
    ray::Ray,
};

struct MapItem {
    item: char,
    coordinate: Point2,
}

impl MapItem {
    fn new(item: char, coordinate: Point2) -> Self {
        Self { item, coordinate }
    }
}

pub struct World {
    map: Vec<Vec<MapItem>>,
    width: usize,
    height: usize,
}

impl World {
    pub fn new(map: &str) -> Self {
        let width = map.lines().next().map(|l| l.len()).unwrap_or(0);
        let height = map.lines().count();

        let mut world_map = vec![];

        for (l_index, line) in map.lines().enumerate() {
            let mut items = vec![];
            for (i_index, item) in line.char_indices() {
                let map_item = MapItem::new(item, Point2::new(i_index as f64, l_index as f64));
                items.push(map_item);
            }
            world_map.push(items);
        }

        Self {
            map: world_map,
            width,
            height,
        }
    }

    pub fn hit(&self, ray: &Ray, rec: &mut HitRecord) -> bool {
        let mut map_x = ray.origin().x.floor() as isize;
        let mut map_y = ray.origin().y.floor() as isize;

        let step_x = if ray.dir().x >= 0.0 { 1 } else { -1 };
        let step_y = if ray.dir().y >= 0.0 { 1 } else { -1 };

        let delta_dist_x = if ray.dir().x == 0.0 {
            INFINITY
        } else {
            1.0 / ray.dir().x.abs()
        };
        let delta_dist_y = if ray.dir().y == 0.0 {
            INFINITY
        } else {
            1.0 / ray.dir().y.abs()
        };

        let mut side_dist_x = (map_x + step_x) as f64 - ray.origin().x;
        let mut side_dist_y = (map_y + step_y) as f64 - ray.origin().y;

        side_dist_x = if ray.dir().x == 0.0 {
            INFINITY
        } else {
            side_dist_x / ray.dir().x.abs()
        };

        side_dist_y = if ray.dir().y == 0.0 {
            INFINITY
        } else {
            side_dist_y / ray.dir().y.abs()
        };

        loop {
            let point = Point2::new(map_x as f64, map_y as f64);
            if !self.in_range(point) {
                return false;
            }

            if self.is_wall(point) {
                rec.pos = point;

                rec.distance = match rec.side {
                    Side::X => side_dist_x - delta_dist_x,
                    Side::Y => side_dist_y - delta_dist_y,
                };

                return true;
            }

            if side_dist_x <= side_dist_y {
                map_x += step_x;
                side_dist_x += delta_dist_x;
                rec.side = Side::X;
            } else {
                map_y += step_y;
                side_dist_y += delta_dist_y;
                rec.side = Side::Y;
            }
        }
    }

    fn in_range(&self, point: Point2) -> bool {
        (point.x >= 0.0 && point.x < self.width as f64)
            && (point.y >= 0.0 && point.y < self.height as f64)
    }

    fn is_wall(&self, point: Point2) -> bool {
        let (x, y) = (point.x as usize, point.y as usize);
        self.map[y][x].item == '#'
    }
}
