## How to debug?

- Run this via terminal to build+flash `cargo build && espflash flash --monitor target/xtensa-esp32s3-espidf/debug/first-esp`

- Use `Attach to ESP32-S3` run the debugger the moment the flash finish

## How to run unit test
- Run unit tests with `cargo test --lib --target x86_64-unknown-linux-gnu`