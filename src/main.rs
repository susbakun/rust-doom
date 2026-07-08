#![allow(dead_code)]

use anyhow::Result;

use crate::engine::Engine;

mod camera;
mod color;
mod engine;
mod hitrecord;
mod math;
mod prelude;
mod ray;
mod texture;
mod world;

fn main() -> Result<()> {
    let mut engine = Engine::new()?;

    engine.run()
}
