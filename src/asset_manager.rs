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
        //load some textures
        let eagle = Texture::new("eagle.png")?;
        let redbrick = Texture::new("redbrick.png")?;
        let purplestone = Texture::new("purplestone.png")?;
        let greystone = Texture::new("greystone.png")?;
        let bluestone = Texture::new("bluestone.png")?;
        let mossy = Texture::new("mossy.png")?;
        let wood = Texture::new("wood.png")?;
        let colorstone = Texture::new("colorstone.png")?;

        //load some sprite textures
        let barrel = Texture::new("barrel.png")?;
        let pillar = Texture::new("pillar.png")?;
        let greenlight = Texture::new("greenlight.png")?;

        self.textures.push(eagle);
        self.textures.push(redbrick);
        self.textures.push(purplestone);
        self.textures.push(greystone);
        self.textures.push(bluestone);
        self.textures.push(mossy);
        self.textures.push(wood);
        self.textures.push(colorstone);
        self.textures.push(barrel);
        self.textures.push(pillar);
        self.textures.push(greenlight);

        Ok(())
    }
}
