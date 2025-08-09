// use esp_idf_svc::hal::{gpio::{Level, Pull, OutputOpenDrain}, prelude::Peripherals, delay::Delay, clock::ClockControl};
use esp_idf_hal::{gpio::*};
use esp_idf_hal::task::*;
use esp_idf_hal::peripherals::Peripherals;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main()  -> anyhow::Result<()> {
  // It is necessary to call this function once. Otherwise some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();
  
  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();

  log::info!("starting, attach debugger if needed");
  sleep(Duration::from_secs(6));
  log::info!("started");

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
  sleep(Duration::from_secs(1));
  
  dht_start(&mut sensor);
  dht_get(&mut sensor);
  
}

fn dht_connect<T: Pin> (sensor: &mut PinDriver<'_, T, InputOutput>){
  log::info!("Starting communication");

  PinDriver::set_high(sensor).unwrap();
  sleep(Duration::from_millis(100));

  PinDriver::set_low(sensor).unwrap();

  sleep(Duration::from_millis(30));

  PinDriver::set_high(sensor).unwrap();
}

fn dht_get_level_until_timeout<T: Pin>(sensor: &mut PinDriver<'_, T, InputOutput>, level_meter: Level) -> Result<(), ()>{
  let start = Instant::now();
  
  loop{
    if PinDriver::get_level(sensor) == level_meter {
      return Ok(());
    } 
    
    if start.elapsed() >= Duration::from_secs(1){
      return Err(())
    }

    sleep(Duration::from_micros(3))
  }
}

fn dht_start<T: Pin> (sensor: &mut PinDriver<'_, T, InputOutput>){
  loop{
    dht_connect(sensor);
    
    match dht_get_level_until_timeout(sensor, Level::Low){
      Ok(_) => {
        match dht_get_level_until_timeout(sensor, Level::High){
          Ok(_) => {    
            //Putting that in a thread because it's slows the process and break the communication with the sensor
            thread::spawn(||{log::info!("Sensor will send data soon\n<")});
            
            break;
          },
          Err(_) => {}
        }
      },
      Err(_) => {}
    };
    
    log::info!("Sensor hasn't aknowledge the communication, retrying...\n<");
  }
}

fn dht_get<T: Pin> (sensor: &mut PinDriver<'_, T, InputOutput>){
  loop{
    // dht_get_level_until_timeout(sensor, Level::Low);
    // let start = Instant::now(); //TODO: This isn't sync exactly because of sleep of a few micro in dht_get_level_until_timeout
    
    // dht_get_level_until_timeout(sensor, Level::High);

    // let stop = start.elapsed();
    // log::info!("elapsed: {stop:?}");
  }
}
