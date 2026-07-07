use crate::{
    camera::Camera,
    hitrecord::HitRecord,
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
            let wall_height = (HEIGHT as f64 / rec.distance) as i32;
            let wall_height = wall_height.min(HEIGHT as i32);

            let draw_start = ((HEIGHT as i32 - wall_height) / 2).max(0) as u32;
            let draw_end = ((HEIGHT as i32 + wall_height) / 2).min(HEIGHT as i32) as u32;

            for y in draw_start..draw_end {
                let index = (((y * WIDTH) + x) * 4) as usize;

                frame[index] = 255;
                frame[index + 1] = 255;
                frame[index + 2] = 255;
                frame[index + 3] = 255;
            }
        }
    }
}
