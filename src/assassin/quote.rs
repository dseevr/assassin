use assassin::tick::Tick;

extern crate chrono;

use self::chrono::prelude::*;

#[derive(Clone)]
pub struct Quote {
	name: String,
	symbol: String,
	bid: f64,
	ask: f64,
	expiration_date: DateTime<FixedOffset>,
	// TODO: depth, etc. if available
}

impl Quote {
	pub fn new(tick: &Tick) -> Quote {
		if tick.bid() > tick.ask() {
			panic!("got bid {} > ask {}", tick.bid(), tick.ask());
		}

		Quote{
			name: tick.name(),
			symbol: tick.symbol(),
			bid: tick.bid(),
			ask: tick.ask(),
			expiration_date: tick.expiration_date(),
		}
	}

	pub fn name(&self) -> String {
		self.name.clone()
	}

	pub fn symbol(&self) -> String {
		self.symbol.clone()
	}

	pub fn bid(&self) -> f64 {
		self.bid
	}

	pub fn ask(&self) -> f64 {
		self.ask
	}

	pub fn expiration_date(&self) -> DateTime<FixedOffset> {
		self.expiration_date
	}
}