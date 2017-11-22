use assassin::quote::Quote;

extern crate chrono;

use self::chrono::prelude::*;

#[derive(Clone)]
pub struct Order {
	quote: Box<Quote>,
	buy: bool,
	open: bool,
	quantity: i32,
	limit: f64,
	strike_price: f64,
	// date: DateTime<FixedOffset>, // TODO: flesh this out
	filled: bool,
	fill_price: f64, // average price
	commission: f64,
}

impl Order {
	pub fn commission(&self) -> f64 {
		self.commission
	}

	pub fn filled_at(&mut self, price: f64, commish: f64) {
		self.filled = true;
		self.fill_price = price;
		self.commission = commish
	}

	pub fn fill_price(&self) -> f64 {
		self.fill_price
	}

	pub fn buy_or_sell_string(&self) -> String {
		if self.buy { "BUY" } else { "SELL" }.to_string()
	}

	// "AAPL: BUY 10 CALL $150 STRIKE at LIMIT $2.50"
	pub fn summary(&self) -> String {
		format!(
			"{} {} {} ${:.2} STRIKE at LIMIT ${:.2}",
			self.symbol(),
			self.buy_or_sell_string(),
			self.quantity,
			self.strike_price,
			self.limit,
		)
	}

	pub fn option_name(&self) -> String {
		self.quote.name()
	}

	pub fn expiration_date(&self) -> DateTime<FixedOffset> {
		self.quote.expiration_date()
	}

	pub fn new_buy_open_order(quote: &Quote, quantity: i32, limit: f64) -> Order {
		if quantity <= 0 {
			panic!("quantity must be > 0 (got {})", quantity);
		}

		if limit < 0.0 {
			panic!("limit must be >= 0.0 (got {})", limit);
		}

		Order{
			quote: Box::new(quote.clone()),
			buy: true,
			open: true,
			quantity: quantity,
			limit: limit,
			strike_price: quote.strike_price(),
			filled: false,
			fill_price: 0.0,
			commission: 0.0,
		}
	}

	pub fn new_sell_open_order(quote: &Quote, quantity: i32, limit: f64) -> Order {
		let mut o = Order::new_buy_open_order(quote, quantity, limit);
		o.buy = false;

		o
	}

	pub fn new_buy_close_order(quote: &Quote, quantity: i32, limit: f64) -> Order {
		let mut o = Order::new_buy_open_order(quote, quantity, limit);
		o.open = false;

		o
	}

	pub fn new_sell_close_order(quote: &Quote, quantity: i32, limit: f64) -> Order {
		let mut o = Order::new_buy_open_order(quote, quantity, limit);
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
		self.quote.symbol().clone()
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
