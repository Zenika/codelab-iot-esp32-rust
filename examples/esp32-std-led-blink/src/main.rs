
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use std::{thread, time::Duration};
use esp_idf_sys::EspError;

fn main() -> Result<(), EspError> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let mut led_pin = PinDriver::output(peripherals.pins.gpio0).unwrap();

    loop {
        led_pin.set_high().unwrap();
        thread::sleep(Duration::from_millis(1000));
    
        led_pin.set_low().unwrap();
        thread::sleep(Duration::from_millis(1000));
    
        println!("blink");
    }
}
