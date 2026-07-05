use std::f64::consts::PI;

use anyhow::Result;

use crate::{
    camera::Camera,
    engine::window::GameWindow,
    math::Vec2,
    prelude::{MOVEMENT_SPEED, ROTATION_SPEED},
};

pub struct Engine {
    camera: Camera,
    game_window: GameWindow,
}

impl Engine {
    pub fn new() -> Result<Self> {
        let camera = Camera::new(
            Vec2::new(0.0, 0.0), // initial position
            MOVEMENT_SPEED,
            PI / 2.0, // looking at +Y
            ROTATION_SPEED,
        );
        let game_window = GameWindow::new()?;

        Ok(Self {
            camera,
            game_window,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        self.game_window.run_event_loop(&mut self.camera)
    }
}
