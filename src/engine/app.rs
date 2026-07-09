use std::f64::consts::PI;

use anyhow::Result;

use crate::{
    camera::Camera,
    engine::window::GameWindow,
    math::Point2,
    prelude::{MOVEMENT_SPEED, ROTATION_SPEED},
};

pub struct Engine<'a> {
    camera: Camera,
    game_window: GameWindow<'a>,
}

impl<'a> Engine<'a> {
    pub fn new() -> Result<Self> {
        let camera = Camera::new(
            Point2::new(4.5, 3.5), // initial position
            MOVEMENT_SPEED,
            PI / 2.0, // looking at +Y,
            1.0,
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
