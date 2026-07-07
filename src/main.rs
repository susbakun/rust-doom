#![allow(dead_code)]

use anyhow::Result;

use crate::engine::Engine;

mod camera;
mod engine;
mod hitrecord;
mod math;
mod prelude;
mod ray;
mod world;

fn main() -> Result<()> {
    let mut engine = Engine::new()?;

    engine.run()
}
