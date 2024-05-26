#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, embassy, peripherals::Peripherals, timer::TimerGroup, prelude::*};
use esp_hal::gpio::{AnyPin, Input, PullUp, IO};

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let del_but = io.pins.gpio35.into_pull_up_input().degrade();
    
    esp_println::logger::init_logger_from_env();

    embassy::init(
        &clocks,
        TimerGroup::new_async(peripherals.TIMG0, &clocks)
    );

    esp_hal::interrupt::enable(
        esp_hal::peripherals::Interrupt::GPIO,
        esp_hal::interrupt::Priority::Priority1,
    ).unwrap();

    spawner.spawn(one_second_task()).unwrap();
    spawner.spawn(press_button(del_but.into())).unwrap();

    loop {
        log::info!("Main loop waiting idle...");
        Timer::after(Duration::from_millis(5_000)).await;
    }
}

#[embassy_executor::task]
async fn one_second_task() {
  let mut count = 0;
    loop {
        log::warn!("Spawn Task Count: {}", count);
        count += 1;
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[embassy_executor::task]
async fn press_button(mut button: AnyPin<Input<PullUp>>){
    button.wait_for_rising_edge().await;
    log::error!("Button Pressed!");
}

// Create another task here, log something periodically and see that it does not depend on the other task timer
async fn my_new_task() {

}


// (Optional and advanced)
// Make the tasks communicate via a Channel. Search online for "Sharing peripherals between tasks" to help you get started on sharing information between tasks.