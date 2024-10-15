use embassy_stm32::{gpio::{Level, Output, Pin, Speed}, Peripheral};

pub struct Led<'a> {
	red: Output<'a>,
	green: Output<'a>,
	blue: Output<'a>
}

impl<'a> Led<'a> {
	pub fn new(red: impl Peripheral<P = impl Pin> + 'a, green: impl Peripheral<P = impl Pin> + 'a, blue: impl Peripheral<P = impl Pin> + 'a) -> Self {
		Self {
			red: Output::new(red, Level::High, Speed::Low),
			green: Output::new(green, Level::High, Speed::Low),
			blue: Output::new(blue, Level::High, Speed::Low)
		}
	}

	pub fn set_red(&mut self) {
		self.red.set_low();
		self.green.set_high();
		self.blue.set_high();
	}

	pub fn set_green(&mut self) {
		self.red.set_high();
		self.green.set_low();
		self.blue.set_high();
	}

	pub fn set_blue(&mut self) {
		self.red.set_high();
		self.green.set_high();
		self.blue.set_low();
	}

	pub fn set_off(&mut self) {
		self.red.set_high();
		self.green.set_high();
		self.blue.set_high();
	}
}
