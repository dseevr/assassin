use std::collections::HashMap;

use assassin::order::Order;
use assassin::position::Position;
use assassin::traits::*;

pub struct BasicBroker {
	balance: f64,
	open_positions: HashMap<String, Position>,
	orders: Vec<Order>,
	commission_schedule: Box<Commission>,
	commission_paid: f64,
}

impl BasicBroker {
	pub fn new(initial_balance: f64, commission_schedule: Box<Commission>) -> BasicBroker {
		if initial_balance <= 0.0 {
			panic!("balance must be > 0.0 (got {})", initial_balance);
		}

		BasicBroker{
			balance: initial_balance,
			open_positions: HashMap::new(),
			orders: vec![],
			commission_schedule: commission_schedule,
			commission_paid: 0.0,
		}
	}
}

impl Broker for BasicBroker {
	fn account_balance(&self) -> f64 {
		self.balance
	}

	fn process_order(&mut self, order: Order) -> bool {
		self.orders.push(order.clone());

		// ensure enough cash available
		if order.cost_basis() > self.balance {
			println!(
				"not enough money (need {}, have {})",
				order.cost_basis(),
				self.balance
			);
			return false;
		}

		self.open_positions.entry(order.symbol()).or_insert(Position::new(&order)).apply_order(&order);

		self.balance += order.canonical_cost_basis();

		// apply commission to balance and running total of paid commission
		let commish = self.commission_schedule.commission_for(order);

		self.balance -= commish;
		self.commission_paid += commish;

		true
	}

	fn open_positions(&self) -> Vec<Position> {
		let mut positions: Vec<Position> = vec![];

		for (_, value) in &self.open_positions {
			if value.quantity() != 0 {
				positions.push(value.clone());
			}
		}

		positions
	}

	fn total_order_count(&self) -> i32 {
		self.orders.len() as i32
	}

	fn commission_paid(&self) -> f64 {
		self.commission_paid
	}
}