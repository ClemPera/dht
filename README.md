# DHT
<!-- [![Continuous Integration](https://github.com/ClemPera/dht/actions/workflows/rust_ci.yml/badge.svg)](https://github.com/ClemPera/dht/actions/workflows/rust_ci.yml) -->
[<img alt="github" src="https://img.shields.io/badge/github-ClemPera/esp--idf--dht-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/ClemPera/esp-idf-dht)
[<img alt="crates.io" src="https://img.shields.io/crates/v/esp-idf-dht.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/esp-idf-dht)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/ClemPera/esp-idf-dht/rust_ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/ClemPera/esp-idf-dht/actions?query=branch%main)

It's a crate that handle the reading of data of a DHT11/22 (temperature/humidity sensor) using `esp_idf_hal` on a esp32

## How to use

```toml
[dependencies]
esp-idf-dht = "^0.1"
```


Here's a sample code:

```rust
use esp_idf_hal::{gpio::*};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::task::*;
use std::thread::sleep;
use std::time::Duration;
use esp_idf_dht;

fn main()  -> anyhow::Result<()> {
  // It is necessary to call this function once. Otherwise some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();
  
  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();
  
  log::info!("starting, attach debugger if needed");
  sleep(Duration::from_secs(4));
  log::info!("started");
  
  let peripherals: Peripherals = Peripherals::take().unwrap();
  let pins = peripherals.pins;
  let mut sensor = PinDriver::input_output_od(pins.gpio21).unwrap();
  sleep(Duration::from_secs(1));
  
  block_on(async {loop{
    let vals = esp_idf_dht::read(&mut sensor).unwrap();

    log::info!("values are {vals:?}");

    sleep(Duration::from_secs(5));
  }})
}
```

## How to debug?
- Run this via terminal to build+flash `cargo build && espflash flash --monitor target/xtensa-esp32s3-espidf/debug/first-esp`

- Use `Attach to ESP32-S3` run the debugger the moment the flash finish

## How to run unit test
- Run unit tests with `rustc --test src/utils.rs -o utils_test && ./utils_test`
