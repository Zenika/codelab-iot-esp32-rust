use esp_idf_hal::units::KiloHertz;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::thread;
use std::time::Duration;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::i2c::I2cDriver;
use embedded_hal::blocking::{i2c, delay::DelayMs};

// LCD
use liquidcrystal_i2c_rs::Lcd;


// implement HAL...
struct I2cEsp32Driver<'d> {
  i2c: I2cDriver<'d>,
}

type I2cError = std::io::Error;

// Implement the `Hardware` trait to give access to LCD pins
impl i2c::Write for I2cEsp32Driver<'_> {
    type Error = I2cError;

    fn write(&mut self, address: u8, data: &[u8]) -> Result<(), Self::Error> {
      if let Ok(()) = self.i2c.write(address, data, 1000) {
        return Ok(());
      }
  
      return Err(Self::Error::new(std::io::ErrorKind::Other, "Error to write data"));
    }
}

struct DelayEsp32Driver { }

// Implement the `Delay` trait to allow library to sleep for the given amount of time
impl DelayMs<u8> for DelayEsp32Driver {
  fn delay_ms(&mut self, delay_usec: u8) {
    thread::sleep(Duration::from_millis(delay_usec.into()));
  }
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let scl = peripherals.pins.gpio22;
    let sda = peripherals.pins.gpio21;

    let i2c_config = esp_idf_hal::i2c::config::Config::new()
      .baudrate(KiloHertz(100).into())
      .scl_enable_pullup(true)
      .sda_enable_pullup(true);

    let i2c_driver = esp_idf_hal::i2c::I2cDriver::new(peripherals.i2c0, sda, scl, &i2c_config).unwrap();

    let mut i2c_esp32_driver = I2cEsp32Driver { 
      i2c: i2c_driver,
    };

    let mut delay_esp32_driver = DelayEsp32Driver {};
 
    let mut lcd_screen = Lcd::new(&mut i2c_esp32_driver, 0x27, &mut delay_esp32_driver).unwrap();

    lcd_screen.clear().unwrap();
    lcd_screen.set_display(liquidcrystal_i2c_rs::Display::On).unwrap();
    lcd_screen.set_backlight(liquidcrystal_i2c_rs::Backlight::On).unwrap();
    lcd_screen.print("Hello 11").unwrap();

    loop {
      thread::sleep(Duration::from_millis(1000));
  
      println!("It's works!");
    }
}
