use esp_idf_hal::{gpio::*};
use esp_idf_hal::task::*;
use esp_idf_hal::peripherals::Peripherals;
use std::thread::sleep;
use std::time::{Duration, Instant};
use first_esp::*;

static NUMBER_OF_TRY_BEFORE_ERROR: u8 = 10;

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
    let _ = dht_start(&mut sensor);
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

fn dht_get_level_until_timeout<T: Pin>(sensor: &mut PinDriver<'_, T, InputOutput>, level_meter: Level) 
    -> Result<Duration, &'static str>{
  const TIMEOUT: Duration = Duration::from_secs(1);
  let start = Instant::now();
  
  loop{
    if PinDriver::get_level(sensor) == level_meter {
      return Ok(start.elapsed());
    } 
    
    if start.elapsed() >= TIMEOUT{
      return Err("Timeout has been exceeded: {TIMEOUT::MILLISECOND}ms");
    }
  }
}

fn dht_start<T: Pin> (sensor: &mut PinDriver<'_, T, InputOutput>) -> Result<[f32; 2], &'static str>{
  let mut tries: u8 = 0;

  loop{
    tries = tries + 1 ;
    dht_connect(sensor);
    
    if dht_get_level_until_timeout(sensor, Level::Low).is_ok() {
      if dht_get_level_until_timeout(sensor, Level::High).is_ok() {
        if dht_get_level_until_timeout(sensor, Level::Low).is_ok(){
          match dht_get(sensor){
            Ok(vals) => {
              log::info!("vals read correctly: {vals:?}");
              return Ok(vals)
            }
            Err(_) => {}
          }
        }
      }
    }
    if tries >= NUMBER_OF_TRY_BEFORE_ERROR {
      log::info!("It tried to read {tries} times but the reading didn't work");
      return Err("It tried to read {tries} times but the reading didn't work");
    }
    
    sleep(Duration::from_secs(5));
  }
}

fn dht_get<T: Pin> (sensor: &mut PinDriver<'_, T, InputOutput>) -> Result<[f32; 2], &'static str>{
  let mut bit: u8 = 0;
  let mut bits: Vec<u8> = Vec::new();

  loop{
    //Wait for timeout between bits is finshed
    if dht_get_level_until_timeout(sensor, Level::High).is_err() {
      log::info!("Timeout between bits for bit n°{bit:?} has been too long, retrying...");
      break;
    }
    
    //Start reading bit
    match dht_get_level_until_timeout(sensor, Level::Low){
      Ok(elapsed) => {
        if elapsed.as_micros() <= 37{
          bits.push(0);
        } else {
          bits.push(1);
        }
      }
      Err(_) => {
        break;
      }
    };
    
    bit = bit+1;
  }

  match dht_check(bits){
    Ok(bytes) => { Ok(convert_to_decimal(bytes)) }
    Err(error) => { return Err(error) }
  }
}

fn dht_check(bits: Vec<u8>) -> Result<[u8; 5], &'static str>{
  if bits.len() != 40 {
    return Err("There's not 40 bits")
  }

  let bytes = bits_to_bytes(bits.clone());

  if checksum(bytes).is_err(){
    log::info!("checksum didn't pass :( {bytes:?}. here's bits: {bits:?}");
    return Err("checksum didn't pass :(")
  }

  Ok(bytes)
}