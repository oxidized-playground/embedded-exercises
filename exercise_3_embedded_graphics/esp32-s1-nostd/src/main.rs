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


#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use core_application::CoreApp;

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
 
use mipidsi::{
    interface::SpiInterface, models::ST7789, options::{ColorInversion, Orientation, Rotation}
};

use esp_hal::{
    gpio::{
        Level,
        Output,
        OutputConfig
    },
    spi::{
        Mode,
        master::{
            Spi,
            Config,
        },
    },
    timer::timg::TimerGroup,
    time::Rate,
    delay::Delay,
};

use embedded_hal_bus::spi::ExclusiveDevice;

use esp_println as _;
use esp_backtrace as _;

const TFT_WIDTH: u16 = 135;
const TFT_HEIGHT: u16 = 240;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {

    // Borrow the needed Peripherals and set the pins
    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);
    let spi = peripherals.SPI2;
    let mosi = peripherals.GPIO19;
    let sclk = peripherals.GPIO18;
    let pin_chip_select   = peripherals.GPIO5;
    let pin_spi_datacommand   = peripherals.GPIO16;
    let pin_reset = peripherals.GPIO23;
    let pin_backlight   = peripherals.GPIO4;

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    defmt::info!("init peripherals completed...");


    // Initialize the SPI interface
    // https://docs.rs/esp32-hal/latest/esp32_hal/spi/index.html
    let spi_bus = Spi::new(
            spi,
            Config::default()
                .with_frequency(Rate::from_mhz(26))
                .with_mode(Mode::_0)
            ).unwrap()
            .with_sck(sclk)
            .with_mosi(mosi);

    // Configure other pins like 
    // chip select = High 
    // data command = Low
    // reset = High
    // backlight = Low 
    // https://docs.rs/esp32-hal/latest/esp32_hal/gpio/struct.Output.html       
    let config = OutputConfig::default();
    let cs_output = Output::new(pin_chip_select, Level::High, config);
    let dc_output = Output::new(pin_spi_datacommand, Level::Low, config);
    let rst_output = Output::new(pin_reset, Level::High, config);
    let backlight_output = todo!("Define new output that is default High");

    // Create a SpiDevice from the SpiBus implementation so that it can be used by the SpiInterface which uses the interafce:Interface trait
    // https://docs.rs/embedded-hal-bus/latest/embedded_hal_bus/spi/struct.ExclusiveDevice.html       
    let spi_device = ExclusiveDevice::new_no_delay(spi_bus, cs_output).unwrap();    

    // Create Display interface
    let mut buffer = [0_u8; 512];
    let di = SpiInterface::new(
        spi_device,    
        dc_output,
        &mut buffer
    );

    // Initialize the display
    let mut delay = Delay::new();
    
    // The TTGO board's screen does not start at offset 0x0, and the physical size is 135x240, instead of 240x320
    // PS: you don't have to change this 🙂
    // https://docs.rs/mipidsi/0.9.0/mipidsi/
    let mut display = mipidsi::Builder::new(ST7789, di)
        .reset_pin(rst_output)
        .display_size(TFT_WIDTH.into(), TFT_HEIGHT.into())
        .display_offset(52, 40)
        // TODO: But you will need to add extra settings to properly initialize your display 🙂
        .init(&mut delay).unwrap();


    // Implement new CoreApp so we can use it to display Ferris! 
    let mut esp_display = CoreApp::new();

    defmt::info!("Ready to draw some displays!");

    display.clear(Rgb565::WHITE).unwrap();
    
    loop{    
        // TODO: call the draw method on the CoreApp instance
        todo!();

        Timer::after(Duration::from_millis(500)).await;
    }
}