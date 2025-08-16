// use esp_idf_svc::hal::{gpio::{Level, Pull, OutputOpenDrain}, prelude::Peripherals, delay::Delay, clock::ClockControl};
use esp_idf_hal::{gpio::*};
use esp_idf_hal::task::*;
use esp_idf_hal::peripherals::Peripherals;
use std::thread::sleep;
use std::time::{Duration, Instant};
use first_esp::*;

fn main()  -> anyhow::Result<()> {
  // It is necessary to call this function once. Otherwise some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();
  
  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();
  
  log::info!("starting, attach debugger if needed");
  sleep(Duration::from_secs(4));
  log::info!("started");
  
  block_on(async {loop{
    log::info!("Hello, world!");
    dht().await;
    sleep(Duration::from_secs(5));
  }})
}

async fn dht()
{
  let peripherals: Peripherals = Peripherals::take().unwrap();
  let pins = peripherals.pins;
  let mut sensor = PinDriver::input_output_od(pins.gpio21).unwrap();
  sleep(Duration::from_secs(1));
  
  loop{
    dht_start(&mut sensor);
    // dht_get(&mut sensor);
  }
}

fn dht_connect<T: Pin> (sensor: &mut PinDriver<'_, T, InputOutput>){
  log::info!("Starting communication");
  
  PinDriver::set_high(sensor).unwrap();
  sleep(Duration::from_millis(100));
  
  PinDriver::set_low(sensor).unwrap();
  
  sleep(Duration::from_millis(30));
  
  PinDriver::set_high(sensor).unwrap();
}

//TODO: On pourrait peut être calculer et renvoyer le temps de lecture là dedans
fn dht_get_level_until_timeout<T: Pin>(sensor: &mut PinDriver<'_, T, InputOutput>, level_meter: Level) -> Result<(), ()>{
  let start = Instant::now();
  
  loop{
    if PinDriver::get_level(sensor) == level_meter {
      return Ok(());
    } 
    
    if start.elapsed() >= Duration::from_secs(1){
      return Err(())
    }
    
    //TODO: See if we can't put a sleep here (keep in mind that it takes the time between level to show data so maybe not? (nanosec could be fine))
    // sleep(Duration::from_micros(3))
  }
}

fn dht_start<T: Pin> (sensor: &mut PinDriver<'_, T, InputOutput>){
  loop{
    dht_connect(sensor);
    
    match dht_get_level_until_timeout(sensor, Level::Low){
      Ok(_) => {
        match dht_get_level_until_timeout(sensor, Level::High){
          Ok(_) => {    
            match dht_get_level_until_timeout(sensor, Level::Low){
              Ok(_) => {
                dht_get(sensor);
              },
              Err(_) => {}
            }
          },
          Err(_) => {}
        }
      },
      Err(_) => {}
    };
    
    // log::info!("Sensor hasn't aknowledge the communication, retrying...\n<");

    sleep(Duration::from_secs(5));
  }
}

fn dht_get<T: Pin> (sensor: &mut PinDriver<'_, T, InputOutput>){
  let mut bit: u8 = 0;
  let mut bits: Vec<u8> = Vec::new();

  loop{
    //Wait for timeout between bits is finshed
    match dht_get_level_until_timeout(sensor, Level::High){
      Ok(_) => {}
      Err(_) => {
        log::error!("Timeout between bits for bit n°{bit:?} has been too long");
        break;
      }
    };
    
    //Start reading bit
    let start = Instant::now(); 
    match dht_get_level_until_timeout(sensor, Level::Low){
      Ok(_) => {
        let stop = start.elapsed().as_micros();
        if stop <= 37{
          bits.push(0);
        }
        else {
          bits.push(1);
        }
      }
      Err(_) => {
        // log::error!("Timeout for reading bit n°{bit:?} has been too long");
        
        break;
      }
    };
    
    bit = bit+1;
  }

  let _ = dht_check(bits);
  // dht_human();
  
  //This logs cause issues for some reason, please don't use
  // log::info!("bits are {bits:?}");
}

//TODO: unit test (+move to lib.rs)
fn dht_check(bits: Vec<u8>) -> Result<(),()>{
  let len =  bits.len();
  if len == 40 {
    // Convert bits to bytes

    bit_to_bytes(bits);

    //TODO: Checksum
    
    return Ok(())
  }else{
    return Err(())
  }
}