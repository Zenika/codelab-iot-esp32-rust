use esp_idf_hal::units::KiloHertz;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::thread;
use std::time::Duration;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::i2c::I2cDriver;

// LCD
use esp_idf_hal::i2c;

mod lcd;

// implement HAL...
struct HW<'d> {
  i2c: I2cDriver<'d>,
  address: u8,
}

// Implement the `Hardware` trait to give access to LCD pins
impl lcd::I2C for HW<'_> {
  fn write(&mut self, data: u8) -> Result<usize, lcd::I2cError> {
    if let Ok(()) = self.i2c.write(self.address, &[data], 1000) {
      return Ok(1);
    }

    return Err(lcd::I2cError::Io);
  }
}

// Implement the `Delay` trait to allow library to sleep for the given amount of time
impl lcd::Delay for HW<'_> {
  fn delay_us(&mut self, delay_usec: u32) {
    thread::sleep(Duration::from_micros(delay_usec.into()));
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

    let i2c_config = i2c::config::Config::new()
      .baudrate(KiloHertz(100).into())
      .scl_enable_pullup(true)
      .sda_enable_pullup(true);

    let i2c_driver = i2c::I2cDriver::new(peripherals.i2c0, sda, scl, &i2c_config).unwrap();

    let hw = HW { 
      i2c: i2c_driver,
      address: 0x27,
     };
 
    let mut lcd_screen = lcd::Lcd::new(hw).unwrap();

    lcd_screen.clear().unwrap();
    lcd_screen.set_display(lcd::Display::On).unwrap();
    lcd_screen.set_backlight(lcd::Backlight::On).unwrap();
    lcd_screen.print("Hello 11").unwrap();

    loop {
      thread::sleep(Duration::from_millis(1000));
  
      println!("It's works!");
    }
}
