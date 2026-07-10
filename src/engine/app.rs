use std::f64::consts::PI;

use anyhow::Result;

use crate::{
    asset_manager::AssetManager,
    camera::Camera,
    engine::window::GameWindow,
    math::Point2,
    prelude::{MOVEMENT_SPEED, ROTATION_SPEED, WORLD_MAP},
    world::World,
};

pub struct Engine<'a> {
    camera: Camera,
    game_window: GameWindow<'a>,
    world: World,
    asset_manager: AssetManager,
}

impl<'a> Engine<'a> {
    pub fn new() -> Result<Self> {
        let camera = Camera::new(
            Point2::new(4.5, 3.5), // initial position
            MOVEMENT_SPEED,
            PI / 2.0, // looking at +Y,
            0.8,      // radius
            ROTATION_SPEED,
        );
        let game_window = GameWindow::new()?;

        let world = World::new(&WORLD_MAP);
        let mut asset_manager = AssetManager::new();
        asset_manager.load_assets()?;

        Ok(Self {
            camera,
            game_window,
            world,
            asset_manager,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        self.game_window
            .run_event_loop(&mut self.camera, &mut self.world, &self.asset_manager)
    }
}
