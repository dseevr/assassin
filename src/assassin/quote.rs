use assassin::money::Money;
use assassin::tick::Tick;

extern crate chrono;

use self::chrono::prelude::*;

#[derive(Clone)]
pub struct Quote {
	name: String,
	symbol: String,
	bid: Money,
	ask: Money,
	strike_price: Money,
	expiration_date: DateTime<Utc>,
	// TODO: depth, etc. if available
}

impl Quote {
	pub fn new(tick: &Tick) -> Quote {
		if tick.bid() > tick.ask() {
			panic!("got bid {} > ask {}", tick.bid(), tick.ask());
		}

		Quote{
			name: tick.name(),
			symbol: tick.symbol().to_string(),
			bid: tick.bid(),
			ask: tick.ask(),
			strike_price: tick.strike_price(),
			expiration_date: tick.expiration_date(),
		}
	}

	pub fn midpoint_price(&self) -> Money {
		(self.ask + self.bid) / 2
	}

	pub fn strike_price(&self) -> Money {
		self.strike_price
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn symbol(&self) -> &str {
		&self.symbol
	}

	pub fn bid(&self) -> Money {
		self.bid
	}

	pub fn ask(&self) -> Money {
		self.ask
	}

	pub fn expiration_date(&self) -> DateTime<Utc> {
		self.expiration_date
	}
}