use std::f64::INFINITY;

mod tile;

use crate::{
    hitrecord::{HitRecord, Side},
    math::Point2,
    ray::Ray,
    world::tile::Tile,
};

pub struct World {
    map: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl World {
    pub fn new(map: &str) -> Self {
        let width = map.lines().next().map(|l| l.len()).unwrap_or(0);
        let height = map.lines().count();

        let mut world_map = vec![];

        for line in map.lines() {
            let mut items = vec![];
            for item in line.chars() {
                let tile = item.into();
                items.push(tile);
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

        // I normalize the ray.dir() so 1.0/ray
        // gives us the deltas
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

        let mut side_dist_x = if step_x > 0 {
            ((map_x as f64 + 1.0) - ray.origin().x) * delta_dist_x
        } else {
            (ray.origin().x - map_x as f64) * delta_dist_x
        };
        let mut side_dist_y = if step_y > 0 {
            ((map_y as f64 + 1.0) - ray.origin().y) * delta_dist_y
        } else {
            (ray.origin().y - map_y as f64) * delta_dist_y
        };

        loop {
            let point = Point2::new(map_x as f64, map_y as f64);
            if !self.in_range(point) {
                return false;
            }

            if self.is_wall(point) {
                rec.pos = point;

                rec.ray_distance = match rec.side {
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

    pub fn is_wall(&self, point: Point2) -> bool {
        let (x, y) = (point.x as usize, point.y as usize);
        self.map[y][x] == Tile::Wall
    }
}
