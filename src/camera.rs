use std::f64::consts::TAU;

use crate::math::{Point2, Vec2};

pub struct Camera {
    position: Point2,
    speed: f64,
    yaw: f64,
    rotation_speed: f64,
}

impl Camera {
    pub fn new(position: Point2, speed: f64, yaw: f64, rotation_speed: f64) -> Self {
        Self {
            position,
            speed,
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

    pub fn move_forward(&mut self, dt: f64) {
        self.position.x += self.yaw.cos() * self.speed * dt;
        self.position.y += self.yaw.sin() * self.speed * dt;
    }

    pub fn strafe_right(&mut self, dt: f64) {
        self.position.x -= self.yaw.sin() * self.speed * dt;
        self.position.y += self.yaw.cos() * self.speed * dt;
    }

    pub fn move_backward(&mut self, dt: f64) {
        self.position.x -= self.yaw.cos() * self.speed * dt;
        self.position.y -= self.yaw.sin() * self.speed * dt;
    }

    pub fn strafe_left(&mut self, dt: f64) {
        self.position.x += self.yaw.sin() * self.speed * dt;
        self.position.y -= self.yaw.cos() * self.speed * dt;
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
