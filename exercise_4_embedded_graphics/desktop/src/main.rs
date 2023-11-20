use std::{
    thread,
    time::Duration,
};

use core_application::CoreApp;

use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565, primitives::Rectangle,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder,
    SimulatorDisplay,
    SimulatorEvent,
    Window,
};

const TFT_HEIGHT: u16 = 135;
const TFT_WIDTH: u16 = 240;

fn main() -> Result<(), core::convert::Infallible>{

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("ESP32 TTGO Simulator", &output_settings);

    // Init displays
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(TFT_WIDTH.into(), TFT_HEIGHT.into()));
    let mut desk_display = CoreApp::new();

    // Set origin for drawing Ferris
    let top_left = Point::new(-120, 0);
    let size = Size::new(TFT_WIDTH.into(), TFT_HEIGHT.into());

    // Run the "event" loop
    'running: loop {

        window.update(&display);
        display.clear(Rgb565::WHITE).unwrap();
        desk_display.draw(&mut display.cropped(&Rectangle::new(top_left, size)))?;
        thread::sleep(Duration::from_millis(500));

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
    }
    Ok(())
}