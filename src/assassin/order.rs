use assassin::tick::Tick;

extern crate chrono;

use self::chrono::prelude::*;

#[derive(Clone)]
pub struct Order {
	tick: Box<Tick>,
	buy: bool,
	open: bool,
	quantity: i32,
	limit: f64,
}

impl Order {
	pub fn option_name(&self) -> String {
		self.tick.name()
	}

	pub fn expiration_date(&self) -> DateTime<FixedOffset> {
		self.tick.expiration_date()
	}

	pub fn new_buy_open_order(tick: Box<Tick>, quantity: i32, limit: f64) -> Order {
		if quantity <= 0 {
			panic!("quantity must be > 0 (got {})", quantity);
		}

		if limit <= 0.0 {
			panic!("limit must be > 0.0 (got {})", limit);
		}

		Order{
			tick: tick,
			buy: true,
			open: true,
			quantity: quantity,
			limit: limit,
		}
	}

	pub fn new_sell_open_order(tick: Box<Tick>, quantity: i32, limit: f64) -> Order {
		let mut o = Order::new_buy_open_order(tick, quantity, limit);
		o.buy = false;

		o
	}

	pub fn new_buy_close_order(tick: Box<Tick>, quantity: i32, limit: f64) -> Order {
		let mut o = Order::new_buy_open_order(tick, quantity, limit);
		o.open = false;

		o
	}

	pub fn new_sell_close_order(tick: Box<Tick>, quantity: i32, limit: f64) -> Order {
		let mut o = Order::new_buy_open_order(tick, quantity, limit);
		o.buy = false;
		o.open = false;

		o
	}

	pub fn buy_to_open(&self) -> bool {
		self.buy && self.open
	}

	pub fn sell_to_open(&self) -> bool {
		! self.buy && self.open
	}

	pub fn buy_to_close(&self) -> bool {
		self.buy && ! self.open
	}

	pub fn sell_to_close(&self) -> bool {
		! self.buy && ! self.open
	}

	pub fn cost_basis(&self) -> f64 {
		self.quantity as f64 * self.limit * 100.0 // TODO: assumes 100...
	}

	pub fn symbol(&self) -> String {
		self.tick.symbol().clone()
	}

	pub fn quantity(&self) -> i32 {
		self.quantity
	}

	pub fn limit(&self) -> f64 {
		self.limit
	}

	pub fn canonical_quantity(&self) -> i32 {
		if self.buy {
			self.quantity
		} else {
			- self.quantity
		}
	}

	pub fn canonical_cost_basis(&self) -> f64 {
		if self.buy {
			// debit
			- self.cost_basis()
		} else {
			// credit
			self.cost_basis()
		}
	}
}
