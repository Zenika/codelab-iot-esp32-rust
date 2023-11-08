use std::thread;
use std::time::Duration;

use esp_idf_hal::gpio::{PinDriver, Pull};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::task::block_on;

use esp_idf_sys::EspError;

use log::*;

fn main() -> Result<(), EspError> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let mut led_pin = PinDriver::input_output(peripherals.pins.gpio2).unwrap();
    led_pin.set_low().unwrap();

    let mut interrupt_pin = PinDriver::input(peripherals.pins.gpio4).unwrap();
    interrupt_pin.set_pull(Pull::Up).unwrap();


    let _ = std::thread::Builder::new()
      .spawn(move || {
        loop {
            let _ = block_on(interrupt_pin.wait_for_rising_edge()); //wait_for_falling_edge());
    
            // Do something
            match interrupt_pin.get_level() {
                esp_idf_hal::gpio::Level::High => info!("Level of Pin4 HIGH"),
                esp_idf_hal::gpio::Level::Low => info!("Level of Pin4 LOW"),
             }
    
            match led_pin.get_level() {
               esp_idf_hal::gpio::Level::High => led_pin.set_low().unwrap(),
               esp_idf_hal::gpio::Level::Low => led_pin.set_high().unwrap(),
            }
        }
    }).unwrap();


    loop {
        info!("Waiting...");
        thread::sleep(Duration::from_millis(2000));
    }
}
