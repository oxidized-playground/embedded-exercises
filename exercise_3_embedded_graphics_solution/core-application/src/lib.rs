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

pub struct CoreApp {
    logo: Bmp<'static, Rgb565>,
    text: String,
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
            text: String::from("Hi I'm Ferris!"),
        }
    }

    pub fn draw<D: embedded_graphics::draw_target::DrawTarget<Color=Rgb565>>(
        &mut self,
        display: &mut D,
    ) -> Result<(), D::Error> {
        self.draw_logo(display)?;
        self.draw_text(display)?;
        Ok(())
    }

    fn draw_logo<D: embedded_graphics::draw_target::DrawTarget<Color=Rgb565>>(
        &mut self,
        display: &mut D,
    ) -> Result<(), D::Error> {
        let center = display.bounding_box().center();
        let logo_position = center.x_axis();
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
        let text_display = Text::with_alignment(
            &self.text.as_str(),
            center.x_axis()+ Point::new(60, 30),
            character_style,
            Alignment::Center,
        );
        text_display.draw(display)?;

        Ok(())
    }

}

