use crate::{
    camera::Camera,
    color::Color,
    hitrecord::{HitRecord, Side},
    math::Vec2,
    prelude::{HEIGHT, WIDTH},
    ray::Ray,
    world::World,
};

pub fn render(world: &World, camera: &Camera, frame: &mut [u8]) {
    // clearing the buffer
    frame.fill(0);

    for x in 0..WIDTH {
        let mut rec = HitRecord::default();

        let camera_x = (2.0 * x as f64 / WIDTH as f64) - 1.0;
        let dir = camera.get_front();
        let plane = Vec2::new(-dir.y, dir.x) * 0.66;
        let ray_dir = Vec2::new(dir.x + plane.x * camera_x, dir.y + plane.y * camera_x);

        let ray = Ray::new(camera.position(), ray_dir);

        if world.hit(&ray, &mut rec) {
            // cos(angle between look direction and ray)
            let perp_dist = rec.ray_distance * (dir.x * ray.dir().x + dir.y * ray.dir().y);
            let perp_dist = perp_dist.max(0.0001);

            let wall_height = (HEIGHT as f64 / perp_dist) as i32;
            let wall_height = wall_height.clamp(1, HEIGHT as i32);

            let draw_start = ((HEIGHT as i32 - wall_height) / 2).max(0) as u32;
            let draw_end = ((HEIGHT as i32 + wall_height) / 2).min(HEIGHT as i32) as u32;

            let color = match rec.side {
                Side::X => Color::new(0, 0, 255, 255),
                Side::Y => Color::new(255, 0, 0, 255),
            };

            for y in draw_start..draw_end {
                let index = (((y * WIDTH) + x) * 4) as usize;

                frame[index] = color.r;
                frame[index + 1] = color.g;
                frame[index + 2] = color.b;
                frame[index + 3] = color.a;
            }
        }
    }
}
