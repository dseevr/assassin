use std::collections::HashMap;

use assassin::order::Order;
use assassin::position::Position;
use assassin::quote::Quote;
use assassin::tick::Tick;
use assassin::traits::*;

pub struct BasicBroker {
	balance: f64,
	open_positions: HashMap<String, Position>,
	orders: Vec<Order>,
	commission_schedule: Box<Commission>,
	commission_paid: f64,
	data_feed: Box<DataFeed>,
	quotes: HashMap<String, Quote>,
	current_date: String,
}

impl BasicBroker {
	pub fn new(initial_balance: f64,
			commission_schedule: Box<Commission>,
			data_feed: Box<DataFeed>,
		) -> BasicBroker {

		if initial_balance <= 0.0 {
			panic!("balance must be > 0.0 (got {})", initial_balance);
		}

		BasicBroker{
			balance: initial_balance,
			open_positions: HashMap::new(),
			orders: vec![],
			commission_schedule: commission_schedule,
			commission_paid: 0.0,
			data_feed: data_feed,
			quotes: HashMap::new(),
			current_date: "".to_string(),
		}
	}
}

impl Broker for BasicBroker {
	fn account_balance(&self) -> f64 {
		self.balance
	}

	// TODO: positions have a correct cost basis

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

	fn close_all_positions(&mut self) {
		// TODO: close all open positions at last price
	}

	fn next_tick(&mut self) -> Option<Tick> {
		if let Some(tick) = self.data_feed.next_tick() {
			self.quotes.insert(tick.name(), tick.quote());

			let date = tick.formatted_date();

			// if day has changed, remove any expired quotes
			if date != self.current_date {
				let mut new_quotes = self.quotes.clone();
				let mut removed_entry_count = 0;

				for (key, quote) in &self.quotes {
					if quote.expiration_date() == self.current_date {
						new_quotes.remove(key);
						removed_entry_count += 1;
					}
				}

				println!(
					"purged {} expired entries for {}",
					removed_entry_count,
					self.current_date,
				);

				self.quotes = new_quotes;
				self.current_date = date;
			}
			Some(tick)
		} else {
			None
		}
	}
}