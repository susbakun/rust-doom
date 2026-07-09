use std::f64::consts::TAU;

use crate::{
    math::{Point2, Vec2},
    world::World,
};

pub struct Camera {
    position: Point2,
    speed: f64,
    yaw: f64,
    radius: f64, // used for maintaining a distance between the player and objects
    rotation_speed: f64,
}

impl Camera {
    pub fn new(position: Point2, speed: f64, yaw: f64, radius: f64, rotation_speed: f64) -> Self {
        Self {
            position,
            speed,
            radius,
            yaw,
            rotation_speed,
        }
    }

    pub fn position(&self) -> Point2 {
        self.position
    }

    pub fn get_front(&self) -> Vec2 {
        let x = self.yaw.cos();
        let y = self.yaw.sin();

        Vec2 { x, y }.normalize()
    }

    pub fn move_forward(&mut self, dt: f64, world: &World) {
        let new_pos_x = self.position.x + self.yaw.cos() * self.speed * dt;
        let new_pos_y = self.position.y + self.yaw.sin() * self.speed * dt;

        self.try_move(new_pos_x, new_pos_y, world);
    }

    pub fn strafe_right(&mut self, dt: f64, world: &World) {
        let new_pos_x = self.position.x - self.yaw.sin() * self.speed * dt;
        let new_pos_y = self.position.y + self.yaw.cos() * self.speed * dt;

        self.try_move(new_pos_x, new_pos_y, world);
    }

    pub fn move_backward(&mut self, dt: f64, world: &World) {
        let new_pos_x = self.position.x - self.yaw.cos() * self.speed * dt;
        let new_pos_y = self.position.y - self.yaw.sin() * self.speed * dt;

        self.try_move(new_pos_x, new_pos_y, world);
    }

    pub fn strafe_left(&mut self, dt: f64, world: &World) {
        let new_pos_x = self.position.x + self.yaw.sin() * self.speed * dt;
        let new_pos_y = self.position.y - self.yaw.cos() * self.speed * dt;

        self.try_move(new_pos_x, new_pos_y, world);
    }

    fn try_move(&mut self, new_pos_x: f64, new_pos_y: f64, world: &World) {
        let r = self.radius;

        if !world.is_wall(Point2::new(new_pos_x + r, self.position.y))
            && !world.is_wall(Point2::new(new_pos_x - r, self.position.y))
        {
            self.position.x = new_pos_x;
        }

        if !world.is_wall(Point2::new(self.position.x, new_pos_y + r))
            && !world.is_wall(Point2::new(self.position.x, new_pos_y - r))
        {
            self.position.y = new_pos_y;
        }
    }

    // used only for mouse move
    pub fn rotate(&mut self, diff: f64) {
        self.yaw = (self.yaw + diff).rem_euclid(TAU);
    }

    pub fn turn_left(&mut self, dt: f64) {
        self.rotate(-self.rotation_speed * dt);
    }

    pub fn turn_right(&mut self, dt: f64) {
        self.rotate(self.rotation_speed * dt);
    }
}
