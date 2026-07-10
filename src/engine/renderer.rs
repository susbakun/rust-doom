use crate::{
    asset_manager::AssetManager,
    camera::Camera,
    color::Color,
    hitrecord::{HitRecord, Side},
    math::Vec2,
    prelude::{CEILING_TEXTUREID, FLOOR_TEXTUREID, HEIGHT, TEX_HEIGHT, TEX_WIDTH, WIDTH},
    ray::Ray,
    world::World,
};

pub fn render(world: &mut World, camera: &Camera, asset_manager: &AssetManager, frame: &mut [u8]) {
    let dir = camera.get_front();
    let plane = Vec2::new(-dir.y, dir.x) * 0.66;

    let floor_texture = &asset_manager.textures[FLOOR_TEXTUREID];
    let ceiling_texture = &asset_manager.textures[CEILING_TEXTUREID];

    let ray_dir_x_0 = dir.x - plane.x;
    let ray_dir_y_0 = dir.y - plane.y;
    let ray_dir_x_1 = dir.x + plane.x;
    let ray_dir_y_1 = dir.y + plane.y;

    // floor and ceiling
    for y in (HEIGHT / 2) + 1..HEIGHT {
        let p = y - HEIGHT / 2;

        let pos_z = HEIGHT as f64 / 2.0;

        let row_distance = pos_z / p as f64;

        let floor_step_x = row_distance * (ray_dir_x_1 - ray_dir_x_0) / WIDTH as f64;
        let floor_step_y = row_distance * (ray_dir_y_1 - ray_dir_y_0) / WIDTH as f64;

        let mut floor_x = camera.position().x + row_distance * ray_dir_x_0;
        let mut floor_y = camera.position().y + row_distance * ray_dir_y_0;

        for x in 0..WIDTH {
            let cell_x = floor_x.floor();
            let cell_y = floor_y.floor();

            let tx = ((floor_x - cell_x) * TEX_WIDTH as f64) as usize;
            let ty = ((floor_y - cell_y) * TEX_WIDTH as f64) as usize;

            floor_x += floor_step_x;
            floor_y += floor_step_y;

            let tex_index = (TEX_WIDTH * ty + tx) * 4;
            let color = get_color(&floor_texture.colors, tex_index);

            let frame_index = (((y * WIDTH) + x) * 4) as usize;
            write_to_framebuffer(frame, frame_index, &color);

            let color = get_color(&ceiling_texture.colors, tex_index);

            let frame_index = ((((HEIGHT - y - 1) * WIDTH) + x) * 4) as usize;
            write_to_framebuffer(frame, frame_index, &color);
        }
    }

    // walls
    for x in 0..WIDTH {
        let mut rec = HitRecord::default();

        let camera_x = (2.0 * x as f64 / WIDTH as f64) - 1.0;
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

            // preparing texture data
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

            let wall_texture = &asset_manager.textures[rec.texture_id];

            for y in draw_start..draw_end {
                let tex_y = tex_pos as usize;
                let tex_index = (TEX_HEIGHT * tex_y + tex_x) * 4;
                tex_pos += step;
                let color = get_wall_color(&wall_texture.colors, tex_index, &rec.side, perp_dist);

                let frame_index = (((y * WIDTH) + x) * 4) as usize;

                write_to_framebuffer(frame, frame_index, &color);
            }
            world.z_buffer[x as usize] = perp_dist;
        }
    }

    // sprites
    for sprite_data in world.sprites.iter_mut() {
        sprite_data.distance = camera.position().distance(sprite_data.sprite.pos);
    }

    world.sort();

    for sprite_data in &world.sprites {
        let sprite = &sprite_data.sprite;
        let sprite_texture = &asset_manager.textures[sprite.texture_id];

        let sprite_x = sprite.pos.x - camera.position().x;
        let sprite_y = sprite.pos.y - camera.position().y;

        //transform sprite with the inverse camera matrix
        // [ planeX   dirX ] -1                                       [ dirY      -dirX ]
        // [               ]       =  1/(planeX*dirY-dirX*planeY) *   [                 ]
        // [ planeY   dirY ]                                          [ -planeY  planeX ]

        let inv_det = 1.0 / (plane.x * dir.y - dir.x * plane.y);

        let transform_x = inv_det * (dir.y * sprite_x - dir.x * sprite_y);
        let transform_y = inv_det * (-plane.y * sprite_x + plane.x * sprite_y);

        let sprite_screen_x =
            ((WIDTH as f64 / 2.0) * (1.0 + transform_x / transform_y)).floor() as isize;

        //calculate height of the sprite on screen
        let sprite_height = (HEIGHT as f64 / transform_y).abs().floor() as isize;
        let draw_start_y = (-sprite_height / 2 + HEIGHT as isize / 2).max(0);
        let draw_end_y = (sprite_height / 2 + HEIGHT as isize / 2).min(HEIGHT as isize - 1);

        //calculate width of the sprite
        let sprite_width = (HEIGHT as f64 / transform_y).abs().floor() as isize;
        let draw_start_x = (-sprite_width / 2 + sprite_screen_x).max(0);
        let draw_end_x = (sprite_width / 2 + sprite_screen_x).min(WIDTH as isize - 1);

        for x in draw_start_x..draw_end_x {
            let tex_x = ((x - (-sprite_width / 2 + sprite_screen_x)) * TEX_WIDTH as isize
                / sprite_width) as usize;

            if transform_y > 0.0
                && x > 0
                && x < WIDTH as isize
                && transform_y < world.z_buffer[x as usize]
            {
                for y in draw_start_y..draw_end_y {
                    let d = y as isize - HEIGHT as isize / 2 + sprite_height / 2;

                    let tex_y = ((d * TEX_HEIGHT as isize) / sprite_height as isize) as usize;

                    let tex_index = ((TEX_WIDTH * tex_y) + tex_x) * 4;

                    let color = get_color(&sprite_texture.colors, tex_index);
                    let frame_index = ((WIDTH as usize * y as usize) + x as usize) * 4;

                    // Sprite assets use pure black (0,0,0) as a colorkey for transparency,
                    // since source PNGs have no alpha channel.
                    if color.r == 0 && color.g == 0 && color.b == 0 {
                        continue;
                    }

                    write_to_framebuffer(frame, frame_index, &color);
                }
            }
        }
    }
}

fn get_color(texture_colors: &Vec<u8>, tex_index: usize) -> Color {
    let r = texture_colors[tex_index];
    let g = texture_colors[tex_index + 1];
    let b = texture_colors[tex_index + 2];
    let a = texture_colors[tex_index + 3];

    Color { r, g, b, a }
}

fn get_wall_color(texture_colors: &Vec<u8>, tex_index: usize, side: &Side, distance: f64) -> Color {
    let mut r = texture_colors[tex_index];
    let mut g = texture_colors[tex_index + 1];
    let mut b = texture_colors[tex_index + 2];
    let a = texture_colors[tex_index + 3];

    let mut brightness = 1.0 / (1.0 + distance * 0.1);

    if *side == Side::Y {
        brightness *= 0.5;
    }

    r = (r as f64 * brightness) as u8;
    g = (g as f64 * brightness) as u8;
    b = (b as f64 * brightness) as u8;

    Color { r, g, b, a }
}

fn write_to_framebuffer(frame: &mut [u8], frame_index: usize, color: &Color) {
    frame[frame_index] = color.r;
    frame[frame_index + 1] = color.g;
    frame[frame_index + 2] = color.b;
    frame[frame_index + 3] = color.a;
}
