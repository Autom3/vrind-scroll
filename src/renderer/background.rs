use quicksilver::{
    Result,
    geom::{Shape, Transform},
    graphics::{Background::Img, Font, Image},
    lifecycle::{Asset, Window}
};

use super::Renderer;

pub struct Background {
    asset: Asset<Image>,
}

impl Background {
    pub fn new() -> Self {
        let asset = Asset::new(Image::load("images/image.png"));
        Background { asset }
    }
}

impl Renderer for Background {

    fn render(&mut self, window: &mut Window, _: &mut Asset<Font>) -> Result<()> {
        self.asset.execute(|image| {
            window.draw_ex(&image.area().with_center((500, 500)), Img(&image), Transform::IDENTITY, -10);
            Ok(())
        })
    }
}

