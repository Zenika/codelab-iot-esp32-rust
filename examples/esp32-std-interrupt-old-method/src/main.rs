// Reference: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/system/freertos.html
// This code use old method with link_section

use anyhow::Result;
use esp_idf_sys::{
    esp, gpio_config, gpio_config_t, gpio_install_isr_service, gpio_int_type_t_GPIO_INTR_POSEDGE,
    gpio_isr_handler_add, gpio_mode_t_GPIO_MODE_INPUT, xQueueGenericCreate, xQueueGiveFromISR,
    xQueueReceive, QueueHandle_t, ESP_INTR_FLAG_IRAM,
};

use std::ptr;

// For led control
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;

type LedPin = PinDriver<'static,  esp_idf_hal::gpio::Gpio2, esp_idf_hal::gpio::InputOutput>;

// This `static mut` holds the queue handle we are going to get from `xQueueGenericCreate`.
// This is unsafe, but we are careful not to enable our GPIO interrupt handler until after this value has been initialised, and then never modify it again
static mut EVENT_QUEUE: Option<QueueHandle_t> = None;

// Rust don't allow uninit global variable. We must use Option.
static mut LED_PIN: Option<LedPin> = None;

// ESP32 has only. Use "iram0.text" for ESP32-C3
#[link_section = ".iram.text"]
unsafe extern "C" fn button_interrupt(_: *mut core::ffi::c_void) {
    xQueueGiveFromISR(EVENT_QUEUE.unwrap(), std::ptr::null_mut());

    // Turn on/off the led!
    let led_pin = LED_PIN.as_mut().unwrap();
    match led_pin.get_level() {
        esp_idf_hal::gpio::Level::Low => led_pin.set_high().unwrap(),
        esp_idf_hal::gpio::Level::High => led_pin.set_low().unwrap()
    };
}

fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    
    const GPIO_NUM: i32 = 4;

    // Configures the button
    let io_conf = gpio_config_t {
        pin_bit_mask: 1 << GPIO_NUM,
        mode: gpio_mode_t_GPIO_MODE_INPUT,
        pull_up_en: true.into(),
        pull_down_en: false.into(),
        intr_type: gpio_int_type_t_GPIO_INTR_POSEDGE, // Positive edge trigger = button down
    };

    // Queue configurations
    const QUEUE_TYPE_BASE: u8 = 0;
    const ITEM_SIZE: u32 = 0; // We're not posting any actual data, just notifying
    const QUEUE_SIZE: u32 = 1;

    unsafe {
        // Writes the button configuration to the registers
        esp!(gpio_config(&io_conf))?;

        // Installs the generic GPIO interrupt handler
        esp!(gpio_install_isr_service(ESP_INTR_FLAG_IRAM as i32))?;

        // Instantiates the event queue
        EVENT_QUEUE = Some(xQueueGenericCreate(QUEUE_SIZE, ITEM_SIZE, QUEUE_TYPE_BASE));

        // Registers our function with the generic GPIO interrupt handler we installed earlier.
        esp!(gpio_isr_handler_add(
            GPIO_NUM,
            Some(button_interrupt),
            std::ptr::null_mut()
        ))?;
    }

    let peripherals = Peripherals::take().unwrap();

    unsafe {
        // Rust don't allow static mutable global variable. We must do this in unsafe section.
        LED_PIN = Some(PinDriver::input_output(peripherals.pins.gpio2).unwrap());
        LED_PIN.as_mut().unwrap().set_low().unwrap();
    }

    // Reads the queue in a loop.
    loop {
        unsafe {
            // Maximum delay
            const QUEUE_WAIT_TICKS: u32 = 1000;

            // Reads the event item out of the queue
            let res = xQueueReceive(EVENT_QUEUE.unwrap(), ptr::null_mut(), QUEUE_WAIT_TICKS);

            // If the event has the value 0, nothing happens. if it has a different value, the button was pressed.
            match res {
                1 => println!("Button pressed!"),
                _ => {}
            };
        }
    }
}
