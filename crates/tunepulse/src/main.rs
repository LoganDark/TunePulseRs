#![feature(impl_trait_in_assoc_type)] // used in `embassy_executor::main`

#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use led::Led;

mod led;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
	let config = embassy_stm32::Config::default();
	let context = embassy_stm32::init(config);

	let mut led = Led::new(context.PB13, context.PB14, context.PB15);

	info!("Hello, world!");

	loop {
		info!("LED red");
		led.set_red();
		Delay.delay_ms(500).await;

		info!("LED green");
		led.set_green();
		Delay.delay_ms(500).await;

		info!("LED blue");
		led.set_blue();
		Delay.delay_ms(500).await;

		info!("LED off");
		led.set_off();
		Delay.delay_ms(500).await;
	}
}
