#![allow(dead_code)]

use anyhow::Result;

use crate::engine::Engine;

mod camera;
mod engine;
mod math;
mod prelude;

fn main() -> Result<()> {
    let mut engine = Engine::new()?;

    engine.run()
}
