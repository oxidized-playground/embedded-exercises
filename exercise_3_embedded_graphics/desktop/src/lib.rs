use core_application::CoreApp;

use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle};

use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use wasm_bindgen::prelude::*;
use web_sys::console;

const TFT_HEIGHT: u16 = 135;
const TFT_WIDTH: u16 = 240;

#[wasm_bindgen(start)]
fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Create a window in the web environment by setting a body
    let document = web_sys::window()
        .expect("could not get window")
        .document()
        .expect("could not get document");
    let body = document.body().expect("could not get document body");

    body.set_inner_html(
        r#"
        <header>
           🦀 Embedded Graphics Web Simulator! 🦀
        </header>

        "#,
    );

    // Configure the output settings for the Web Simulator
    let output_settings = OutputSettingsBuilder::new().scale(2).build();

    // Init displays
    let mut display = WebSimulatorDisplay::new(
        (TFT_WIDTH.into(), TFT_HEIGHT.into()),
        &output_settings,
        None,
    );
    let mut desk_display = CoreApp::new();

    // Set origin for drawing Ferris
    let top_left = Point::new(0, 0);
    let size = Size::new(TFT_WIDTH.into(), TFT_HEIGHT.into());

    display.clear(Rgb565::WHITE).unwrap();
    desk_display
        .draw(&mut display.cropped(&Rectangle::new(top_left, size)))
        .unwrap_or_else(|_| console::log_1(&"Couldn't draw image".into()));
    display.flush().expect("Couldn't update");

    Ok(())
}
