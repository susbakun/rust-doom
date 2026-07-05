use crate::{math::Point2, ray::Ray};

pub struct World {
    map: String,
    player_pos: Point2,
    walls: Vec<Point2>,
}

impl World {
    pub fn new(map: &str) -> Self {
        let mut player_pos = Point2::new(0.0, 0.0);
        let mut walls = vec![];

        for (l_index, line) in map.lines().enumerate() {
            for (i_index, item) in line.char_indices() {
                if item == '#' {
                    walls.push(Point2::new(
                        (i_index as isize - 4) as f64,
                        (l_index as isize - 3) as f64,
                    ));
                } else if item == 'P' {
                    player_pos =
                        Point2::new((i_index as isize - 4) as f64, (l_index as isize - 3) as f64);
                }
            }
        }

        Self {
            map: map.to_string(),
            player_pos,
            walls,
        }
    }

    pub fn hit(&self, ray: &mut Ray) -> bool {
        let mut t = 0.0;
        let n = self.map.lines().count() as f64;

        loop {
            if t >= n {
                return false;
            }

            let p = ray.origin() + (ray.dir() * t);

            for wall in &self.walls {
                if wall.x == p.x && wall.y == p.y {
                    println!("wall: {wall:?}, distance: {}", wall.distance(ray.origin()));
                    return true;
                }
            }

            t += 1.0;
        }
    }
}
