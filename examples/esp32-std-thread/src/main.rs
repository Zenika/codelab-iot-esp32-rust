use esp_idf_hal::{task::thread::ThreadSpawnConfiguration, cpu::Core};
use esp_idf_sys::{self as _, configMAX_PRIORITIES}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::{thread, time::Duration};

// see https://dev.to/apollolabsbin/esp-embedded-rust-multithreading-with-freertos-bindings-251g

fn main() {
  // It is necessary to call this function once. Otherwise some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_sys::link_patches();
  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();

  let mut children = vec![];

  println!("Rust main thread: {:?}", thread::current());

  ThreadSpawnConfiguration {
    name: Some("Thread-Classic\0".as_bytes()),
    priority: 1,
    ..Default::default()
  }
  .set()
  .unwrap();

  for i in 0..5 {
    // Spin up another thread
    children.push(thread::spawn(move || {
        println!("This is thread number {}, {:?}", i, thread::current());
    }));
  }

  ThreadSpawnConfiguration {
    name: Some("Thread-A\0".as_bytes()),
    stack_size: 10000,
    priority: (configMAX_PRIORITIES - 1) as u8, // Max priority
    pin_to_core: Some(Core::Core0),
    ..Default::default()
  }
  .set()
  .unwrap();

  let a = std::thread::Builder::new()
    .spawn(move || {
      println!("This is ThreadSpawnConfiguration {:?}", thread::current());  
    }).unwrap();

  let _ = a.join();

  println!(
    "About to join the threads. If ESP-IDF was patched successfully, joining will NOT crash"
  );

  for child in children {
      // Wait for the thread to finish. Returns a result.
      let _ = child.join();
  }

  thread::sleep(Duration::from_secs(2));

  println!("Joins were successful.");

  loop {
    thread::sleep(Duration::from_millis(1000));
    println!("It's works!");
  }
}
