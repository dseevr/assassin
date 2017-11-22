use assassin::order::Order;

extern crate chrono;

use self::chrono::prelude::*;

#[derive(Clone)]
pub struct Position {
	name: String,
	symbol: String,
	quantity: i32,
	expiration_date: DateTime<FixedOffset>,
	orders: Vec<Order>,
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
			// don't set the order here because it gets applied in
			// apply_order() below.
			orders: vec![],
		}
	}

	pub fn symbol(&self) -> String {
		self.symbol.clone()
	}

	pub fn name(&self) -> String {
		self.name.clone()
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

	pub fn expiration_date(&self) -> DateTime<FixedOffset> {
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

	pub fn is_expired(&self, current_date: DateTime<FixedOffset>) -> bool {
		// < instead of <= because we update the current date in the broker
		// _before_ calling this function.  if we used <=, it would close
		// positions which expire on the current trading day before the
		// model's logic has run.
		self.expiration_date.num_days_from_ce() < current_date.num_days_from_ce()
	}
}
