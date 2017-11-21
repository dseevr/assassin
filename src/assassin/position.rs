use assassin::order::Order;

extern crate chrono;

use self::chrono::prelude::*;

#[derive(Clone)]
pub struct Position {
	name: String,
	symbol: String,
	quantity: i32,
	expiration_date: DateTime<FixedOffset>,
}

impl Position {
	// NOTE: apply_order() still needs to be called afterwards.
	//       order is only used to set the name/symbol/expiration date
	pub fn new(order: &Order) -> Position {
		Position{
			name: order.option_name(),
			symbol: order.symbol(),
			quantity: 0,
			expiration_date: order.expiration_date(),
		}
	}

	pub fn name(&self) -> String {
		self.name.clone()
	}

	pub fn apply_order(&mut self, order: &Order) {
		self.quantity += order.canonical_quantity()
	}

	pub fn quantity(&self) -> i32 {
		self.quantity
	}

	pub fn expiration_date(&self) -> DateTime<FixedOffset> {
		self.expiration_date
	}

	pub fn is_open(&self) -> bool {
		self.quantity != 0 // can be negative if short
	}

	// TODO: add expires_on() and use in Broker.process_order()

	pub fn is_expired(&self, current_date: DateTime<FixedOffset>) -> bool {
		// < instead of <= because we update the current date in the broker
		// _before_ calling this function.  if we used <=, it would close
		// positions which expire on the current trading day before the
		// model's logic has run.
		self.expiration_date.num_days_from_ce() < current_date.num_days_from_ce()
	}
}
