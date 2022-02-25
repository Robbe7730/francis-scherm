use embedded_graphics::geometry::{Size, OriginDimensions};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::text::{TextStyleBuilder, Baseline, Text};
use embedded_graphics::image::Image;

use tinybmp::Bmp;

use rayon::prelude::*;

struct FrancisScherm {}

impl FrancisScherm {
    pub fn set_pixel(
        &self,
        x: u32,
        y: u32,
        r: u8,
        g: u8,
        b: u8
    ) {
        let client = reqwest::blocking::Client::new();
        client.post(format!(
            "http://10.1.0.198:8000/{}/{}/{}/{}/{}",
            x, y, r, g, b
        )).send().unwrap();
    }
}

impl OriginDimensions for FrancisScherm {
    fn size(&self) -> Size {
        Size::new(400, 300)
    }
}

impl DrawTarget for FrancisScherm
{
    type Color = Rgb888;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>
    {
        pixels.into_iter().collect::<Vec<_>>().par_iter().for_each(|pixel| {
            self.set_pixel(
                pixel.0.x as u32,
                pixel.0.y as u32,
                pixel.1.r(),
                pixel.1.g(),
                pixel.1.b()
            );
        });
        Ok(())
    }
}

fn main() {
    let mut scherm = FrancisScherm {};

    let character_style = MonoTextStyle::new(&FONT_6X10, Rgb888::GREEN);

    let text_style = TextStyleBuilder::new()
        .baseline(Baseline::Top)
        .build();

    Text::with_text_style(
        "Pieter",
        Point::new(150, 100),
        character_style,
        text_style,
    ).draw(&mut scherm).unwrap();
    
    // let bmp_data = include_bytes!("../poes888.bmp");
    // let image = Bmp::<Rgb888>::from_slice(bmp_data).unwrap();
    // Image::new(&image, Point::new(25,50))
    //     .draw(&mut scherm)
    //     .unwrap();

}
