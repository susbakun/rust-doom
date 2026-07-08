use std::path::Path;

use anyhow::Result;
use image::{ImageReader, imageops::FilterType};

use crate::prelude::{TEX_HEIGHT, TEX_WIDTH};

pub struct Texture {
    pub colors: Vec<u8>,
}

impl Texture {
    pub fn new(image_name: &str) -> Result<Self> {
        let path = Path::new("./assets/textures").join(image_name);
        let img = ImageReader::open(path)?;
        let colors = img
            .decode()?
            .resize(TEX_WIDTH as u32, TEX_HEIGHT as u32, FilterType::Nearest)
            .to_rgba8()
            .to_vec();

        Ok(Texture { colors })
    }
}
