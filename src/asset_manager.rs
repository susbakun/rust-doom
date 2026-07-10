use anyhow::Result;

use crate::texture::Texture;

pub struct AssetManager {
    pub textures: Vec<Texture>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self { textures: vec![] }
    }

    pub fn load_assets(&mut self) -> Result<()> {
        let wall_texture = Texture::new("redbrick.png")?;
        let floor_texture = Texture::new("greystone.png")?;
        let ceiling_texture = Texture::new("wood.png")?;

        self.textures.push(wall_texture);
        self.textures.push(floor_texture);
        self.textures.push(ceiling_texture);

        Ok(())
    }
}
