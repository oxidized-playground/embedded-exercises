#![no_std]
#![no_main]

use defmt::Format;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_println as _;
use esp_hal::{
    timer::timg::TimerGroup,
    gpio::{Input, InputConfig}
};
use embassy_sync::{
    watch::Watch,
    blocking_mutex::raw::CriticalSectionRawMutex
};

esp_bootloader_esp_idf::esp_app_desc!();

#[derive(Format, Clone)]
enum ButtonState {
    Pressed,
    Released,
}

static BUTTON_WATCH: Watch<CriticalSectionRawMutex, ButtonState, 2> = Watch::new();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let button_pin = Input::new(peripherals.GPIO35, InputConfig::default());
    // let second_button_pin = Input::new(peripherals.GPIO0, InputConfig::default());

    // TODO: Setup all async tasks in the spawner
    spawner.spawn(todo!("invoke print_button_state_task")).ok();
    spawner.spawn(todo!("invoke read_button_task with a button")).ok();
    spawner.spawn(todo!("invoke another_button_watching_task")).ok();
    // For additional challenge
    // spawner.spawn(another_button_publishing_task(second_button_pin)).ok();


    loop {
        defmt::info!("main loop!");
        // you could do stuff here, but for now we keep this empty
        Timer::after(Duration::from_millis(5_000)).await;
    }
}

#[embassy_executor::task]
async fn print_button_state_task() {
    // TODO: Build the receiving part of the watch. See https://docs.embassy.dev/embassy-sync/git/default/watch/struct.Watch.html for embassy sync options.
    // Subscribe to changes and print these in the console
    defmt::info!("start print_button_state_task");
    let button_watch_receiver_result = todo!("Add subscriber to BUTTON_WATCH");
    match button_watch_receiver_result {
        Some(mut button_watch_receiver) => {
            loop {
                let button_state = todo!("Read the button_watch_receiver here, subscribe via changed(), make sure to use await for its result");
                defmt::info!("button state: {:?}", button_state);
            }
        }
        None => { defmt::error!("no extra watchers available!") }
    }
}

#[embassy_executor::task]
async fn read_button_task(mut button: Input<'static>){
    defmt::info!("start read_button_task");
    let sender = BUTTON_WATCH.sender();
    loop {
        button.wait_for_falling_edge().await;
        todo!("Send ButtonState::Pressed to the sender");
        Timer::after(Duration::from_millis(5)).await; //debounce time
        button.wait_for_rising_edge().await;
        todo!("Send ButtonState::Released to the sender");
        Timer::after(Duration::from_millis(5)).await; //debounce time
    }
}


// optional: see what happens when you want to use multiple receivers for the watch.

#[embassy_executor::task]
async fn another_button_watching_task() {
    defmt::info!("start another_button_watching_task");
    todo!("Add another button watcher here and print the button state");
}


// (Optional and advanced)
// Create another task that also produces data. So, 2 producers and 2 consumers. Maybe you need another
// async datatype: https://docs.embassy.dev/embassy-sync/git/default/index.html

// use embassy_sync::pubsub::PubSubChannel;
// static BUTTON_PUB_SUB: PubSubChannel<CriticalSectionRawMutex, ButtonState, 1, 2, 2> = PubSubChannel::new();
//
// #[embassy_executor::task]
// async fn another_button_publishing_task(mut button: Input<'static>){
//     defmt::info!("start another_button_publishing_task");
//     let publisher = BUTTON_PUB_SUB.publisher().unwrap();
//     loop {}
//          todo!("Wait for button falling edge");
//          todo!("Publish results to publisher")
//     }
// }
//
// #[embassy_executor::task]
// async fn yet_another_button_watching_task() {
//     defmt::info!("start yet_another_button_watching_task");
//     let mut consumer = todo!("Create consumer from this pub sub channel");
//
//     loop {
//         todo!("Consume from pub sub and log with defmt");
//         defmt::info!(todo!("Your log here"));
//     }
// }