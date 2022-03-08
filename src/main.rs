use embedded_graphics::geometry::{Size, OriginDimensions};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::text::{TextStyleBuilder, Baseline, Text};

use rayon::prelude::*;

use rand::{thread_rng, Rng};

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

    let mut rng = thread_rng();

    loop {
        let x: u32 = rng.gen_range(0..800);
        let y: u32 = rng.gen_range(0..600);

        Text::with_text_style(
            "Niko",
            Point::new(x.try_into().unwrap(), y.try_into().unwrap()),
            character_style,
            text_style,
        ).draw(&mut scherm).unwrap();
    }
}
