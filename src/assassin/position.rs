use assassin::order::Order;

extern crate chrono;

use self::chrono::prelude::*;

#[derive(Clone)]
pub struct Position {
	name: String,
	symbol: String,
	quantity: i32,
	expiration_date: DateTime<Utc>,
	orders: Vec<Order>,
}

impl Position {
	// NOTE: apply_order() still needs to be called afterwards.
	//       order is only used to set the name/symbol/expiration date
	pub fn new(order: &Order) -> Position {
		Position{
			name: order.option_name().to_string(),
			symbol: order.symbol().to_string(),
			quantity: 0,
			expiration_date: order.expiration_date(),
			// don't set the order here because it gets applied in
			// apply_order() below.
			orders: vec![],
		}
	}

	// OPTIMIZE: this can be updated when orders are applied
	pub fn realized_profit(&self) -> f64 {
		self.orders.iter().fold(0.0, |sum, o|
			// NOTE: a buy order really changes the position's value
			//       by a negative amount because it's tying up capital
			//       in a debit. a sell order grants a credit and is thus
			//       a positive value.
			//
			//       canonical_quantity() returns the correct values
			//       (i.e., a buy is 10, a sell is -10) for quantity, but
			//       we want to invert this because we want a buy to be
			//       a debit and a sell to be a credit.
			sum + -(o.canonical_quantity() as f64 * 100.0 * o.fill_price())
		)
	}

	// OPTIMIZE: this can be updated when orders are applied
	pub fn commission_paid(&self) -> f64 {
		self.orders.iter().map(|o| o.commission()).sum()
	}

	pub fn symbol(&self) -> &str {
		&self.symbol
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn orders(&self) -> Vec<Order> {
		self.orders.clone()
	}

	pub fn apply_order(&mut self, order: &Order) {
		self.quantity += order.canonical_quantity();
		self.orders.push(order.clone());
	}

	pub fn quantity(&self) -> i32 {
		self.quantity
	}

	pub fn expiration_date(&self) -> DateTime<Utc> {
		self.expiration_date
	}

	pub fn is_long(&self) -> bool {
		self.quantity > 0
	}

	pub fn is_short(&self) -> bool {
		! self.is_long()
	}

	pub fn is_open(&self) -> bool {
		self.quantity != 0 // can be negative if short
	}

	pub fn is_closed(&self) -> bool {
		! self.is_open()
	}

	// TODO: add expires_on() and use in Broker.process_order()

	pub fn is_expired(&self, current_date: DateTime<Utc>) -> bool {
		// < instead of <= because we update the current date in the broker
		// _before_ calling this function.  if we used <=, it would close
		// positions which expire on the current trading day before the
		// model's logic has run.
		self.expiration_date.num_days_from_ce() < current_date.num_days_from_ce()
	}
}
