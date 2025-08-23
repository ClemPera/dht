# DHT
It's (going to be) a crate that handle the reading of data of a DHT11/22 (temperature/humidity sensor) using `esp_idf_hal` on a esp32

## How to debug?
- Run this via terminal to build+flash `cargo build && espflash flash --monitor target/xtensa-esp32s3-espidf/debug/first-esp`

- Use `Attach to ESP32-S3` run the debugger the moment the flash finish

## How to run unit test
- Run unit tests with `cargo test --lib --no-default-features --target x86_64-unknown-linux-gnu`