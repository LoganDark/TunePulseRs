#![feature(impl_trait_in_assoc_type)] // used in `embassy_executor::main`

#![no_std]
#![no_main]

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
	let device_config = embassy_stm32::Config::default();
	let _context = embassy_stm32::init(device_config);
	info!("Hello, world!");
}
