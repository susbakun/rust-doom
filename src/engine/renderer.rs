use crate::{
    camera::Camera,
    color::Color,
    hitrecord::{HitRecord, Side},
    math::Vec2,
    prelude::{HEIGHT, TEX_HEIGHT, TEX_WIDTH, WIDTH},
    ray::Ray,
    texture::Texture,
    world::World,
};

pub fn render(world: &World, camera: &Camera, texture: &Texture, frame: &mut [u8]) {
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

            // finding texture coordinates
            let mut wallx = match rec.side {
                Side::X => ray.origin().y + perp_dist * ray_dir.y,
                Side::Y => ray.origin().x + perp_dist * ray_dir.x,
            };
            wallx -= wallx.floor();

            let mut tex_x = (wallx * TEX_WIDTH as f64) as usize;

            if rec.side == Side::X && ray_dir.x > 0.0 {
                tex_x = TEX_WIDTH - tex_x - 1;
            }
            if rec.side == Side::Y && ray_dir.y < 0.0 {
                tex_x = TEX_WIDTH - tex_x - 1;
            }

            let step = 1.0 * TEX_HEIGHT as f64 / wall_height as f64;
            let mut tex_pos = 0.0;

            for y in draw_start..draw_end {
                let tex_y = tex_pos as usize;
                let tex_index = (TEX_HEIGHT * tex_y + tex_x) * 4;
                tex_pos += step;

                let frame_index = (((y * WIDTH) + x) * 4) as usize;

                let color = get_color(&texture.colors, tex_index, &rec.side, perp_dist);

                frame[frame_index] = color.r;
                frame[frame_index + 1] = color.g;
                frame[frame_index + 2] = color.b;
                frame[frame_index + 3] = color.a;
            }
        }
    }
}

fn get_color(texture_colors: &Vec<u8>, tex_index: usize, side: &Side, distance: f64) -> Color {
    let mut r = texture_colors[tex_index];
    let mut g = texture_colors[tex_index + 1];
    let mut b = texture_colors[tex_index + 2];
    let mut a = texture_colors[tex_index + 3];

    let mut brightness = 1.0 / (1.0 + distance * 0.1);

    if *side == Side::Y {
        brightness *= 0.5;
    }

    r = (r as f64 * brightness) as u8;
    g = (g as f64 * brightness) as u8;
    b = (b as f64 * brightness) as u8;

    Color { r, g, b, a }
}
