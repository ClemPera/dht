#[cfg(target_arch = "xtensa")] 
pub mod dht;

mod utils;

#[cfg(target_arch = "xtensa")] 
pub use dht::read;