use esp_idf_hal::peripheral::Peripheral;
// use esp_idf_svc::hal::{gpio::{Level, Pull, OutputOpenDrain}, prelude::Peripherals, delay::Delay, clock::ClockControl};
use esp_idf_hal::{gpio::*};
use esp_idf_hal::{io, task::*};
use esp_idf_hal::peripherals::Peripherals;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

fn main()  -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    block_on(async {loop{
        log::info!("Hello, world!");
        dht().await;
        sleep(Duration::from_secs(1000));
    }})
}

async fn dht()
{
    let peripherals: Peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let mut sensor = PinDriver::input_output_od(pins.gpio21).unwrap();
    sleep(Duration::from_secs(2));
    log::info!("starting");
    
    PinDriver::set_high(&mut sensor).unwrap();
    sleep(Duration::from_millis(100));
    let level = PinDriver::get_level(&mut sensor);
    log::info!("0:{level:?}");
    //start communication 
    PinDriver::set_low(&mut sensor).unwrap();
    let level = PinDriver::get_level(&mut sensor);
    log::info!("1:{level:?}");
    sleep(Duration::from_millis(30));
    PinDriver::set_high(&mut sensor).unwrap();
    let level = PinDriver::get_level(&mut sensor);
    log::info!("2:{level:?}");
    
    //wait for the sensor to answer
    PinDriver::wait_for_low(&mut sensor).await.unwrap();
    // let level = PinDriver::get_level(&mut sensor);
    // log::info!("3:{level:?}");
    PinDriver::wait_for_high(&mut sensor).await.unwrap();
    // let level = PinDriver::get_level(&mut sensor);
    // log::info!("4:{level:?}");
    
    // log::info!("Communication established");
    PinDriver::wait_for_low(&mut sensor).await.unwrap();
    // let level = PinDriver::get_level(&mut sensor);
    // log::info!("5:{level:?}");
    let start = Instant::now();
    
    PinDriver::wait_for_high(&mut sensor).await.unwrap();
    // let level = PinDriver::get_level(&mut sensor);
    // log::info!("6:{level:?}");
    let stop = start.elapsed();
    log::info!("elapsed: {stop:?}");
}