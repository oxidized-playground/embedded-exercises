#![no_std]

use embedded_graphics::{
    image::Image,
    mono_font::{
        ascii::FONT_10X20,
        MonoTextStyle,
    },
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, Text},
};
use tinybmp::Bmp;
use heapless::String;

pub struct CoreApp {
    logo: Bmp<'static, Rgb565>,
    text: String<50>,
}

impl Default for CoreApp {
    fn default() -> Self {
        return CoreApp::new();
    }
}

impl CoreApp {
    pub fn new() -> CoreApp {
        CoreApp {
            logo: Bmp::from_slice(include_bytes!("../assets/ferris.bmp"))
                .unwrap(),
            text: String::try_from("Hi I'm Ferris!").unwrap(),
        }
    }

    pub fn draw<D: embedded_graphics::draw_target::DrawTarget<Color=Rgb565>>(
        &mut self,
        display: &mut D,
    ) -> Result<(), D::Error> {
        self.draw_logo(display)?;
        // Uncomment the following line to draw the text!
        // self.draw_text(display)?;
        Ok(())
    }

    fn draw_logo<D: embedded_graphics::draw_target::DrawTarget<Color=Rgb565>>(
        &mut self,
        display: &mut D,
    ) -> Result<(), D::Error> {
        let display_center = display.bounding_box().center().x_axis();
        let logo_center = self.logo.bounding_box().center().x_axis();
        let logo_position = display_center - logo_center;
        let image = Image::new(&self.logo, logo_position);
        image.draw(display)?;

        Ok(())
    }
    
    // Implement the draw_text function so we can show the text "Hi I'm Ferris!"
    // https://docs.rs/embedded-graphics/latest/embedded_graphics/text/index.html
    fn draw_text<D: embedded_graphics::draw_target::DrawTarget<Color=Rgb565>>(
        &mut self,
        display: &mut D,
        ) -> Result<(), D::Error> {
        let center = display.bounding_box().center();
        let character_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_ORANGE);
        let text_display = todo!();

        // text_display.draw(display)?;

        Ok(())
    }

    // Additional:
    // Implement a function to draw a shape of your choice
    // https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/index.html
    // fn draw_shape<D: embedded_graphics::draw_target::DrawTarget<Color=Rgb565>>(
    //     &mut self,
    //     display: &mut D,
    // ) -> Result<(), D::Error> {
    //     todo!();
    //     Ok(())
    // }

}

