use embedded_graphics::geometry::{Size, OriginDimensions};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::text::{TextStyleBuilder, Baseline, Text};
use embedded_graphics::image::Image;

use websocket::client::ClientBuilder;
use websocket::message::Message;
use websocket::client::sync::Client;

use tinybmp::Bmp;

struct FrancisScherm<S: std::io::Write + std::io::Read> {
    ws_client: Client<S>,
}

impl<S: std::io::Write + std::io::Read> FrancisScherm<S> {
    pub fn set_pixel(
        &mut self,
        x: i32,
        y: i32,
        color: &str
    ) {
        let message = Message::text(format!(
            "{{\"w\": {}, \"h\": {}, \"c\": \"{}\"}}",
            x,
            y,
            color
        ));

        self.ws_client.send_message(&message).unwrap();
    }
}

impl<S: std::io::Write + std::io::Read> OriginDimensions for FrancisScherm<S> {
    fn size(&self) -> Size {
        Size::new(400, 300)
    }
}

impl<S: std::io::Write + std::io::Read> DrawTarget for FrancisScherm<S>
{
    type Color = Rgb888;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>
    {
        for pixel in pixels {
            self.set_pixel(
                pixel.0.x,
                pixel.0.y,
                &format!("{:02x}{:02x}{:02x}", pixel.1.r(), pixel.1.g(), pixel.1.b()),
            );
        }
        Ok(())
    }
}

fn main() {
    let mut scherm = FrancisScherm {
        ws_client: ClientBuilder::new("ws://10.0.6.9:8000/draw")
                        .unwrap()
                        .connect_insecure()
                        .unwrap()
    };

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
    
    let bmp_data = include_bytes!("../poes888.bmp");
    let image = Bmp::<Rgb888>::from_slice(bmp_data).unwrap();
    Image::new(&image, Point::new(25,50))
        .draw(&mut scherm)
        .unwrap();

}
