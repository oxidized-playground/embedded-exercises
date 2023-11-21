//! TTGO T ESP 32 main
//!
//! This example prints some text on an st7789-based
//! display (via eSPI)
//!
//! TFT_WIDTH  240
//! TFT_HEIGHT 135
//! 
//! The following wiring is assumed:
//! - SPI      => SPI2
//! - TFT_MOSI => GPIO19
//! - TFT_SCLK => GPIO18
//! - TFT_CS   => GPIO5
//! - TFT_DC   => GPIO16
//! - TFT_RST  => GPIO23
//! - TFT_BL   => GPIO4
//! 
//! DEFAULT SPI_FREQUENCY 26000000
//! MAX SPI_FREQUENCY  40000000
//! MAX SPI_READ_FREQUENCY  6000000
//!
//! https://github.com/esp-rs/esp-hal/blob/main/esp32-hal/examples/i2c_display.rs


use std::thread;
use std::time::Duration;

use core_application::CoreApp;

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*, primitives::Rectangle,
};

use display_interface_spi::SPIInterfaceNoCS;

use mipidsi::{
    Builder,
    Orientation,
};

use esp_idf_hal::{
    gpio::*,
    spi::*,
    delay::Ets,
    peripherals::Peripherals,
    prelude::*,
};

use esp_idf_sys::{self as _}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported


const TFT_HEIGHT: u16 = 135;
const TFT_WIDTH: u16 = 240;

fn main()-> anyhow::Result<()>{

    // *Take* the needed Peripherals and set the pins
    // https://esp-rs.github.io/esp-idf-hal/esp_idf_hal/peripherals/struct.Peripherals.html
    let peripherals = todo!();
    let spi = todo!();
    let mosi = todo!();
    let sclk = todo!();
    let cs   = todo!();
    let dc   = todo!();
    let rst  = todo!();
    let bl   = todo!();
   

    println!("init peripherals completed...");

    // Implement a new display interface based on a SPI driver
    // https://docs.rs/display-interface-spi/latest/display_interface_spi/
    // https://esp-rs.github.io/esp-idf-hal/esp_idf_hal/spi/struct.SpiDeviceDriver.html
    // Hint: the pinconfiguration can be found in the comments at the top of this file
    // Hint: look up the docs of the SpiDeviceDriver
    
    //let di = SPIInterfaceNoCS::new();
    

    // Pass the display interface to the ST7789 Builder and init the Builder
    // https://docs.rs/mipidsi/0.7.1/mipidsi/
    // https://esp-rs.github.io/esp-idf-hal/esp_idf_hal/delay/index.html
    let mut display = Builder::st7789(...)
        .with_display_size(TFT_WIDTH, TFT_HEIGHT)
        .with_orientation(Orientation::Landscape(true))
        //Some extra display options can be put here 🤔
        .init(&mut Ets, Some(PinDriver::output(rst)?)) 
        .map_err(|e| anyhow::anyhow!("Display error: {:?}", e))?;

    
    // Configure the backlight pin (TFT_BL) to output and set the pin on HIGH
    // https://esp-rs.github.io/esp-idf-hal/esp_idf_hal/gpio/struct.PinDriver.html
    //backlight

    // Implement esp_display with CoreApp so we can use it to display Ferris! 
    //esp_display

    // The TTGO board's screen does not start at offset 0x0, and the physical size is 135x240
    // You don't have to change this 🙂
    let top_left = Point::new(52, 52);
    let size = Size::new(TFT_WIDTH.into(), TFT_HEIGHT.into());
    
    display.clear(Rgb565::WHITE).unwrap();
    
    loop{
        esp_display.draw(&mut display.cropped(&Rectangle::new(top_left, size))).unwrap();      
     
        thread::sleep(Duration::from_millis(500));
    }
}
